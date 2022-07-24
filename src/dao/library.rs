use std::borrow::Borrow;
use std::collections::HashSet;
use std::string::String;

use diesel::query_dsl::methods::OffsetDsl;
use diesel::result::Error;
use diesel::{EqAll, ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl};

use crate::dao::layers::delete_layer_by_id;
use crate::dao::DatabaseConnect;
use crate::schema::{config_layers, fs_layers, images};

pub async fn select_library_limit_offset(
    limit: &Option<usize>,
    offset: &Option<usize>,
    connect: &DatabaseConnect,
) -> Result<Vec<String>, diesel::result::Error> {
    let result: Result<Vec<String>, diesel::result::Error> = match offset {
        None => match limit {
            None => QueryDsl::distinct(
                images::table
                    .order_by(images::library.asc())
                    .select(images::library),
            )
            .limit(15)
            .load::<String>(connect),
            Some(limit) => QueryDsl::distinct(
                images::table
                    .order_by(images::library.asc())
                    .select(images::library),
            )
            .limit(*limit as i64)
            .load::<String>(connect),
        },
        Some(offset) => match limit {
            None => OffsetDsl::offset(
                QueryDsl::distinct(
                    images::table
                        .order_by(images::library.asc())
                        .select(images::library),
                ),
                *offset as i64,
            )
            .limit(15)
            .load::<String>(connect),
            Some(limit) => OffsetDsl::offset(
                QueryDsl::distinct(
                    images::table
                        .order_by(images::library.asc())
                        .select(images::library),
                ),
                *offset as i64,
            )
            .limit(*limit as i64)
            .load::<String>(connect),
        },
    };
    return result;
}
// select size from images
// inner join config_layers cl on images.fslayer_configid = cl.configid
// inner join fs_layers fL on cl.layersid = fL.id
// where images.library='default'
pub async fn count_size_by_libray(
    library: &String,
    connect: &DatabaseConnect,
) -> Result<f32, diesel::result::Error> {
    // joinable!(config_layers->)
    let result: Vec<(u32, u32)> = QueryDsl::distinct(
        images::table
            .inner_join(
                config_layers::table.on(images::fslayer_configid.eq_all(config_layers::configid)),
            )
            .inner_join(fs_layers::table.on(config_layers::layersid.eq_all(fs_layers::id)))
            .filter(images::library.eq_all(library))
            .select((config_layers::layersid, fs_layers::size)),
    )
    .load::<(u32, u32)>(connect)?;
    let mut count: u32 = 0;
    for x in result {
        count += x.1;
    }
    return Ok(count as f32 / 1024.0 / 1024.0);
}
pub async fn count_image_by_library(
    library: &String,
    connect: &DatabaseConnect,
) -> Result<usize, diesel::result::Error> {
    let x = QueryDsl::distinct(
        images::table
            .filter(images::library.eq_all(library))
            .select(images::name),
    )
    .load::<String>(connect)?;
    let i = x.iter().count();
    return Ok(i);
}
pub async fn count_tag_by_library(
    library: &String,
    connect: &DatabaseConnect,
) -> Result<usize, diesel::result::Error> {
    let vec = QueryDsl::distinct(
        images::table
            .filter(images::library.eq_all(library))
            .select(images::tag),
    )
    .load::<String>(connect)?;
    let i = vec.iter().count();
    return Ok(i);
}
// return match diesel::delete(fs_layers::table.filter(fs_layers::digest.eq_all(digest)))
// .execute(connect)
// {
// Ok(s) => s == 1,
// Err(_) => false,
// };
pub async fn delete_library_by_name(
    library: &String,
    connect: &DatabaseConnect,
) -> Result<usize, diesel::result::Error> {
    let configids = QueryDsl::distinct(
        images::table
            .filter(images::library.eq_all(library))
            .select(images::fslayer_configid),
    )
    .load::<u32>(connect)?;
    let layers: Vec<u32> = QueryDsl::distinct(
        images::table
            .inner_join(
                config_layers::table.on(images::fslayer_configid.eq_all(config_layers::configid)),
            )
            .filter(images::library.eq_all(library))
            .select(config_layers::layersid),
    )
    .load::<u32>(connect)?;
    let vec = [configids, layers].concat();
    let set = vec.into_iter().collect::<HashSet<u32>>();
    let mut count: usize = 0;
    for x in set.borrow() {
        let statement = delete_layer_by_id(*x, connect).await?;
        count += statement;
    }
    if count != set.len() {
        return Err(Error::NotFound);
    }
    return Ok(count);
}
pub async fn delete_all_imageitem_by_name(
    name: &String,
    connect: &DatabaseConnect,
) -> Result<usize, diesel::result::Error> {
    let configids = QueryDsl::distinct(
        images::table
            .filter(images::name.eq_all(name))
            .select(images::fslayer_configid),
    )
    .load::<u32>(connect)?;
    let layers: Vec<u32> = QueryDsl::distinct(
        images::table
            .inner_join(
                config_layers::table.on(images::fslayer_configid.eq_all(config_layers::configid)),
            )
            .filter(images::name.eq_all(name))
            .select(config_layers::layersid),
    )
    .load::<u32>(connect)?;
    let vec = [configids, layers].concat();
    let set = vec.into_iter().collect::<HashSet<u32>>();
    let mut count: usize = 0;
    for x in set.borrow() {
        let statement = delete_layer_by_id(*x, connect).await?;
        count += statement;
    }
    if count != set.len() {
        return Err(Error::NotFound);
    }
    return Ok(count);
}
pub async fn delete_imageitem_by_name_tag(
    name: &String,
    tag: &String,
    connect: &DatabaseConnect,
) -> Result<usize, diesel::result::Error> {
    let configid: u32 = images::table
        .filter(images::name.eq_all(name))
        .filter(images::tag.eq_all(tag))
        .select(images::fslayer_configid)
        .first::<u32>(connect)?;
    let layers: Vec<u32> = images::table
        .inner_join(
            config_layers::table.on(images::fslayer_configid.eq_all(config_layers::configid)),
        )
        .filter(images::name.eq_all(name))
        .filter(images::tag.eq_all(tag))
        .select(config_layers::layersid)
        .load::<u32>(connect)?;
    let mut count: usize = 0;
    delete_layer_by_id(configid, connect).await?;
    for x in layers {
        let statement = delete_layer_by_id(x, connect).await?;
        count += statement;
    }
    return Ok(count);
}
pub async fn count_library(connect: &DatabaseConnect) -> Result<i64, diesel::result::Error> {
    let result = QueryDsl::count(QueryDsl::distinct(images::table.select(images::library)))
        .first::<i64>(connect)?;
    return Ok(result);
}
#[cfg(test)]
mod tests {
    use std::borrow::Borrow;
    use std::ops::Add;
    use std::string::String;
    use std::thread::sleep;
    use std::time::{Duration, SystemTime, UNIX_EPOCH};

