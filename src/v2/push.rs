use actix_web::http::header;
use actix_web::http::header::{HeaderValue, RANGE};

// use std::slice::range;

use actix_web::{head, patch, post, put};
use actix_web::{web, HttpResponse, Responder};

use serde::Deserialize;
use sha256::digest_file;

use crate::dao::layers::{
    head_blobs_by_digest, insert_layers, insert_layers_with_mediaType, select_id_by_digest,
    update_mediaType_by_digest,
};
use redis::Commands;
use serde_json::Value;
use tokio::fs::{rename, File};
use tokio::io::AsyncWriteExt;
use tokio_stream::StreamExt;
use tracing::{error, info, span, Level};

use uuid::Uuid;

use crate::dao::config_layers::insert_config_layers;
use crate::dao::images::{insert_images, select_digest_by_name_tag, select_tag_by_name};

use crate::utils::{create_file, create_file_url};
use crate::v2::auth::SERVERHOST;

use crate::v2::content_range::ContentRange;
use crate::v2::content_type::ContentType;
use crate::v2::errs::ErrsInto;

use crate::{Calim, DatabasePool, RedisPool};

#[derive(Deserialize, Debug)]
pub struct Info {
    pub state: String,
    pub digest: Option<String>,
}
// HEAD /v2/library/ryuk/blobs/sha256: 4957e2cb9907bed50a1f8e29f61299bfeff3af20999c4227e68a883989939c11 HTTP/1.1\r\n
#[head("/{name:.*}/blobs/{digest}")]
pub(crate) async fn head_blobs(
    _: Calim,
    params: web::Path<(String, String)>,
    pool: web::Data<DatabasePool>,
) -> impl Responder {
    let (name_info, digest_info) = params.into_inner();
    let span = span!(
        Level::INFO,
        "head_blobs",
        name = name_info.as_str(),
        digest = digest_info.as_str()
    );
    let (prefix, digest_info) = digest_info.split_once(":").unwrap();
    let digest = prefix.trim().to_string() + ":" + digest_info.trim();
    let connection = pool.get().unwrap();
    let _entered = span.enter();
    return match head_blobs_by_digest(&digest, &connection).await {
        true => {
            info!("this blobs already exists");
            HttpResponse::Ok()
        }
        false => {
            error!("this blobs does not exist");
            HttpResponse::NotFound()
        }
    };
}
#[post("/{name:.*}/blobs/uploads/")]
pub(crate) async fn post_uploads_blobs(
    _: Calim,
    name: web::Path<String>,
    redis_pool: web::Data<RedisPool>,
) -> impl Responder {
    let name = name.into_inner();
    let span = span!(Level::INFO, "post_uploads_blobs", name = name.as_str());
    let state = Uuid::new_v4().to_string();
    let uuid = Uuid::new_v4().to_string();
    let location = geneate_location(SERVERHOST.to_string(), &name, &uuid, &state);
    let dockeruuid = HeaderValue::from_str(uuid.as_str()).unwrap();
    let _entered = span.enter();
    let mut connection = redis_pool.get().unwrap();
    let res: u8 = connection.hset(&name, &uuid, &state).unwrap();
    if res != 1 {
        return ErrsInto::RedisFailed.error_response();
    }
    let range = HeaderValue::from_str("0-0").unwrap();
    HttpResponse::Accepted()
        .insert_header(("Location", location))
        .insert_header(("Docker-Upload-Uuid", dockeruuid))
        .insert_header((RANGE, range))
        .finish()
}

