pub type Result<T> = core::result::Result<T,Error>;

#[derive(Debug)]
pub enum Error {
    LoginFail,
    DatabaseError(String)
}

// region:    --- Error Boilerplate
impl core::fmt::Display for Error {
	fn fmt(
		&self,
		fmt: &mut core::fmt::Formatter,
	) -> core::result::Result<(), core::fmt::Error> {
		write!(fmt, "{self:?}")
	}
}

impl From<surrealdb::Error> for Error {
    fn from(error: surrealdb::Error) -> Self {
        // Implement the conversion logic here
        // This could involve mapping the SurrealDB error to your custom error type
       Error::DatabaseError(error.to_string()) // Example conversion
    }
}