    use actix_web::http::header::{TryIntoHeaderValue, AUTHORIZATION};
    use actix_web::test::TestRequest;
    use actix_web::{web, FromRequest};
    use diesel::result::Error;
    use uuid::Uuid;

    use crate::dao::library::{
        count_image_by_library, count_library, count_size_by_libray, count_tag_by_library,
        delete_imageitem_by_name_tag, delete_library_by_name, select_library_limit_offset,
    };
    use crate::v2::authorization::Bearer;
    use crate::{database_pool, Authorization, Calim};

    #[actix_rt::test]
    async fn test1() {
        dotenv::dotenv().ok();
        let pool = database_pool().get().unwrap();
        let string = String::from("library");
        let x = count_size_by_libray(&string, &pool).await;
        match x {
            Ok(vec) => {
                println!("{:?}", vec);
            }
            Err(_) => {}
        }
    }
    #[actix_rt::test]
    async fn test2() {
        dotenv::dotenv().ok();
        let pool = database_pool().get().unwrap();
        match select_library_limit_offset(&Some(10), &Some(10), &pool).await {
            Ok(v) => {
                println!("{:?}", v);
            }
            Err(_) => {}
        }
    }
    #[actix_rt::test]
    async fn test3() {
        dotenv::dotenv().ok();
        let pool = database_pool().get().unwrap();
        match count_image_by_library(&String::from("library"), &pool).await {
            Ok(v) => {
                println!("{:?}", v);
            }
            Err(_) => {}
        }
    }
    #[actix_rt::test]
    async fn test4() {
        dotenv::dotenv().ok();
        let pool = database_pool().get().unwrap();
        match count_tag_by_library(&String::from("default"), &pool).await {
            Ok(v) => {
                println!("{:?}", v);
            }
            Err(_) => {}
        }
    }
    #[actix_rt::test]
    async fn test5() {
        dotenv::dotenv().ok();
        let pool = database_pool().get().unwrap();
        match delete_library_by_name(&String::from("linux"), &pool).await {
            Ok(v) => {
                println!("{:?}", v);
            }
            Err(_) => {}
        }
    }
    #[actix_rt::test]
    async fn test6() {
        dotenv::dotenv().ok();
        let pool = database_pool().get().unwrap();
        match count_library(&pool).await {
            Ok(v) => {
                println!("{:?}", v);
            }
            Err(_) => {}
        }
    }
    #[actix_rt::test]
    async fn test7() {
        dotenv::dotenv().ok();
        let pool = database_pool().get().unwrap();
        let name = String::from("linux/debian");
        let tag = String::from("latest");
        match delete_imageitem_by_name_tag(&name, &tag, &pool).await {
            Ok(v) => {
                println!("{:?}", v);
            }
            Err(_) => {
                println!("error")
            }
        }
    }
}
