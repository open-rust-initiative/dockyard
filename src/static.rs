use actix_files::{Files, NamedFile};
use actix_web::Responder;
use actix_web::{get, web, Scope};
pub fn static_files() -> Scope {
    return web::scope("")
        .default_service(
            Files::new("", "./static/")
                .index_file("index.html")
                .use_last_modified(true),
        )
        .service(login)
        .service(library)
        .service(image)
        .service(home);
}
#[get("/login")]
pub async fn login() -> impl Responder {
    NamedFile::open_async("./static/index.html").await
}
#[get("/library")]
pub async fn library() -> impl Responder {
    NamedFile::open_async("./static/index.html").await
}
#[get("/image")]
pub async fn image() -> impl Responder {
    NamedFile::open_async("./static/index.html").await
}
#[get("/home")]
pub async fn home() -> impl Responder {
    NamedFile::open_async("./static/index.html").await
}
