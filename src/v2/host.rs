use crate::v2::errs::{ErrorsInfo, Errs};
use actix_utils::future::{err, ok, Ready};
use actix_web::body::BoxBody;
use actix_web::dev::Payload;
use actix_web::http::{header, StatusCode};
use actix_web::{FromRequest, HttpRequest, HttpResponse, ResponseError};
use mime::APPLICATION_JSON;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub enum HostError {
    HostInvaild,
}

impl Display for HostError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Request Header no Host")
    }
}

impl ResponseError for HostError {
    fn status_code(&self) -> StatusCode {
        StatusCode::BAD_REQUEST
    }
    fn error_response(&self) -> HttpResponse<BoxBody> {
        let errs = Errs {
            errors: Some(vec![ErrorsInfo::new("BAD_REQUEST", "host required", None)]),
        };
        HttpResponse::build(self.status_code())
            .content_type(APPLICATION_JSON)
            .keep_alive()
            .json(errs)
    }
}
#[derive(Clone, PartialEq, Debug)]
pub struct Host(pub String);
impl FromRequest for Host {
    type Error = HostError;
    type Future = Ready<Result<Self, Self::Error>>;
    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        match req.headers().get(header::HOST) {
            None => err(HostError::HostInvaild),
            Some(host) => match host.to_str() {
                Ok(a) => ok(Host(String::from(a))),
                Err(_) => err(HostError::HostInvaild),
            },
        }
    }
}
