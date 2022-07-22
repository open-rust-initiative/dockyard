use crate::dao::user::{
    insert_user, select_userinfo_by_username_password, update_passwd_by_username,
    update_userinfo_by_username,
};

use crate::{generate_token, Calim, DatabasePool};
use actix_web::web::Json;
use actix_web::{post, put, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use tracing::{error, info, span, Level};

#[derive(Deserialize, Debug)]
pub struct Register {
    username: String,
    password: String,
    email: String,
    name: String,
    comment: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterRes {
    token: String,
}
#[post("/user/register")]
pub(crate) async fn user_register(
    data: Json<Register>,
    pool: web::Data<DatabasePool>,
) -> impl Responder {
    let span = span!(
        Level::INFO,
        "user_register",
        username = data.username.as_str(),
    );
    let _entered = span.enter();
    let connection = pool.get().unwrap();
    return match insert_user(
        &data.username,
        &data.name,
        &data.password,
        false,
        &data.email,
        &data.comment,
        &connection,
    )
    .await
    {
        true => {
            let token = generate_token(data.username.clone());
            info!("register successed");
            HttpResponse::Ok().json(RegisterRes { token })
        }
        false => {
            error!("register failed");
            HttpResponse::ExpectationFailed().finish()
        }
    };
}
#[derive(Deserialize, Debug)]
pub struct Login {
    username: String,
    password: String,
}
#[post("/user/login")]
pub async fn user_login(data: Json<Login>, pool: web::Data<DatabasePool>) -> impl Responder {
    let span = span!(Level::INFO, "user_login", username = data.username.as_str());
    let _entered = span.enter();
    let connection = pool.get().unwrap();
    return match select_userinfo_by_username_password(&data.username, &data.password, &connection)
        .await
    {
        Ok(user) => {
            info!("login_success");
            HttpResponse::Ok().json(user)
        }
        Err(_) => {
            error!("login_failure");
            HttpResponse::Unauthorized().finish()
        }
    };
}
#[derive(Deserialize, Debug)]
pub struct UserUpdate {
    username: String,
    name: String,
    email: String,
    comment: String,
}
#[put("/user/update_info")]
pub async fn user_update_info(
    _: Calim,
    data: Json<UserUpdate>,
    pool: web::Data<DatabasePool>,
) -> impl Responder {
    let span = span!(
        Level::INFO,
        "user_update_info",
        username = data.username.as_str()
    );
    let _entered = span.enter();
    let connection = pool.get().unwrap();
    match update_userinfo_by_username(
        &data.username,
        &data.name,
        &data.email,
        &data.comment,
        &connection,
    )
    .await
    {
        true => HttpResponse::Ok().finish(),
        false => HttpResponse::NotModified().finish(),
    }
}
#[derive(Deserialize, Debug)]
pub struct PasswdUpdate {
    username: String,
    password: String,
    new_password: String,
}
#[put("/user/update_passwd")]
pub async fn user_update_passwd(
    _: Calim,
    data: Json<PasswdUpdate>,
    pool: web::Data<DatabasePool>,
) -> impl Responder {
    let span = span!(
        Level::INFO,
        "user_update_passwd",
        username = data.username.as_str()
    );
    let _entered = span.enter();
    let connection = pool.get().unwrap();
    match update_passwd_by_username(
        &data.username,
        &data.password,
        &data.new_password,
        &connection,
    )
    .await
    {
        true => HttpResponse::Ok().finish(),
        false => HttpResponse::NotModified().finish(),
    }
}
