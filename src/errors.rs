use std::string::FromUtf8Error;
use axum::{
    body::Body,
    http::{Response, StatusCode},
    response::IntoResponse,
};
use redis::RedisError;
use serde::Serialize;
use serde_json::json;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Clone, Debug, Serialize)]
pub enum Error {
    LoginFail,
    DatabaseError(String),
    DataExist(String),
    DataNotAvaliable(String),
    TokenError(String),
    DecodeError(String),
    StringError(String),
    UserUnauthorized(String),
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}

impl From<surrealdb::Error> for Error {
    fn from(error: surrealdb::Error) -> Self {
        Error::DatabaseError(error.to_string())
    }
}

impl From<jsonwebtoken::errors::Error> for Error {
    fn from(error: jsonwebtoken::errors::Error) -> Self {
        Error::TokenError(error.to_string())
    }
}

impl From<base64::DecodeError> for Error {
    fn from(error: base64::DecodeError) -> Self {
        Error::DecodeError(error.to_string())
    }
}

impl From<FromUtf8Error> for Error {
    fn from(error: FromUtf8Error) -> Self {
        Error::StringError(error.to_string())
    }
}

impl From<RedisError> for Error {
    fn from(error: RedisError) -> Self {
        Error::DatabaseError(error.to_string())
    }
}

impl From<uuid::Error> for Error {
    fn from(error: uuid::Error) -> Self {
        Error::StringError(error.to_string())
    }
}

impl From<argon2::password_hash::Error> for Error {
    fn from(error: argon2::password_hash::Error) -> Self {
        Error::DatabaseError(error.to_string())
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response<Body> {
        let (status, error_message) = match &self {
            Error::LoginFail => (StatusCode::UNAUTHORIZED, "Login failed".to_string()),
            Error::DatabaseError(error) => (StatusCode::INTERNAL_SERVER_ERROR, format!("There was a problem with the database: {}", error)),
            Error::DataExist(id) => (StatusCode::NOT_ACCEPTABLE, format!("{} already registered", id)),
            Error::DataNotAvaliable(id) => (StatusCode::NOT_FOUND, format!("{} Not Available", id)),
            Error::TokenError(message) => (StatusCode::UNAUTHORIZED, message.to_string()),
            Error::DecodeError(message) => (StatusCode::FORBIDDEN, message.to_string()),
            Error::StringError(message) => (StatusCode::FORBIDDEN, message.to_string()),
            Error::UserUnauthorized(message) => (StatusCode::UNAUTHORIZED, message.to_string()),
        };

        let body = Body::from(json!({
            "status": "failed",
            "error": error_message
        }).to_string());

        let mut response = Response::new(body);
        *response.status_mut() = status;
        response
    }
}
