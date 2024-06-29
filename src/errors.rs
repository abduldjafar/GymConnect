use std::string::FromUtf8Error;
use axum::{body::Body, http::{Response, StatusCode}, response::IntoResponse};
use redis::RedisError;
use serde::Serialize;

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
    fn into_response(self) -> axum::response::Response {
        match &self {
            Error::LoginFail => {
                let mut response = Response::new(Body::new("Login failed".to_string()));
                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                response
            },
            Error::DatabaseError(error) => {
                let mut response = Response::new(Body::new(format!("There was a problem with the database: {}", error)));
                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                response
            },
            Error::DataExist(id) => {
                let mut response = Response::new(Body::new(format!("{} already registered", id)));
                *response.status_mut() = StatusCode::NOT_ACCEPTABLE;
                response
            },
            Error::DataNotAvaliable(id) => {
                let mut response = Response::new(Body::new(format!("{} Not Available", id)));
                *response.status_mut() = StatusCode::NOT_FOUND;
                response
            },
            Error::TokenError(message) => {
                let mut response = Response::new(Body::new(format!("{}", message)));
                *response.status_mut() = StatusCode::UNAUTHORIZED;
                response
            },
            Error::DecodeError(message) => {
                let mut response = Response::new(Body::new(format!("{}", message)));
                *response.status_mut() = StatusCode::FORBIDDEN;
                response
            },
            Error::StringError(message) => {
                let mut response = Response::new(Body::new(format!("{}", message)));
                *response.status_mut() = StatusCode::FORBIDDEN;
                response
            },
        }
    }
}
