use crate::{ErrorsInfo, Errs};
use actix_utils::future::{err, ok, Ready};
use actix_web::body::BoxBody;

use crate::v2::content_type::ContentTypeError::ContentTypeInvaild;
use actix_web::dev::Payload;
use actix_web::http::{header, StatusCode};
use actix_web::{FromRequest, HttpRequest, HttpResponse, ResponseError};
use mime::APPLICATION_JSON;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum ContentTypeError {
    ContentTypeInvaild,
}

impl Display for ContentTypeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Request Header no Content-Type")
    }
}

impl ResponseError for ContentTypeError {
    fn status_code(&self) -> StatusCode {
        StatusCode::BAD_REQUEST
    }
    fn error_response(&self) -> HttpResponse<BoxBody> {
        let errs = Errs {
            errors: Some(vec![ErrorsInfo::new(
                "BAD_REQUEST",
                "Content-Type required",
                None,
            )]),
        };
        HttpResponse::build(self.status_code())
            .content_type(APPLICATION_JSON)
            .keep_alive()
            .json(errs)
    }
}
#[derive(Clone, PartialEq, Debug)]
pub struct ContentType(pub String);

impl FromRequest for ContentType {
    type Error = ContentTypeError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        match req.headers().get(header::CONTENT_TYPE) {
            None => err(ContentTypeInvaild),
            Some(content_type) => match content_type.to_str() {
                Ok(t) => ok(ContentType(String::from(t))),
                Err(_) => err(ContentTypeInvaild),
            },
        }
    }
}
