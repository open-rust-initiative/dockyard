use actix_web::http::header::{HeaderName, HeaderValue};
use actix_web::http::{header, StatusCode};

use actix_web::{get, head, Either, HttpRequest};
use actix_web::{web, HttpResponse, Responder};

use crate::dao::images::{
    select_digest_mediaType_by_name_tag, select_digest_mediaType_path_by_name_tag,
    update_pulltime_by_digest, update_pulltime_by_name_tag,
};
use crate::dao::layers::{
    select_mediaType_by_digest, select_path_by_digest, select_path_mediaType_by_digest,
};
use crate::{Calim, DatabasePool};
use actix_files::NamedFile;

use mime::Mime;

use std::str::FromStr;
#[allow(non_snake_case)]
#[head("/{name:.*}/manifests/{reference}")]
pub(crate) async fn head_manifests(
    _: Calim,
    params: web::Path<(String, String)>,
    pool: web::Data<DatabasePool>,
) -> impl Responder {
    let (name, reference) = params.into_inner();
    let connection = pool.get().unwrap();
    match select_digest_mediaType_by_name_tag(&name, &reference, &connection).await {
        Ok((digest, mediaType)) => {
            return HttpResponse::Ok()
                .insert_header((header::CONTENT_TYPE, mediaType))
                .insert_header(("Docker-Content-Digest", digest))
                .finish()
        }
        Err(_) => match select_mediaType_by_digest(&reference, &connection).await {
            Ok(media_type) => {
                return HttpResponse::Ok()
                    .insert_header((header::CONTENT_TYPE, media_type))
                    .insert_header(("Docker-Content-Digest", reference))
                    .finish()
            }
            Err(_) => {}
        },
    }
    return HttpResponse::NotFound().finish();
}
#[allow(non_snake_case, deprecated)]
#[get("/{name:.*}/manifests/{reference}")]
pub(crate) async fn get_manifests(
    _: Calim,
    params: web::Path<(String, String)>,
    pool: web::Data<DatabasePool>,
    req: HttpRequest,
) -> impl Responder {
    let (name, reference) = params.into_inner();
    let connection = pool.get().unwrap();
    match select_path_mediaType_by_digest(&reference, &connection).await {
        Ok((mediaType, path)) => {
            let mine = Mime::from_str(mediaType.as_str()).unwrap();
            match NamedFile::open_async(path).await {
                Ok(file) => {
                    update_pulltime_by_digest(&reference, &connection).await;
                    let file = file.set_content_type(mine).set_status_code(StatusCode::OK);
                    let mut response = file.into_response(&req);
                    response.headers_mut().append(
                        HeaderName::from_str("Docker-Content-Digest").unwrap(),
                        HeaderValue::from_str(reference.as_str()).unwrap(),
                    );
                    return response;
                }
                Err(_) => {}
            };
        }
        Err(_) => {
            match select_digest_mediaType_path_by_name_tag(&name, &reference, &connection).await {
                Ok((digest, mediaType, path)) => match NamedFile::open_async(path).await {
                    Ok(file) => {
                        update_pulltime_by_name_tag(&name, &reference, &connection).await;
                        let mut response = file
                            .set_content_type(Mime::from_str(mediaType.as_str()).unwrap())
                            .set_status_code(StatusCode::OK)
                            .into_response(&req);
                        response.headers_mut().append(
                            HeaderName::from_str("Docker-Content-Digest").unwrap(),
                            HeaderValue::from_str(digest.as_str()).unwrap(),
                        );
                        return response;
                    }
                    Err(_) => {}
                },
                Err(_) => {}
            }
        }
    };
    return HttpResponse::NotFound().finish();
}
#[allow(deprecated)]
#[get("/{name:.*}/blobs/{digest}")]
pub(crate) async fn get_blobs(
    params: web::Path<(String, String)>,
    pool: web::Data<DatabasePool>,
) -> Either<NamedFile, HttpResponse> {
    let (_, digest) = params.into_inner();
    let connection = pool.get().unwrap();
    match select_path_by_digest(&digest, &connection).await {
        Ok(path) => {
            match NamedFile::open_async(path).await {
                Ok(file) => return Either::Left(file.set_status_code(StatusCode::OK)),
                Err(_) => {}
            };
        }
        Err(_) => {}
    };
    return Either::Right(HttpResponse::NotFound().finish());
}
