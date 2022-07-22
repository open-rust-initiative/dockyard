use crate::dao::DatabaseConnect;
use crate::schema::fs_layers;
use diesel::result::Error;
use diesel::{EqAll, Insertable, QueryDsl, RunQueryDsl};

// use tracing::{error, info, Level};

pub async fn head_blobs_by_digest(digest_info: &String, connect: &DatabaseConnect) -> bool {
    return match fs_layers::table
        .filter(fs_layers::digest.eq_all(digest_info))
        .select(fs_layers::id)
        .load::<u32>(connect)
    {
        Ok(ids) => ids.len() == 1,
        Err(_) => false,
    };
}
#[derive(Insertable)]
#[table_name = "fs_layers"]
#[allow(non_snake_case)]
pub struct NewLayer<'a> {
    pub digest: &'a String,
    pub size: u32,
    pub path: &'a String,
}
#[allow(non_snake_case)]
pub async fn insert_layers<'a>(
    digest: &String,
    size: u32,
    path: &String,
    connect: &DatabaseConnect,
) -> bool {
    let layer = NewLayer { digest, size, path };
    match diesel::insert_into(fs_layers::table)
        .values(&layer)
        .execute(connect)
    {
        Ok(size) => size == 1 || size == 0,
        Err(_) => false,
    }
}
#[derive(Insertable)]
#[table_name = "fs_layers"]
#[allow(non_snake_case)]
pub struct NewLayerWithMediaType<'a> {
    pub digest: &'a String,
    pub size: u32,
    pub mediaType: &'a String,
    pub path: &'a String,
}
#[allow(non_snake_case)]
pub async fn insert_layers_with_mediaType(
    digest: &String,
    size: u32,
    mediaType: &String,
    path: &String,
    connect: &DatabaseConnect,
) -> bool {
    let layer = NewLayerWithMediaType {
        digest,
        size,
        mediaType,
        path,
    };
    match diesel::insert_into(fs_layers::table)
        .values(&layer)
        .execute(connect)
    {
        Ok(size) => size == 1,
        Err(_) => false,
    }
}
#[allow(non_snake_case)]
pub async fn update_mediaType_by_digest(
    digest: &str,
    mediaType: &str,
    connect: &DatabaseConnect,
) -> bool {
    return match diesel::update(fs_layers::table.filter(fs_layers::digest.eq_all(digest)))
        .set(fs_layers::mediaType.eq_all(mediaType))
        .execute(connect)
    {
        Ok(a) => a == 1 || a == 0,
        Err(_) => false,
    };
}
pub async fn select_id_by_digest(digest: &str, connect: &DatabaseConnect) -> Result<u32, Error> {
    return match fs_layers::table
        .filter(fs_layers::digest.eq_all(digest))
        .select(fs_layers::id)
        .load::<u32>(connect)
    {
        Ok(a) => {
            if a.len() == 1 {
                Ok(a[0])
            } else {
                Err(Error::NotFound)
            }
        }
        Err(a) => Err(a),
    };
}
#[allow(non_snake_case)]
pub async fn select_path_mediaType_by_digest(
    digest: &String,
    connect: &DatabaseConnect,
) -> Result<(String, String), Error> {
    let result: Result<Vec<(String, String)>, diesel::result::Error> = fs_layers::table
        .filter(fs_layers::digest.eq_all(digest))
        .select((fs_layers::mediaType, fs_layers::path))
        .load::<(String, String)>(connect);
    match result {
        Ok(k) => {
            if k.len() == 1 {
                let x = k.get(0).unwrap().clone();
                return Ok(x);
            }
        }
        Err(_) => {}
    }
    return Err(Error::NotFound);
}
#[allow(non_snake_case)]
pub async fn select_path_by_digest(
    digest: &String,
    connect: &DatabaseConnect,
) -> Result<String, Error> {
    let result: Result<Vec<String>, diesel::result::Error> = fs_layers::table
        .filter(fs_layers::digest.eq_all(digest))
        .select(fs_layers::path)
        .load::<String>(connect);
    match result {
        Ok(k) => {
            if k.len() == 1 {
                let x = k.get(0).unwrap().clone();
                return Ok(x);
            }
        }
        Err(_) => {}
    }
    return Err(Error::NotFound);
}
#[allow(non_snake_case)]
pub async fn select_mediaType_by_digest(
    digest: &String,
    connect: &DatabaseConnect,
) -> Result<String, Error> {
    let result: Result<Vec<String>, diesel::result::Error> = fs_layers::table
        .filter(fs_layers::digest.eq_all(digest))
        .select(fs_layers::mediaType)
        .load::<String>(connect);
    match result {
        Ok(k) => {
            if k.len() == 1 {
                let x = k.get(0).unwrap().clone();
                return Ok(x);
            }
        }
        Err(_) => {}
    }
    return Err(Error::NotFound);
}
pub async fn delete_layer_by_digest(
    digest: &String,
    connect: &DatabaseConnect,
) -> Result<usize, diesel::result::Error> {
    let path = fs_layers::table
        .filter(fs_layers::digest.eq_all(digest))
        .select(fs_layers::path)
        .first::<String>(connect)?;

    match diesel::delete(fs_layers::table.filter(fs_layers::digest.eq_all(digest))).execute(connect)
    {
        Ok(result) => match tokio::fs::remove_file(path).await {
            Ok(_) => return Ok(result),
            Err(_) => {}
        },
        Err(_) => {}
    };
    return Err(diesel::result::Error::NotFound);
}
pub async fn delete_layer_by_id(
    id: u32,
    connect: &DatabaseConnect,
) -> Result<usize, diesel::result::Error> {
    let path = fs_layers::table
        .filter(fs_layers::id.eq_all(id))
        .select(fs_layers::path)
        .first::<String>(connect)?;
    match diesel::delete(fs_layers::table.filter(fs_layers::id.eq_all(id))).execute(connect) {
        Ok(result) => match tokio::fs::remove_file(path).await {
            Ok(_) => return Ok(result),
            Err(_) => {}
        },
        Err(_) => {}
    };
    return Err(diesel::result::Error::NotFound);
}
