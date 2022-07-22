use crate::v2::errs::ErrsInto;
use crate::v2::errs::ErrsInto::FileCreateFailed;
use crate::{
    generate_token, verify_user, Authorization, Basic, DatabasePool, ErrorsInfo, Errs, RedisPool,
};
use actix_web::http::header::{HeaderValue, CONTENT_SECURITY_POLICY, X_FRAME_OPTIONS};
use actix_web::{get, web, HttpResponse, Responder};
use chrono::{SecondsFormat, Utc};
use diesel::r2d2::ConnectionManager;
use diesel::{r2d2, MysqlConnection};
use lazy_static::lazy_static;
use openssl::ssl::{SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod};
use serde::{Deserialize, Serialize};

use std::time::Duration;
use tokio::fs::{File, OpenOptions};

use tracing::{error, info, span, Level};

#[get("/service/token")]
pub async fn getservicetoken(
    web::Header(Authorization(basic)): web::Header<Authorization<Basic>>,
    pool: web::Data<DatabasePool>,
) -> impl Responder {
    let user_str = basic.username.as_str();
    let span = span!(Level::INFO, "generate_token", username = user_str);
    let connection = &pool.get().unwrap();
    let _entered = span.enter();
    let password_encoded =
        sha256::digest(user_str.to_string() + &(sha256::digest(&basic.password)));
    return match verify_user(&basic.username, &password_encoded, connection).await {
        true => {
            info!("database verification succeeded {:?}", &basic);
            let token = generate_token(basic.username);
            let token1 = GetToken {
                token,
                access_token: "".to_string(),
                expires_in: 1800,
                issued_at: Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true),
            };
            info!("token generate successed {:?}", token1);
            HttpResponse::Ok()
                .append_header((X_FRAME_OPTIONS, HeaderValue::from_str("DENY").unwrap()))
                .append_header((
                    CONTENT_SECURITY_POLICY,
                    HeaderValue::from_str("frame-ancestors 'none'").unwrap(),
                ))
                .json(token1)
        }
        false => {
            error!("database verification failed {:?}", basic);
            let errros = Errs {
                errors: Some(vec![ErrorsInfo::new(
                    "UNAUTHORIZED",
                    "username or password error",
                    None,
                )]),
            };
            HttpResponse::Unauthorized().json(errros)
        }
    };
}
#[derive(Serialize, Deserialize, Debug)]
struct GetToken {
    token: String,
    access_token: String,
    expires_in: u64,
    issued_at: String,
}
pub fn database_pool() -> DatabasePool {
    let database_url = std::env::var("DATABASE_URL").expect("Database url must be set");
    let poolmaxsize = std::env::var("DATABASE_POOL_MAX_SIZE").unwrap_or("15".to_string());
    let poolmaxsize = poolmaxsize.parse::<u32>().expect(
        "the maximum number of connections managed by the database pool set fail,maybe is not usize",
    );
    let connection_timeout = std::env::var("DATABASE_POOL_CONNECTION_TIMEOUT")
        .unwrap_or("30".to_string())
        .parse::<u64>()
        .expect("the connection timeout used by the database pool set fail,maybe is not u64 ");
    match DatabasePool::builder()
        .max_size(poolmaxsize)
        .connection_timeout(Duration::from_secs(connection_timeout))
        .build(ConnectionManager::<MysqlConnection>::new(database_url))
    {
        Ok(pool) => return pool,
        Err(_) => {
            panic!("create database pool failed")
        }
    }
}
pub fn redis_pool() -> Option<RedisPool> {
    let redis_url = std::env::var("REDIS_URL").expect("Redis url must be set");
    match redis::Client::open(redis_url) {
        Ok(cli) => {
            let poolmaxsie = std::env::var("REDIS_POOL_MAX_SIZE").unwrap_or("15".to_string()).parse::<u32>().expect(
                "the maximum number of connections managed by the redis pool set fail,maybe is not usize"
            );
            let connection_timeout = std::env::var("REDIS_POOL_CONNECTION_TIMEOUT")
                .unwrap_or("30".to_string())
                .parse::<u64>()
                .expect("the connection timeout used by the redis pool set fail,maybe is not u64 ");
            let duration = Duration::from_secs(connection_timeout);
            match r2d2::Pool::builder()
                .max_size(poolmaxsie)
                .connection_timeout(duration)
                .build(cli)
            {
                Ok(pool) => return Some(pool),
                Err(_) => {
                    panic!("create redis pool failed")
                }
            };
        }
        Err(_) => {
            panic!("connect redis failed")
        }
    }
}
pub fn supprt_ssl() -> SslAcceptorBuilder {
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();
    return builder;
}
lazy_static! {
    static ref BASE_STORGE_URL: String =
        std::env::var("BASE_STORGE_URL").expect("BASE_STORGE_URL must be set");
    static ref SINGLE_APP: String = std::env::var("SINGLE_APP").expect("SINGLE_APP must be set");
}
pub async fn create_file(name: &str, file_name: &String) -> Result<File, ErrsInto> {
    let prefix = BASE_STORGE_URL.to_string() + name;
    match tokio::fs::create_dir_all(prefix.as_str()).await {
        Ok(_) => {
            let file = prefix + "/" + file_name;
            match OpenOptions::new()
                .append(true)
                .create(true)
                .open(file.as_str())
                .await
            {
                Ok(file) => return Ok(file),
                Err(_) => {}
            };
        }
        Err(_) => {}
    };
    return Err(FileCreateFailed);
}

pub fn create_file_url(name: &str, file_name: &String) -> String {
    BASE_STORGE_URL.to_string() + name + "/" + file_name
}
#[cfg(test)]
mod tests {
    use crate::utils::create_file;
    use crate::v2::errs::ErrsInto;
    use tokio::fs::File;
    use tokio::io::AsyncWriteExt;
    #[actix_rt::test]
    async fn test_create_file() {
        dotenv::dotenv().ok();
        let result = create_file("log-pro", &("1".to_string())).await;
        match result {
            Ok(mut file) => {
                let result1 = file.write_all(b"aaaa").await;
                file.flush();
                assert_eq!(file.metadata().await.unwrap().len(), 8);
            }
            Err(_) => {}
        }
    }
}
