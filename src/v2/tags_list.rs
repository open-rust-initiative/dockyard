use crate::dao::images::select_tags_by_name_from_last_limit_n;

use crate::{Calim, DatabasePool};
use actix_web::{get, web, HttpResponse, Responder};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TagsList {
    name: String,
    tags: Vec<String>,
}
#[derive(Deserialize, Debug)]
pub struct TagInfo {
    pub n: Option<usize>,
    pub last: Option<String>,
}
#[get("/{name:.*}/tags/list")]
pub(crate) async fn get_tags_list(
    _: Calim,
    params: web::Path<String>,
    pool: web::Data<DatabasePool>,
    taginfo: web::Query<TagInfo>,
) -> impl Responder {
    let name = params.into_inner();
    let info = taginfo.into_inner();
    let connection = pool.get().unwrap();
    match select_tags_by_name_from_last_limit_n(&name, info.last, info.n, &connection).await {
        Ok(tags) => {
            let tags_list = TagsList { name, tags };
            HttpResponse::Ok().json(tags_list)
        }
        Err(_) => HttpResponse::NotFound().finish(),
    }
}
