use chrono::{NaiveDateTime, Utc};
use diesel::query_dsl::methods::OffsetDsl;
use diesel::result::Error;
use diesel::{EqAll, ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl};
use serde::{Deserialize, Serialize};

use crate::dao::layers::select_id_by_digest;
use crate::dao::DatabaseConnect;
use crate::schema::config_layers;
use crate::schema::fs_layers;
use crate::schema::images;

#[derive(Insertable)]
#[table_name = "images"]
pub struct NewImage<'a> {
    pub library: &'a str,
    pub name: &'a str,
    pub tag: &'a String,
    pub fslayer_configid: u32,
}
pub async fn insert_images<'a>(
    fullname: &String,
    tag: &String,
    fslayer_configid: u32,
    connect: &DatabaseConnect,
) -> bool {
    let (library, name) = match fullname.split_once("/") {
        None => ("default", fullname.as_str()),
        Some((lib, _)) => (lib, fullname.as_str()),
    };
    let new_image = NewImage {
        library,
        name,
        tag,
        fslayer_configid,
    };

    match diesel::insert_into(images::table)
        .values(&new_image)
        .execute(connect)
    {
        Ok(num) => {
            if num == 1 {
                return true;
            }
            return false;
        }
        Err(_) => return false,
    };
}
pub async fn select_tag_by_name(
    fullname: &String,
    connect: &DatabaseConnect,
) -> Result<Vec<String>, Error> {
    match images::table
        .filter(images::name.eq_all(fullname))
        .select(images::tag)
        .load::<String>(connect)
    {
        Ok(v) => return Ok(v),
        Err(e) => Err(e),
    }
}
pub async fn select_digest_by_name_tag(
    fullname: &String,
    tag: &String,
    connect: &DatabaseConnect,
) -> Result<String, diesel::result::Error> {
    let result: Result<Vec<String>, diesel::result::Error> = images::table
        .inner_join(fs_layers::table)
        .filter(images::name.eq_all(fullname))
        .filter(images::tag.eq_all(tag))
        .select(fs_layers::digest)
        .load::<String>(connect);
    match result {
        Ok(v) => {
            if v.len() == 1 {
                let x = v.get(0).unwrap().clone();
                return Ok(x);
            }
        }
        Err(_) => {}
    }
    return Err(Error::NotFound);
}
pub async fn select_configid_by_name_tag(
    fullname: &String,
    tag: &String,
    connect: &DatabaseConnect,
) -> Result<u32, diesel::result::Error> {
    let result: Result<Vec<u32>, diesel::result::Error> = images::table
        .filter(images::name.eq_all(fullname))
        .filter(images::tag.eq_all(tag))
        .select(images::fslayer_configid)
        .load::<u32>(connect);
    match result {
        Ok(v) => {
            if v.len() == 1 {
                let i = v.get(0).unwrap().clone();
                return Ok(i);
            }
        }
        Err(_) => {}
    }
    return Err(Error::NotFound);
}
#[allow(non_snake_case)]
pub async fn select_digest_mediaType_by_name_tag(
    fullname: &String,
    tag: &String,
    connect: &DatabaseConnect,
) -> Result<(String, String), diesel::result::Error> {
    let result: Result<Vec<(String, String)>, diesel::result::Error> = images::table
        .inner_join(fs_layers::table)
        .filter(images::name.eq_all(fullname))
        .filter(images::tag.eq_all(tag))
        .select((fs_layers::digest, fs_layers::mediaType))
        .load::<(String, String)>(connect);
    match result {
        Ok(v) => {
            if v.len() == 1 {
                let (digest, mediaType) = v.get(0).unwrap().clone();
                return Ok((digest, mediaType));
            }
        }
        Err(_) => {}
    }
    return Err(Error::NotFound);
}
#[allow(non_snake_case)]
pub async fn select_digest_mediaType_path_by_name_tag(
    fullname: &String,
    tag: &String,
    connect: &DatabaseConnect,
) -> Result<(String, String, String), diesel::result::Error> {
    let result: Result<Vec<(String, String, String)>, diesel::result::Error> = images::table
        .inner_join(fs_layers::table)
        .filter(images::name.eq_all(fullname))
        .filter(images::tag.eq_all(tag))
        .select((fs_layers::digest, fs_layers::mediaType, fs_layers::path))
        .load::<(String, String, String)>(connect);
    match result {
        Ok(v) => {
            if v.len() == 1 {
                let (digest, mediaType, path) = v.get(0).unwrap().clone();
                return Ok((digest, mediaType, path));
            }
        }
        Err(_) => {}
    }
    return Err(Error::NotFound);
}
#[allow(dead_code)]
pub async fn select_tags_by_name(
    fullname: &String,
    connect: &DatabaseConnect,
) -> Result<Vec<String>, diesel::result::Error> {
    let result: Result<Vec<String>, diesel::result::Error> = images::table
        .filter(images::name.eq_all(fullname))
        .select(images::tag)
        .load::<String>(connect);
    return result;
}
// return match diesel::update(fs_layers::table.filter(fs_layers::digest.eq_all(digest)))
// .set(fs_layers::mediaType.eq_all(mediaType))
// .execute(connect)
pub async fn update_pulltime_by_name_tag(
    name: &String,
    tag: &String,
    connect: &DatabaseConnect,
) -> bool {
    let time = chrono::NaiveDateTime::from_timestamp(Utc::now().timestamp(), 0);
    let result = diesel::update(
        images::table
            .filter(images::name.eq_all(name))
            .filter(images::tag.eq_all(tag)),
    )
    .set(images::pull_time.eq(time))
    .execute(connect)
    .unwrap();
    if result == 1 {
        true
    } else {
        false
    }
}
pub async fn update_pulltime_by_digest(digest: &String, connect: &DatabaseConnect) -> bool {
    match select_id_by_digest(digest, connect).await {
        Ok(id) => {
            let time = chrono::NaiveDateTime::from_timestamp(Utc::now().timestamp(), 0);
            let i = diesel::update(images::table.filter(images::fslayer_configid.eq_all(id)))
                .set(images::pull_time.eq(time))
                .execute(connect)
                .unwrap();
            if i == 1 {
                return true;
            }
        }
        Err(_) => {}
    }
    false
}
pub async fn select_tags_by_name_from_last_limit_n(
    fullname: &String,
    last: Option<String>,
    n: Option<usize>,
    connect: &DatabaseConnect,
) -> Result<Vec<String>, diesel::result::Error> {
    let statement = images::table.filter(images::name.eq_all(fullname));
    let result: Result<Vec<String>, diesel::result::Error> = match last {
        Some(last) => match n {
            Some(n) => statement
                .filter(images::tag.gt(last))
                .order_by(images::tag.asc())
                .limit(n as i64)
                .select(images::tag)
                .load::<String>(connect),
            None => statement
                .filter(images::tag.gt(last))
                .order_by(images::tag.asc())
                .select(images::tag)
                .load::<String>(connect),
        },
        None => match n {
            Some(n) => statement
                .order_by(images::tag.asc())
                .limit(n as i64)
                .select(images::tag)
                .load::<String>(connect),
            None => statement
                .order_by(images::tag.asc())
                .select(images::tag)
                .load::<String>(connect),
        },
    };
    return result;
}
pub async fn select_name_limit_offset_by_library(
    library: &String,
    limit: &Option<usize>,
    offset: &Option<usize>,
    connect: &DatabaseConnect,
) -> Result<Vec<String>, diesel::result::Error> {
    let result: Result<Vec<String>, diesel::result::Error> = match offset {
        None => match limit {
            None => QueryDsl::distinct(
                images::table
                    .filter(images::library.eq_all(library))
                    .order_by(images::name.asc())
                    .select(images::name),
            )
            .limit(10)
            .load::<String>(connect),
            Some(limit) => QueryDsl::distinct(
                images::table
                    .filter(images::library.eq_all(library))
                    .order_by(images::name.asc())
                    .select(images::name),
            )
            .limit(*limit as i64)
            .load::<String>(connect),
        },
        Some(offset) => match limit {
            None => OffsetDsl::offset(
                QueryDsl::distinct(
                    images::table
                        .filter(images::library.eq_all(library))
                        .order_by(images::name.asc())
                        .select(images::name),
                ),
                *offset as i64,
            )
            .limit(15)
            .load::<String>(connect),
            Some(limit) => OffsetDsl::offset(
                QueryDsl::distinct(
                    images::table
                        .filter(images::library.eq_all(library))
                        .order_by(images::name.asc())
                        .select(images::name),
                ),
                *offset as i64,
            )
            .limit(*limit as i64)
            .load::<String>(connect),
        },
    };
    return result;
}
#[derive(Serialize, Deserialize, Debug)]
pub struct TimeInfo {
    pub first_creation: String,
    pub last_creation: String,
    pub last_push: String,
    pub last_pull: String,
}
pub async fn select_timeinfo_by_name(
    name: &String,
    connect: &DatabaseConnect,
) -> Result<TimeInfo, diesel::result::Error> {
    let vec = images::table
        .filter(images::name.eq_all(name))
        .select((images::creation_time, images::pull_time, images::push_time))
        .load::<(
            chrono::NaiveDateTime,
            chrono::NaiveDateTime,
            chrono::NaiveDateTime,
        )>(connect)?;
    let (creation, pull, push) = vec.get(0).unwrap().clone();
    let mut last_creation: NaiveDateTime = creation;
    let mut first_creation: NaiveDateTime = creation.clone();
    let mut last_push: NaiveDateTime = pull;
    let mut last_pull: NaiveDateTime = push;
    for (creation, pull, push) in vec {
        if creation > last_creation {
            last_creation = creation
        } else if creation < first_creation {
            first_creation = creation
        }
        if pull > last_pull {
            last_pull = pull
        }
        if push > last_push {
            last_push = push
        }
    }

    let timeinfo = TimeInfo {
        first_creation: first_creation.to_string(),
        last_creation: last_creation.to_string(),
        last_push: last_push.to_string(),
        last_pull: last_pull.to_string(),
    };
    Ok(timeinfo)
}
pub async fn count_size_by_name(
    name: &String,
    connect: &DatabaseConnect,
) -> Result<f32, diesel::result::Error> {
    let result: Vec<(u32, u32)> = QueryDsl::distinct(
        images::table
            .inner_join(
                config_layers::table.on(images::fslayer_configid.eq_all(config_layers::configid)),
            )
            .inner_join(fs_layers::table.on(config_layers::layersid.eq_all(fs_layers::id)))
            .filter(images::name.eq_all(name))
            .select((config_layers::layersid, fs_layers::size)),
    )
    .load::<(u32, u32)>(connect)?;
    let mut count: u32 = 0;
    for x in result {
        count += x.1;
    }
    return Ok(count as f32 / 1024.0 / 1024.0);
}
pub async fn count_name_by_library(
    library: &String,
    connect: &DatabaseConnect,
) -> Result<usize, diesel::result::Error> {
    let count = QueryDsl::distinct(
        images::table
            .filter(images::library.eq(library))
            .select(images::name),
    )
    .load::<String>(connect)?;

    return Ok(count.len());
}
pub async fn select_tags_limit_offset_by_name(
    name: &String,
    limit: &Option<usize>,
    offset: &Option<usize>,
    connect: &DatabaseConnect,
) -> Result<Vec<(String, NaiveDateTime, NaiveDateTime, NaiveDateTime)>, diesel::result::Error> {
    let result: Result<
        Vec<(String, NaiveDateTime, NaiveDateTime, NaiveDateTime)>,
        diesel::result::Error,
    > = match offset {
        None => match limit {
            None => images::table
                .filter(images::name.eq_all(name))
                .order_by(images::tag.desc())
                .select((
                    images::tag,
                    images::creation_time,
                    images::pull_time,
                    images::push_time,
                ))
                .load::<(String, NaiveDateTime, NaiveDateTime, NaiveDateTime)>(connect),
            Some(limit) => images::table
                .filter(images::name.eq_all(name))
                .order_by(images::tag.desc())
                .select((
                    images::tag,
                    images::creation_time,
                    images::pull_time,
                    images::push_time,
                ))
                .limit(*limit as i64)
                .load::<(String, NaiveDateTime, NaiveDateTime, NaiveDateTime)>(connect),
        },
        Some(offset) => match limit {
            None => OffsetDsl::offset(
                images::table
                    .filter(images::name.eq_all(name))
                    .order_by(images::tag.desc())
                    .select((
                        images::tag,
                        images::creation_time,
                        images::pull_time,
                        images::push_time,
                    )),
                *offset as i64,
            )
            .load::<(String, NaiveDateTime, NaiveDateTime, NaiveDateTime)>(connect),
            Some(limit) => OffsetDsl::offset(
                images::table
                    .filter(images::name.eq_all(name))
                    .order_by(images::tag.desc())
                    .select((
                        images::tag,
                        images::creation_time,
                        images::pull_time,
                        images::push_time,
                    )),
                *offset as i64,
            )
            .limit(*limit as i64)
            .load::<(String, NaiveDateTime, NaiveDateTime, NaiveDateTime)>(connect),
        },
    };
    return result;
}
pub async fn count_size_by_tag_name(
    name: &String,
    tag: &String,
    connect: &DatabaseConnect,
) -> Result<f32, diesel::result::Error> {
    let result: Vec<(u32, u32)> = images::table
        .inner_join(
            config_layers::table.on(images::fslayer_configid.eq_all(config_layers::configid)),
        )
        .inner_join(fs_layers::table.on(config_layers::layersid.eq_all(fs_layers::id)))
        .filter(images::name.eq_all(name))
        .filter(images::tag.eq_all(tag))
        .select((config_layers::layersid, fs_layers::size))
        .load::<(u32, u32)>(connect)?;
    let mut count: u32 = 0;
    for x in result {
        count += x.1;
    }
    return Ok(count as f32 / 1024.0 / 1024.0);
}
pub async fn count_tags_by_name(
    name: &String,
    connect: &DatabaseConnect,
) -> Result<i64, diesel::result::Error> {
    let result = images::table
        .filter(images::name.eq_all(name))
        .count()
        .get_result(connect)?;
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
    use chrono::{NaiveDateTime, TimeZone, Utc};
    use diesel::dsl::max;
    use diesel::result::Error;
    use redis::Commands;
    use uuid::Uuid;

    use crate::dao::images::{
        count_name_by_library, count_size_by_name, count_tags_by_name,
        select_name_limit_offset_by_library, select_timeinfo_by_name,
    };
    use crate::dao::library::{
        count_image_by_library, count_library, count_size_by_libray, count_tag_by_library,
        delete_library_by_name, select_library_limit_offset,
    };
    use crate::v2::authorization::Bearer;
    use crate::{database_pool, Authorization, Calim};

    #[actix_rt::test]
    async fn test1() {
        dotenv::dotenv().ok();
        let pool = database_pool().get().unwrap();
        let string = String::from("linux");
        let x = select_name_limit_offset_by_library(&string, &Some(10), &Some(0), &pool).await;
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
        let string = String::from("linux/ubuntu");
        let x = select_timeinfo_by_name(&string, &pool).await;
        match x {
            Ok(vec) => {
                println!("{:?}", vec);
            }
            Err(_) => {}
        }
    }
    #[actix_rt::test]
    async fn test3() {
        dotenv::dotenv().ok();
        let pool = database_pool().get().unwrap();
        let string = String::from("linux/ubuntu");
        let x = count_size_by_name(&string, &pool).await;
        match x {
            Ok(vec) => {
                println!("{:?}", vec);
            }
            Err(_) => {}
        }
    }
    #[actix_rt::test]
    async fn test4() {
        dotenv::dotenv().ok();
        let pool = database_pool().get().unwrap();
        let string = String::from("linux/centos");
        let x = count_tags_by_name(&string, &pool).await;
        match x {
            Ok(vec) => {
                println!("{:?}", vec);
            }
            Err(_) => {}
        }
    }
    #[actix_rt::test]
    async fn test5() {
        dotenv::dotenv().ok();
        let pool = database_pool().get().unwrap();
        let string = String::from("linux");
        let x = count_name_by_library(&string, &pool).await;
        match x {
            Ok(vec) => {
                println!("{:?}", vec);
            }
            Err(_) => {}
        }
    }
}
