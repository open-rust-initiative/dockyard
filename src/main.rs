#[macro_use]
extern crate diesel;

use actix_web::http::header::HeaderValue;
use actix_web::{middleware, web, App, HttpServer};
use diesel::r2d2::ConnectionManager;
use diesel::{r2d2, MysqlConnection};
use lazy_static::lazy_static;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{fmt, EnvFilter};

use v2::auth::Calim;
use v2::authorization::{Authorization, Basic, Credentials};

use crate::dao::user::verify_user;
use crate::r#static::static_files;
use crate::utils::{database_pool, getservicetoken, redis_pool, supprt_ssl};
use crate::v2::auth::{generate_token, SERVERHOST};
use crate::v2::errs::{ErrorsInfo, Errs};

mod dao;
mod schema;
mod r#static;
mod ui;
mod utils;
mod v2;

pub type DatabasePool = diesel::r2d2::Pool<ConnectionManager<MysqlConnection>>;
pub type RedisPool = r2d2::Pool<redis::Client>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();
    let database_pool = database_pool();
    let redis_pool = redis_pool().unwrap();
    let server = HttpServer::new(move || {
        let app = App::new()
            .wrap(
                middleware::DefaultHeaders::new()
                    .add(("Docker-Distribution-Api-Version", "registry/2.0")),
            )
            .service(web::scope("/v2").configure(v2::routes::init))
            .service(getservicetoken)
            .service(web::scope("/ui").configure(ui::routes::init))
            .service(static_files())
            .app_data(web::Data::new(database_pool.clone()))
            .app_data(web::Data::new(redis_pool.clone()));
        app
    });
    let string = SERVERHOST.to_string();
    println!("{}", string);
    let (prefix, _) = string.split_once("://").unwrap_or(("http", "dockyard"));
    if prefix == "https" {
        let _builder = supprt_ssl();
        server.bind_openssl("0.0.0.0:4000", _builder)?.run().await
    } else {
        server.bind(("0.0.0.0", 4000))?.run().await
    }
}

lazy_static! {
    static ref TOKEN_EXPIRES_IN: u64 = std::env::var("TOKEN_EXPIRES_IN")
        .expect("TOKEN_EXPIRES_IN must be set")
        .parse::<u64>()
        .expect("TOKEN_EXPIRES_IN maybe not u64");
}
