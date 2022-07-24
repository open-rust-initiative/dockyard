use crate::dao::images::select_configid_by_name_tag;
use crate::dao::layers::{delete_layer_by_digest, delete_layer_by_id, select_id_by_digest};
use crate::{Calim, DatabasePool};

use actix_web::delete;
use actix_web::{web, HttpResponse, Responder};

#[delete("/{name:.*}/blobs/{digest}")]
pub(crate) async fn delete_blobs(
    _: Calim,
    params: web::Path<(String, String)>,
    pool: web::Data<DatabasePool>,
) -> impl Responder {
    let (_name, digest) = params.into_inner();
    let connection = pool.get().unwrap();
    return match delete_layer_by_digest(&digest, &connection).await {
        Ok(_) => HttpResponse::Accepted().finish(),
        Err(_) => HttpResponse::BadRequest().finish(),
    };
}
#[delete("/{name:.*}/manifests/{reference}")]
pub async fn delete_manifests(
    _: Calim,
    params: web::Path<(String, String)>,
    pool: web::Data<DatabasePool>,
) -> impl Responder {
    let (name, reference) = params.into_inner();
    let connection = pool.get().unwrap();
    let configid = match select_id_by_digest(reference.as_str(), &connection).await {
        Ok(id) => id,
        Err(_) => match select_configid_by_name_tag(&name, &reference, &connection).await {
            Ok(id) => id,
            Err(_) => return HttpResponse::NotFound(),
        },
    };
    return match delete_layer_by_id(configid, &connection).await {
        Ok(_) => HttpResponse::Accepted(),
        Err(_) => HttpResponse::NotFound(),
    };
}
