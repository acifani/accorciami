use actix_web::{error, HttpResponse};
use failure::Fail;
use redis::RedisError;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct ErrorResponse {
    status_code: i32,
    error: String,
}

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "Not Found: {}", _0)]
    NotFound(String),

    #[fail(display = "Bad Request: {}", _0)]
    BadRequest(String),

    #[fail(display = "Internal Server Error")]
    InternalServerError,
}

pub enum AccorciamiError {
    URLNotFound,
    EmptyURL,
}

impl error::ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        match *self {
            Error::NotFound(ref message) => HttpResponse::NotFound().json(ErrorResponse {
                status_code: 404,
                error: message.clone(),
            }),
            Error::BadRequest(ref message) => HttpResponse::BadRequest().json(ErrorResponse {
                status_code: 400,
                error: message.clone(),
            }),
            Error::InternalServerError => HttpResponse::InternalServerError().json(ErrorResponse {
                status_code: 500,
                error: "Internal Server Error".to_string(),
            }),
        }
    }
}

impl From<RedisError> for Error {
    fn from(_error: RedisError) -> Self {
        Error::InternalServerError
    }
}

impl From<AccorciamiError> for Error {
    fn from(error: AccorciamiError) -> Self {
        match error {
            AccorciamiError::URLNotFound => Error::NotFound("URL not found".to_string()),
            AccorciamiError::EmptyURL => Error::BadRequest("Empty URL parameter".to_string()),
        }
    }
}
