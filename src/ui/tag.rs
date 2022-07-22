use crate::dao::images::{
    count_size_by_tag_name, count_tags_by_name, select_tags_limit_offset_by_name,
};
use crate::dao::library::delete_imageitem_by_name_tag;

use crate::{Calim, DatabasePool};
use actix_web::web::Query;
use actix_web::{delete, get, web, HttpResponse, Responder};

use serde::{Deserialize, Serialize};
use tracing::{span, Level};

#[derive(Serialize)]
pub struct TagsItem {
    pub(crate) tag: String,
    pub(crate) size: f32,
    pub(crate) create_time: String,
    pub(crate) pull_time: String,
    pub(crate) push_time: String,
}

#[derive(Deserialize)]
pub struct TagInfo {
    name: String,
    limit: Option<usize>,
    offset: Option<usize>,
}
#[get("/tag/list")]
pub async fn get_tag_list(
    _: Calim,
    info: Query<TagInfo>,
    pool: web::Data<DatabasePool>,
) -> impl Responder {
    let span = span!(Level::INFO, "tag_list",);
    let _entered = span.enter();
    let connection = pool.get().unwrap();
    let info = info.into_inner();
    let mut tags_list: Vec<TagsItem> = vec![];
    match select_tags_limit_offset_by_name(&info.name, &info.limit, &info.offset, &connection).await
    {
        Ok(s) => {
            for (tag, create, pull, push) in s {
                let size = count_size_by_tag_name(&info.name, &tag, &connection)
                    .await
                    .unwrap();
                tags_list.push(TagsItem {
                    tag,
                    size,
                    create_time: create.to_string(),
                    pull_time: pull.to_string(),
                    push_time: push.to_string(),
                })
            }
            return HttpResponse::Ok().json(tags_list);
        }
        Err(_) => {}
    }
    return HttpResponse::Ok().finish();
}

#[derive(Deserialize)]
pub struct DelTagQuery {
    name: String,
    tag: String,
}
#[delete("/tag/remove")]
pub async fn del_tag(
    _: Calim,
    name: Query<DelTagQuery>,
    pool: web::Data<DatabasePool>,
) -> impl Responder {
    let span = span!(Level::INFO, "tag_remove",);
    let _entered = span.enter();
    let query = name.into_inner();
    let connection = pool.get().unwrap();
    return match delete_imageitem_by_name_tag(&query.name, &query.tag, &connection).await {
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::ExpectationFailed(),
    };
}
#[derive(Serialize)]
struct TagsCount {
    count: i64,
}
#[derive(Deserialize)]
pub struct TagsCountInfo {
    name: String,
}
#[get("/tag/count")]
pub async fn get_tag_count(
    name: web::Query<TagsCountInfo>,
    pool: web::Data<DatabasePool>,
) -> impl Responder {
    let span = span!(Level::INFO, "tags_count",);
    let _entered = span.enter();
    let connection = pool.get().unwrap();
    match count_tags_by_name(&name.name, &connection).await {
        Ok(count) => HttpResponse::Ok().json(TagsCount { count }),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}
