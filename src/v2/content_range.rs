use crate::{ErrorsInfo, Errs};
use actix_utils::future::{err, ok, Ready};
use actix_web::body::BoxBody;

use crate::v2::content_range::ContentRangeError::ContentRangeInvaild;
use actix_web::dev::Payload;
use actix_web::http::{header, StatusCode};
use actix_web::{FromRequest, HttpRequest, HttpResponse, ResponseError};
use mime::APPLICATION_JSON;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum ContentRangeError {
    ContentRangeInvaild,
}

impl Display for ContentRangeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Request Header no Content-Type")
    }
}

impl ResponseError for ContentRangeError {
    fn status_code(&self) -> StatusCode {
        StatusCode::BAD_REQUEST
    }
    fn error_response(&self) -> HttpResponse<BoxBody> {
        let errs = Errs {
            errors: Some(vec![ErrorsInfo::new(
                "BAD_REQUEST",
                "Content-Range required",
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
pub struct ContentRange(pub Option<(usize, usize)>);

impl FromRequest for ContentRange {
    type Error = ContentRangeError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        match req.headers().get(header::CONTENT_RANGE) {
            None => ok(ContentRange(None)),
            Some(content_range) => match content_range.to_str() {
                Ok(t) => {
                    match t.split_once("-") {
                        None => {}
                        Some((l, r)) => match l.parse::<usize>() {
                            Ok(l) => match r.parse::<usize>() {
                                Ok(r) => return ok(ContentRange(Some((l, r)))),
                                Err(_) => {}
                            },
                            Err(_) => {}
                        },
                    }
                    err(ContentRangeInvaild)
                }
                Err(_) => err(ContentRangeInvaild),
            },
        }
    }
}
