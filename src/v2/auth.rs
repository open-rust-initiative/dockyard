use std::fmt::{Debug, Display, Formatter};
use std::ops::Add;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use actix_utils::future::{err, ok, Ready};
use actix_web::body::BoxBody;
use actix_web::dev::Payload;
use actix_web::http::header::Header;
use actix_web::http::{header, StatusCode};
use actix_web::{FromRequest, HttpRequest, HttpResponse, ResponseError};
use lazy_static::lazy_static;
use mime::APPLICATION_JSON;
use serde::{Deserialize, Serialize};
use tracing::{error, info, span, warn, Level};
use uuid::Uuid;

use crate::v2::authorization::Bearer;
use crate::v2::errs::{ErrorsInfo, Errs};
use crate::{Authorization, Credentials, HeaderValue, TOKEN_EXPIRES_IN};

lazy_static! {
    pub static ref SERVERHOST: String =
        std::env::var("SERVERHOST").expect("SERVERHOST must be set");
    static ref TOKEN_VALIDITY_INTERVAL: u64 = std::env::var("TOKEN_VALIDITY_INTERVAL")
        .unwrap_or("1800".to_string())
        .parse::<u64>()
        .unwrap();
    static ref WWW_AUTH: String = "Bearer realm=\"".to_string()
        + &SERVERHOST
        + &"/service/token\",service=\"dockeryard\"".to_string();
}

#[derive(Debug)]
pub enum AuthError {
    NoAuthHeader,
    ExpAuthBearer,
}
impl Display for AuthError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthError::NoAuthHeader => {
                write!(f, "Request Header no auth")
            }
            AuthError::ExpAuthBearer => {
                write!(f, "Auth Bearer is exp")
            }
        }
    }
}

impl ResponseError for AuthError {
    fn status_code(&self) -> StatusCode {
        StatusCode::UNAUTHORIZED
    }
    fn error_response(&self) -> HttpResponse<BoxBody> {
        let errros = match self {
            AuthError::NoAuthHeader => Errs {
                errors: Some(vec![ErrorsInfo::new(
                    "UNAUTHORIZED",
                    "authentication required",
                    None,
                )]),
            },
            AuthError::ExpAuthBearer => Errs {
                errors: Some(vec![ErrorsInfo::new(
                    "UNAUTHORIZED",
                    "auth bearer was exp",
                    None,
                )]),
            },
        };
        HttpResponse::build(self.status_code())
            .append_header((
                header::WWW_AUTHENTICATE,
                HeaderValue::from_static(WWW_AUTH.as_str()),
            ))
            .content_type(APPLICATION_JSON)
            .keep_alive()
            .json(errros)
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Calim {
    pub(crate) iss: String,
    pub(crate) sub: String,
    pub(crate) aud: String,
    pub(crate) exp: u64,
    pub(crate) nbf: u64,
    pub(crate) iat: u64,
    pub(crate) jti: String,
}

impl FromRequest for Calim {
    type Error = AuthError;
    type Future = Ready<Result<Self, Self::Error>>;
    #[inline]
    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        return match Authorization::<Bearer<Calim>>::parse(req) {
            Ok(Authorization(Bearer(calim))) => {
                let exp = calim.exp;
                let now = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
                let span = span!(Level::TRACE, "jwt_check", sub = calim.sub.as_str());
                let _entered = span.enter();
                if exp < now {
                    warn!("this token has been expired");
                    return err(AuthError::ExpAuthBearer);
                }
                info!("this token is validiy");
                ok(calim)
            }
            Err(_) => {
                error!("Header AUTHORIZATION don't exist");
                err(AuthError::NoAuthHeader)
            }
        };
    }
}
pub fn generate_token(username: String) -> String {
    let span = span!(Level::TRACE, "token_generate", username = username.as_str());
    let _entered = span.enter();
    let calim = Calim {
        iss: "dockyard-token-issuer".to_string(),
        sub: username,
        aud: "dockyard-registry".to_string(),
        exp: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .add(Duration::from_secs(*TOKEN_EXPIRES_IN))
            .as_secs(),
        nbf: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        iat: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        jti: Uuid::new_v4().to_string(),
    };

    let token = Bearer(calim).encode();
    info!("generate_success");
    return token;
}
#[cfg(test)]
mod tests {
    use std::ops::Add;
    use std::thread::sleep;
    use std::time::{Duration, SystemTime, UNIX_EPOCH};

    use actix_web::http::header::{TryIntoHeaderValue, AUTHORIZATION};
    use actix_web::test::TestRequest;
    use actix_web::{web, FromRequest};
    use uuid::Uuid;

    use crate::v2::authorization::Bearer;
    use crate::{Authorization, Calim};

    #[test]
    fn test1() {
        let i = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let i2 = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .add(Duration::from_secs(10))
            .as_secs();
        println!("{}", i);
        let i1 = i.add(10);
        println!("{}", i1);
        println!("{}", i2);
        sleep(Duration::from_secs(10));
        println!(
            "{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
        );
    }
    #[actix_rt::test]
    async fn test2() {
        dotenv::dotenv().ok();
        let calim = Calim {
            iss: "dockyard-token-issuer".to_string(),
            sub: "csm".to_string(),
            aud: "dockyard-registry".to_string(),
            exp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .add(Duration::from_secs(30 * 60))
                .as_secs(),
            nbf: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            iat: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            jti: Uuid::new_v4().to_string(),
        };
        let value = Authorization(Bearer(calim)).try_into_value().unwrap();
        println!("{:?}", value);
        let (req, mut pl) = TestRequest::default()
            .insert_header((AUTHORIZATION, value))
            .to_http_parts();
        let web::Header(Authorization(Bearer(calim))) =
            web::Header::<Authorization<Bearer<Calim>>>::from_request(&req, &mut pl)
                .await
                .unwrap();
        println!("{:?}", calim);
    }
}