#[patch("/{name:.*}/blobs/uploads/{uuid}")]
pub(crate) async fn patch_uploads_blobs(
    _: Calim,
    params: web::Path<(String, String)>,
    query: web::Query<Info>,
    ContentRange(content_range): ContentRange,
    mut palyod: web::Payload,
    redis_pool: web::Data<RedisPool>,
) -> impl Responder {
    let (name, uuid) = params.into_inner();
    let span = span!(
        Level::INFO,
        "patch_uploads_blobs",
        image_name = name.as_str(),
        uuid = uuid.as_str(),
        state = query.state.as_str()
    );
    let dockeruuid = HeaderValue::from_str(uuid.as_str()).unwrap();
    let _entered = span.enter();
    let mut connection = redis_pool.get().unwrap();
    let hexist: u8 = connection.hexists(&name, &uuid).unwrap();
    if hexist == 1 {
        let state_s: Option<String> = connection.hget(&name, &uuid).unwrap();
        match state_s {
            None => {
                error!(
                    "this request is invalidity,because map don't store info about the image_name"
                );
                return ErrsInto::RedisFailed.error_response();
            }
            Some(s) => {
                if s != query.state {
                    error!("this state may be failed");
                    return ErrsInto::RedisFailed.error_response();
                }
                let mut file = match create_file(name.as_str(), &(uuid.clone() + ".cache")).await {
                    Ok(file) => file,
                    Err(e) => return e.error_response(),
                };
                while let Some(item) = palyod.next().await {
                    let bytes = item.unwrap();
                    let x = bytes.as_ref();
                    match file.write_all(x).await {
                        Ok(_) => {}
                        Err(_) => {
                            error!("data can't be writed to local filesystem");
                            return HttpResponse::ExpectationFailed().finish();
                        }
                    }
                }
                match file.flush().await {
                    Ok(_) => {}
                    Err(_) => {
                        error!("data can't be flushed into local filesystem");
                        return HttpResponse::ExpectationFailed().finish();
                    }
                };
                let mut size_offset = file.metadata().await.unwrap().len() as usize;
                let range = if size_offset >= 1 {
                    size_offset = size_offset - 1;
                    HeaderValue::from_str(format!("0-{}", size_offset).as_str()).unwrap()
                } else {
                    HeaderValue::from_str("0-0").unwrap()
                };
                let mut content_length: usize = 0;
                match content_range {
                    None => {}
                    Some((start, end)) => {
                        if size_offset != end {
                            return HttpResponse::RangeNotSatisfiable().finish();
                        }
                        content_length = end - start;
                    }
                };
                let state = Uuid::new_v4().to_string();
                info!("this time blobs store successed,update the next request's state of the same fs_layers");
                let res: u8 = connection.hset(&name, &uuid, &state).unwrap();

                if res != 0 {
                    return ErrsInto::RedisFailed.error_response();
                }
                let location = geneate_location(SERVERHOST.to_string(), &name, &uuid, &state);
                return HttpResponse::Accepted()
                    .insert_header(("Location", location))
                    .insert_header(("Docker-Upload-Uuid", dockeruuid))
                    .insert_header((header::RANGE, range))
                    .insert_header((header::CONTENT_LENGTH, content_length))
                    .finish();
            }
        }
    }
    return ErrsInto::RedisFailed.error_response();
}
#[put("/{name:.*}/blobs/uploads/{uuid}")]
pub(crate) async fn put_uploads_blobs(
    _: Calim,
    params: web::Path<(String, String)>,
    query: web::Query<Info>,
    mut palyod: web::Payload,
    redis_pool: web::Data<RedisPool>,
    pool: web::Data<DatabasePool>,
) -> impl Responder {
    let (name, uuid) = params.into_inner();
    let span = span!(
        Level::INFO,
        "put_uploads_blobs",
        image_name = name.as_str(),
        uuid = uuid.as_str(),
        state = query.state.as_str()
    );
    // let mut shareddata = shared.state.lock().unwrap();
    let _entered = span.enter();

    let mut connection = redis_pool.get().unwrap();
    let database_connection = match pool.get() {
        Ok(con) => con,
        Err(_) => return ErrsInto::DataBasePoolFailed.error_response(),
    };

    match query.digest.clone() {
        None => {}
        Some(digest) => match select_id_by_digest(digest.as_str(), &database_connection).await {
            Ok(_) => {
                return HttpResponse::Created()
                    .insert_header(("Location", generate_blobs_location(&name, &digest)))
                    .insert_header(("Docker-Content-Digest", digest.as_str()))
                    .finish();
            }
            Err(_) => {}
        },
    }

    let hkeys: Vec<String> = connection.hkeys(&name).unwrap();
    if hkeys.contains(&uuid) {
        let state_s: Option<String> = connection.hget(&name, &uuid).unwrap();
        match state_s {
            None => {
                error!(
                    "this request is invalidity,because map don't store info about the image_name"
                );
                return HttpResponse::NotModified().finish();
            }
            Some(s) => {
                if s != query.state {
                    error!("this state may be failed");
                    return ErrsInto::RedisFailed.error_response();
                }
                let mut file = match create_file(name.as_str(), &(uuid.clone() + ".cache")).await {
                    Ok(file) => file,
                    Err(e) => return e.error_response(),
                };
                while let Some(item) = palyod.next().await {
                    let bytes = item.unwrap();
                    let x = bytes.as_ref();
                    match file.write_all(x).await {
                        Ok(_) => {}
                        Err(_) => {
                            error!("data can't be writed to local filesystem");
                            return HttpResponse::ExpectationFailed().finish();
                        }
                    }
                }
                match file.flush().await {
                    Ok(_) => {}
                    Err(_) => {
                        error!("data can't be flushed into local filesystem");
                        return HttpResponse::ExpectationFailed().finish();
                    }
                };
                let url = create_file_url(name.as_str(), &(uuid.clone() + ".cache"));
                match digest_file(&url) {
                    Err(_) => {
                        error!("digest file cache failed,may be file does not exists");
                        return HttpResponse::AlreadyReported().finish();
                    }
                    Ok(digest) => {
                        let digest = "sha256:".to_string() + &digest;
                        let newurl = create_file_url(name.as_str(), &digest);
                        match rename(url, &newurl).await {
                            Err(_) => {
                                error!("can't rename to digest");
                                return ErrsInto::FileRenameFailed.error_response();
                            }
                            Ok(_) => {
                                match select_id_by_digest(digest.as_str(), &database_connection)
                                    .await
                                {
                                    Ok(_) => {
                                        return HttpResponse::Created()
                                            .insert_header((
                                                "Location",
                                                generate_blobs_location(&name, &digest),
                                            ))
                                            .insert_header(("Docker-Content-Digest", digest))
                                            .finish();
                                    }
                                    Err(_) => {
                                        let size_info = File::open(&newurl)
                                            .await
                                            .unwrap()
                                            .metadata()
                                            .await
                                            .unwrap()
                                            .len()
                                            as u32;
                                        info!("this fslayer has been competely uploaded, digest: {}, size: {}, local filesystem path :{}",digest,size_info,newurl);
                                        // map.remove(&uuid);
                                        let x: u8 = connection.hdel(&name, &uuid).unwrap();
                                        if x != 1 {
                                            error!("hdel failed");
                                            return HttpResponse::MultipleChoices().finish();
                                        }
                                        return match insert_layers(
                                            &digest,
                                            size_info,
                                            &newurl,
                                            &database_connection,
                                        )
                                        .await
                                        {
                                            true => {
                                                info!("the info of the fslayer successfully insert into database");
                                                return HttpResponse::Created()
                                                    .insert_header((
                                                        "Location",
                                                        generate_blobs_location(&name, &digest),
                                                    ))
                                                    .insert_header((
                                                        "Docker-Content-Digest",
                                                        digest,
                                                    ))
                                                    .finish();
                                            }
                                            false => {
                                                error!("the info of the fslayer failed to insert into database");
                                                HttpResponse::ExpectationFailed().finish()
                                            }
                                        };
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    return HttpResponse::Conflict().finish();
}

#[put("/{name:.*}/manifests/{tag}")]
pub(crate) async fn put_uploads_manifests(
    _: Calim,
    params: web::Path<(String, String)>,
    ContentType(content_type): ContentType,
    mut palyod: web::Payload,
    pool: web::Data<DatabasePool>,
) -> impl Responder {
    let (name, tag) = params.into_inner();
    let connection = pool.get().unwrap();
    let span = span!(
        Level::INFO,
        "put_uploads_mainfests",
        name = name.as_str(),
        tag = tag.as_str()
    );
    match select_tag_by_name(&name, &connection).await {
        Ok(vec) => {
            if vec.contains(&tag) {
                match select_digest_by_name_tag(&name, &tag, &connection).await {
                    Ok(digest) => {
                        return HttpResponse::Created()
                            .insert_header(("Docker-Content-Digest", digest.as_str()))
                            .insert_header((
                                header::LOCATION,
                                generate_manifests_location(&name, &digest),
                            ))
                            .finish();
                    }
                    Err(_) => {}
                };
            }
        }
        Err(_) => {}
    }
    let _entered = span.enter();
    let mut bytes = web::BytesMut::new();
    while let Some(item) = palyod.next().await {
        let bytes1 = item.unwrap();
        bytes.extend_from_slice(&bytes1);
    }
    let data = String::from_utf8(bytes.to_vec()).unwrap();

    let uuid = Uuid::new_v4().to_string();
    let url = create_file_url(name.as_str(), &(uuid.clone() + ".cache"));
    let mut file = match create_file(name.as_str(), &(uuid + ".cache")).await {
        Ok(file) => file,
        Err(e) => return e.error_response(),
    };
    match file.write_all(bytes.as_ref()).await {
        Ok(_) => {
            match file.flush().await {
                Ok(_) => {}
                Err(_) => {
                    error!("data can't be flushed into local file system");
                    return HttpResponse::ExpectationFailed().finish();
                }
            };
        }
        Err(_) => {
            error!("data can't be writed into local file system");
            return HttpResponse::ExpectationFailed().finish();
        }
    };

    let size_info = file.metadata().await.unwrap().len() as u32;
    let sha = sha256::digest_file(&url).unwrap();
    let digest_config = "sha256:".to_string() + &sha;
    let newurl = create_file_url(name.as_str(), &digest_config);
    match rename(&url, &newurl).await {
        Ok(_) => {}
        Err(_) => {
            error!("rename failed");
            return HttpResponse::ExpectationFailed().finish();
        }
    };

    insert_layers_with_mediaType(
        &digest_config,
        size_info,
        &content_type,
        &newurl,
        &connection,
    )
    .await;

    let serde_result: Result<Value, ErrsInto> =
        serde_json::from_str(data.as_str()).map_err(|_| ErrsInto::ManifestInvalid);
    let manifests = match serde_result {
        Ok(v) => v,
        Err(e) => return e.error_response(),
    };
    match update_mediaType_by_digest(
        manifests["config"]["digest"].as_str().unwrap(),
        manifests["config"]["mediaType"].as_str().unwrap(),
        &connection,
    )
    .await
    {
        false => {
            return ErrsInto::ManifestInvalid.error_response();
        }
        true => {
            let connection = pool.get().unwrap();
            let id = match select_id_by_digest(
                manifests["config"]["digest"].as_str().unwrap(),
                &connection,
            )
            .await
            {
                Ok(id) => id,
                Err(_) => {
                    error!(
                        "select id by digest {} failed",
                        manifests["config"]["digest"].as_str().unwrap()
                    );

                    return ErrsInto::ManifestInvalid.error_response();
                }
            };
            let mut layerids = vec![id];

            let mut layers_iter = manifests["layers"].as_array().unwrap().iter();

            let connection1 = pool.get().unwrap();
            while let Some(layer) = layers_iter.next() {
                match update_mediaType_by_digest(
                    layer["digest"].as_str().unwrap(),
                    layer["mediaType"].as_str().unwrap(),
                    &connection1,
                )
                .await
                {
                    false => {
                        error!(
                            "updated mediaType failed the digest is {} \
                use default value \
                'application/vnd.docker.image.rootfs.diff.tar.gzip'",
                            layer["digest"].as_str().unwrap()
                        );

                        return ErrsInto::ManifestInvalid.error_response();
                    }
                    true => {
                        match select_id_by_digest(layer["digest"].as_str().unwrap(), &connection)
                            .await
                        {
                            Ok(id) => {
                                layerids.push(id);
                            }
                            Err(_) => return ErrsInto::ManifestInvalid.error_response(),
                        };
                    }
                };
            }

            let config_id = match select_id_by_digest(&digest_config, &connection).await {
                Ok(id) => id,
                Err(_) => {
                    error!("config layer select id by digest {}", digest_config);
                    return ErrsInto::DigestFailed.error_response();
                }
            };

            match insert_config_layers(&config_id, &layerids, &connection).await {
                true => {
                    insert_images(&name, &tag, config_id, &connection).await;
                    return HttpResponse::Created()
                        .insert_header(("Docker-Content-Digest", digest_config.as_str()))
                        .insert_header((
                            header::LOCATION,
                            generate_manifests_location(&name, &digest_config),
                        ))
                        .finish();
                }
                false => {
                    return ErrsInto::ManifestInvalid.error_response();
                }
            };
        }
    };
}
fn generate_manifests_location(name: &String, digest: &String) -> HeaderValue {
    let loc = SERVERHOST.to_string() + "/v2/" + name + "/manifests/" + digest;
    HeaderValue::from_str(loc.as_str()).unwrap()
}
fn generate_blobs_location(name: &String, digest: &String) -> HeaderValue {
    let loc = SERVERHOST.to_string() + "/v2/" + name + "/blobs/" + digest;
    HeaderValue::from_str(loc.as_str()).unwrap()
}
fn geneate_location(host: String, name: &String, uuid: &String, state: &String) -> HeaderValue {
    let location = host + "/v2/" + name + "/blobs/uploads/" + uuid + "?state=" + state;
    HeaderValue::from_str(location.as_str()).unwrap()
}
#[cfg(test)]
mod tests {
    use std::fs::{File, OpenOptions};
    use std::io::{Read, Write};

    #[test]
    fn test1() {
        // /home/csmsoledad/下载/OVGradientDescent (1).html
        let mut result = File::open("./src/test.txt").unwrap();
        // result.
        // let mut file = File::create("/home/csmsoledad/图片/aaaa.html").unwrap();
        let mut file1 = OpenOptions::new()
            .append(true)
            .create(true)
            .open("./src/a.txt")
            .unwrap();
        let mut buff: [u8; 1024] = [0; 1024];
        // let i = ;
        let mut accout = 0;
        loop {
            let i = result.read(&mut buff[..]).unwrap();
            if i > 0 {
                let result1 = file1.write(&mut buff[..i]).unwrap();
                accout = accout + 1;
            } else {
                break;
            }
        }
        println!("{}", accout);
    }
    #[test]
    fn test2() {
        let file = File::open("/home/csmsoledad/图片/a/1.jpg").unwrap();
        let metadata = file.metadata().unwrap();
        let i = metadata.len();
        println!("{}", i);
    }
}
