use axum::{http::StatusCode, response::{IntoResponse}};
use serde::Serialize;

pub type Result<T> = core::result::Result<T,Error>;

#[derive(Clone, Debug, Serialize)]
pub enum Error {
    LoginFail,
    DatabaseError(String)
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

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        // Implement conversion to response
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}