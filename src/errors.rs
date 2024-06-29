use std::string::FromUtf8Error;

use axum::{body::Body, http::{Response, StatusCode}, response::IntoResponse};
use redis::RedisError;
use serde::Serialize;
use surrealdb::error;

pub type Result<T> = core::result::Result<T,Error>;

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
	fn fmt(
		&self,
		fmt: &mut core::fmt::Formatter,
	) -> core::result::Result<(), core::fmt::Error> {
		write!(fmt, "{self:?}")
	}
}


impl std::error::Error for Error {}

impl From<surrealdb::Error> for Error {
    fn from(error: surrealdb::Error) -> Self {
        // Implement the conversion logic here
        // This could involve mapping the SurrealDB error to your custom error type
       Error::DatabaseError(error.to_string()) // Example conversion
    }
}

impl From<jsonwebtoken::errors::Error> for Error {
    fn from(error: jsonwebtoken::errors::Error) -> Self {
        Error::TokenError(error.to_string())
    }
}

impl  From<base64::DecodeError> for Error {
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
                let mut response = Response::new(Body::new("login failed".to_string()));
                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR.into();
                response
            },
            Error::DatabaseError(_) => {
				let mut response = Response::new(Body::new("There was a problem with the database".to_string()));
                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR.into();

                response
                
			},
            Error::DataExist(id) => {
				let mut response = Response::new(Body::new(format!("Data with {} already registered",id)));
                *response.status_mut() = StatusCode::NOT_ACCEPTABLE.into();

                response
                
			},
            Error::DataNotAvaliable(id) => {
				let mut response = Response::new(Body::new(format!("Data with {} Not Avaliable",id)));
                *response.status_mut() = StatusCode::NOT_FOUND.into();

                response
                
			},
            Error::TokenError(message) =>{
                let mut response = Response::new(Body::new(format!("{}",message)));
                *response.status_mut() = StatusCode::UNAUTHORIZED.into();

                response
            },
            Error::DecodeError(message)=>{
                let mut response = Response::new(Body::new(format!("{}",message)));
                *response.status_mut() = StatusCode::FORBIDDEN.into();

                response
            },
            Error::StringError(message)=>{
                let mut response = Response::new(Body::new(format!("{}",message)));
                *response.status_mut() = StatusCode::FORBIDDEN.into();

                response
            },
        }
    }
}