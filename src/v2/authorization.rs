use actix_web::error::ParseError;
use actix_web::http::header::Header as ParseHeader;
use actix_web::http::header::{
    HeaderName, HeaderValue, InvalidHeaderValue, TryIntoHeaderValue, AUTHORIZATION,
};
use actix_web::HttpMessage;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

pub trait Credentials: Sized {
    const PREFIX: &'static str;
    fn encode(&self) -> String;
    fn decode(value: &str) -> Result<Self, ParseError>;
    fn from_header_value(
        prefix: &str,
        headvalue: &HeaderValue,
        decode: fn(&str) -> Result<Self, ParseError>,
    ) -> Result<Self, ParseError> {
        let value = match headvalue.to_str() {
            Ok(k) => k,
            Err(_) => return Err(ParseError::Header),
        };
        let key = value.strip_prefix(prefix).ok_or(ParseError::Header)?;
        let x = key.trim();
        return decode(x);
    }
    fn to_header_value(&self, prefix: &str) -> Result<HeaderValue, InvalidHeaderValue> {
        let encoded = self.encode();
        HeaderValue::try_from(prefix.to_string() + &encoded)
    }
}
#[derive(Clone, PartialEq, Debug)]
pub struct Basic {
    pub username: String,
    pub password: String,
}

impl Credentials for Basic {
    const PREFIX: &'static str = "Basic ";
    fn encode(&self) -> String {
        base64::encode_config(
            format!("{}:{}", self.username, self.password),
            base64::STANDARD,
        )
    }
    fn decode(value: &str) -> Result<Self, ParseError> {
        let vec = match base64::decode(value) {
            Ok(s) => s,
            Err(_) => return Err(ParseError::Header),
        };
        let decoded = match String::from_utf8(vec) {
            Ok(s) => s,
            Err(_) => return Err(ParseError::Header),
        };
        let (username, password) = decoded.split_once(":").unwrap();
        return Ok(Basic {
            username: username.to_string(),
            password: password.to_string(),
        });
    }
}
#[derive(Clone, PartialEq, Debug)]
pub struct Bearer<T>(pub T);
lazy_static! {
    static ref JWT_SECRET: String = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
}
impl<'a, T: Serialize + for<'de> Deserialize<'de>> Credentials for Bearer<T> {
    const PREFIX: &'static str = "Bearer ";
    fn encode(&self) -> String {
        jsonwebtoken::encode(
            &Header::default(),
            &self.0,
            &EncodingKey::from_secret(JWT_SECRET.as_bytes()),
        )
        .unwrap()
    }
    fn decode(value: &str) -> Result<Self, ParseError> {
        let token_data = jsonwebtoken::decode::<T>(
            &value,
            &DecodingKey::from_secret(JWT_SECRET.as_bytes()),
            &Validation::default(),
        )
        .map_err(|_| ParseError::Header)?;
        let clasims: T = token_data.claims;
        Ok(Bearer(clasims))
    }
}
#[derive(Clone, PartialEq, Debug)]
pub struct Authorization<T: Credentials>(pub T);
impl Authorization<Basic> {
    #[allow(dead_code)]
    pub fn basic(username: &str, password: &str) -> Authorization<Basic> {
        let basic = Basic {
            username: username.to_string(),
            password: password.to_string(),
        };
        Authorization(basic)
    }
}

impl<T: Credentials> TryIntoHeaderValue for Authorization<T> {
    type Error = InvalidHeaderValue;
    //encode
    #[inline]
    fn try_into_value(self) -> Result<HeaderValue, Self::Error> {
        T::to_header_value(&self.0, T::PREFIX)
    }
}
impl<T: Credentials> ParseHeader for Authorization<T> {
    fn name() -> HeaderName {
        AUTHORIZATION
    }
    //decode
    fn parse<M: HttpMessage>(msg: &M) -> Result<Self, ParseError> {
        let header_value = match msg.headers().get(&Self::name()) {
            None => return Err(ParseError::Header),
            Some(s) => s,
        };
        // let string = T::encode(T.into);
        return match T::from_header_value(T::PREFIX, header_value, T::decode) {
            Ok(k) => Ok(Authorization(k)),
            Err(k) => Err(k),
        };
    }
}
#[cfg(test)]
mod tests {
    use std::time::{SystemTime, UNIX_EPOCH};

    use actix_web::http::header::{TryIntoHeaderValue, AUTHORIZATION};
    use actix_web::test::TestRequest;
    use actix_web::{web, FromRequest};
    use serde::{Deserialize, Serialize};

    use crate::v2::authorization::{Authorization, Basic, Bearer, Credentials};

    #[actix_rt::test]
    async fn test_basic() {
        let authorization = Authorization::basic("alali", "12345");
        let value = authorization.try_into_value().unwrap();
        println!("{:?}", value);
        let (req, mut pl) = TestRequest::default()
            .insert_header((AUTHORIZATION, value))
            .to_http_parts();
        let header1 = web::Header::<Authorization<Basic>>::from_request(&req, &mut pl)
            .await
            .unwrap();
        let authorization2 = header1.into_inner();
        println!("{:?}", authorization2);
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct CustomClaims {
        username: String,
        passwd: String,
        exp: u64,
    }
    #[actix_rt::test]
    async fn test_bearer_encode() {
        dotenv::dotenv().ok();
        let claims = CustomClaims {
            username: "alali".to_string(),
            passwd: "12345".to_string(),
            exp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs(),
        };
        let bearer = Bearer(claims);
        let encoded = bearer.encode();
        let beare = Bearer::<CustomClaims>::decode(encoded.as_str()).unwrap();
        println!("{}", encoded);
        println!("{:?}", beare.0);
    }
    #[actix_rt::test]
    async fn test_bearer() {
        dotenv::dotenv().ok();
        let claims = CustomClaims {
            username: "alali".to_string(),
            passwd: "12345".to_string(),
            exp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs(),
        };
        let header_value = Authorization(Bearer(claims)).try_into_value().unwrap();
        println!("{:?}", header_value);
        let (req, mut pl) = TestRequest::default()
            .insert_header((AUTHORIZATION, header_value))
            .to_http_parts();
        let web::Header(Authorization(Bearer(calim))) =
            web::Header::<Authorization<Bearer<CustomClaims>>>::from_request(&req, &mut pl)
                .await
                .unwrap();
        println!("{:?}", calim);
    }
}
