use crate::dao::DatabaseConnect;
use crate::diesel::ExpressionMethods;
use crate::generate_token;
use crate::schema::user;
use diesel::result::Error;
use diesel::{EqAll, NullableExpressionMethods, QueryDsl, RunQueryDsl};
use serde::{Deserialize, Serialize};

#[derive(Insertable)]
#[table_name = "user"]
pub struct NewUser<'a> {
    pub username: &'a String,
    pub name: &'a String,
    pub password: &'a String,
    pub admin: bool,
    pub email: &'a String,
    pub comment: &'a String,
}
pub async fn insert_user<'a>(
    username: &String,
    name: &String,
    password_encoded: &String,
    admin: bool,
    email: &String,
    comment: &String,
    connect: &DatabaseConnect,
) -> bool {
    let user = NewUser {
        username,
        name,
        password: password_encoded,
        admin,
        email,
        comment,
    };
    match diesel::insert_into(user::table)
        .values(&user)
        .execute(connect)
    {
        Ok(num) => {
            if num == 1 {
                return true;
            }
            return false;
        }
        Err(_) => false,
    }
}
pub async fn verify_user(
    username: &String,
    password_encoded: &String,
    connect: &DatabaseConnect,
) -> bool {
    return match user::table
        .filter(user::username.eq_all(username))
        .select(user::password)
        .load::<String>(connect)
    {
        Ok(passwds) => passwds.contains(password_encoded),
        Err(_) => false,
    };
}
#[derive(Serialize, Deserialize)]
pub struct UserInfo {
    pub username: String,
    pub name: String,
    pub admin: bool,
    pub email: String,
    pub token: String,
    pub comment: Option<String>,
}
pub async fn select_userinfo_by_username_password(
    username: &String,
    password_encoded: &String,
    connect: &DatabaseConnect,
) -> Result<UserInfo, Error> {
    let username_clone = username.clone();
    match user::table
        .filter(user::username.eq(&username))
        .select((
            user::password,
            user::name,
            user::admin,
            user::email,
            user::comment.nullable(),
        ))
        .load::<(String, String, bool, String, Option<String>)>(connect)
    {
        Ok(vec) => {
            if vec.len() == 1 {
                let (password, name, admin, email, comment) = vec.get(0).unwrap().clone();
                if password == *password_encoded {
                    let token = generate_token(username.clone());
                    let info = UserInfo {
                        username: username_clone,
                        name,
                        admin,
                        email,
                        token,
                        comment,
                    };
                    return Ok(info);
                }
            }
        }
        Err(_) => {}
    };
    return Err(Error::NotFound);
}
pub async fn update_userinfo_by_username(
    username: &String,
    name: &String,
    email: &String,
    comment: &String,
    connect: &DatabaseConnect,
) -> bool {
    match diesel::update(user::table.filter(user::username.eq_all(username)))
        .set((
            user::name.eq_all(name),
            user::email.eq_all(email),
            user::comment.eq_all(comment),
        ))
        .execute(connect)
    {
        Ok(s) => s == 1,
        Err(_) => false,
    }
}
pub async fn update_passwd_by_username(
    username: &String,
    password: &String,
    new_password: &String,
    connect: &DatabaseConnect,
) -> bool {
    return match diesel::update(
        user::table
            .filter(user::username.eq_all(username))
            .filter(user::password.eq_all(password)),
    )
    .set(user::password.eq_all(new_password))
    .execute(connect)
    {
        Ok(s) => s == 1,
        Err(_) => false,
    };
}
