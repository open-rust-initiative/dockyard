use actix_web::web::Query;
use actix_web::{delete, get, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use tracing::{span, Level};

use crate::dao::library::{
    count_image_by_library, count_library, count_size_by_libray, count_tag_by_library,
    delete_library_by_name, select_library_limit_offset,
};
use crate::{Calim, DatabasePool};

#[derive(Serialize)]
pub struct Library {
    name: String,
    size: f32,
    images_count: usize,
    tags_count: usize,
}
#[derive(Deserialize)]
pub struct LibraryInfo {
    limit: Option<usize>,
    offset: Option<usize>,
}
#[get("/library/list")]
pub async fn get_library_list(
    _: Calim,
    library: Query<LibraryInfo>,
    pool: web::Data<DatabasePool>,
) -> impl Responder {
    let span = span!(Level::INFO, "library_list",);
    let _entered = span.enter();
    let connection = pool.get().unwrap();
    let mut library_list: Vec<Library> = vec![];
    match select_library_limit_offset(&library.limit, &library.offset, &connection).await {
        Ok(library) => {
            for name in library {
                let size: f32 = match count_size_by_libray(&name, &connection).await {
                    Ok(s) => s,
                    Err(_) => 0.0,
                };
                let images_count = match count_image_by_library(&name, &connection).await {
                    Ok(s) => s,
                    Err(_) => 0,
                };
                let tags_count = match count_tag_by_library(&name, &connection).await {
                    Ok(s) => s,
                    Err(_) => 0,
                };
                library_list.push(Library {
                    name,
                    size,
                    images_count,
                    tags_count,
                })
            }
            HttpResponse::Ok().json(library_list)
        }
        Err(_) => HttpResponse::NotFound().finish(),
    }
}
#[derive(Deserialize)]
pub struct DelQuery {
    name: String,
}
#[delete("/library/remove")]
pub async fn del_library(
    _: Calim,
    name: Query<DelQuery>,
    pool: web::Data<DatabasePool>,
) -> impl Responder {
    let span = span!(Level::INFO, "library_list",);
    let _entered = span.enter();
    let query = name.into_inner().name;
    let connection = pool.get().unwrap();
    return match delete_library_by_name(&query, &connection).await {
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::ExpectationFailed(),
    };
}
#[derive(Serialize)]
pub struct LibraryCount {
    count: i64,
}
#[get("/library/count")]
pub async fn get_library_count(_: Calim, pool: web::Data<DatabasePool>) -> impl Responder {
    let span = span!(Level::INFO, "library_count",);
    let _entered = span.enter();
    let connection = pool.get().unwrap();
    match count_library(&connection).await {
        Ok(s) => HttpResponse::Ok().json(LibraryCount { count: s }),
        Err(_) => return HttpResponse::NotFound().finish(),
    }
}
