use crate::dao::images::{
    count_name_by_library, count_size_by_name, count_size_by_tag_name, count_tags_by_name,
    select_name_limit_offset_by_library, select_tags_limit_offset_by_name, select_timeinfo_by_name,
    TimeInfo,
};
use crate::dao::library::delete_all_imageitem_by_name;

use crate::ui::tag::TagsItem;
use crate::{Calim, DatabasePool};
use actix_web::web::Query;
use actix_web::{delete, get, web, HttpResponse, Responder};

use serde::{Deserialize, Serialize};
use tracing::{span, Level};

#[derive(Serialize)]
pub struct ImageItem {
    name: String,
    size: f32,
    timeinfo: TimeInfo,
    tags_count: i64,
    tags: Vec<TagsItem>,
}

#[derive(Deserialize)]
pub struct ImageInfo {
    library: String,
    limit: Option<usize>,
    offset: Option<usize>,
}
#[get("/image/list")]
pub async fn get_image_list(
    _: Calim,
    info: Query<ImageInfo>,
    pool: web::Data<DatabasePool>,
) -> impl Responder {
    let span = span!(Level::INFO, "image_list",);
    let _entered = span.enter();
    let connection = pool.get().unwrap();
    let info = info.into_inner();
    let mut images_list: Vec<ImageItem> = vec![];
    match select_name_limit_offset_by_library(&info.library, &info.limit, &info.offset, &connection)
        .await
    {
        Ok(names) => {
            for name in names {
                let timeinfo = select_timeinfo_by_name(&name, &connection).await.unwrap();
                let size = count_size_by_name(&name, &connection).await.unwrap();
                let tags_count = count_tags_by_name(&name, &connection).await.unwrap();
                let mut tags_list: Vec<TagsItem> = vec![];
                let tags = select_tags_limit_offset_by_name(&name, &None, &None, &connection)
                    .await
                    .unwrap();
                for (tag, create, pull, push) in tags {
                    let size = count_size_by_tag_name(&name, &tag, &connection)
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
                images_list.push(ImageItem {
                    name,
                    size,
                    timeinfo,
                    tags_count,
                    tags: tags_list,
                })
            }
            return HttpResponse::Ok().json(images_list);
        }
        Err(_) => {}
    }
    return HttpResponse::NotFound().finish();
}

#[derive(Deserialize)]
pub struct DelNameQuery {
    name: String,
}
#[delete("/image/remove")]
pub async fn del_image(
    _: Calim,
    name: Query<DelNameQuery>,
    pool: web::Data<DatabasePool>,
) -> impl Responder {
    let span = span!(Level::INFO, "image_remove",);
    let _entered = span.enter();
    let query = name.into_inner().name;
    let connection = pool.get().unwrap();
    return match delete_all_imageitem_by_name(&query, &connection).await {
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::ExpectationFailed(),
    };
}
#[derive(Serialize)]
struct ImageCount {
    count: usize,
}
#[derive(Deserialize)]
pub struct ImageCountInfo {
    library: String,
}
#[get("/image/count")]
pub async fn get_image_count(
    _: Calim,
    info: web::Query<ImageCountInfo>,
    pool: web::Data<DatabasePool>,
) -> impl Responder {
    let span = span!(Level::INFO, "library_count",);
    let _entered = span.enter();
    let connection = pool.get().unwrap();
    return match count_name_by_library(&info.library, &connection).await {
        Ok(s) => HttpResponse::Ok().json(ImageCount { count: s }),
        Err(_) => return HttpResponse::NotFound().finish(),
    };
}
