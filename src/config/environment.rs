
use std::env;

#[derive(Debug,PartialEq)]
pub struct  Environment {
    pub db_host: String,
    pub db_port: String,
    pub db_user: String,
    pub db_pass: String,
}


impl Environment{
    pub fn new() -> Self{
        let db_host = env::var("DB_HOST").unwrap_or(String::from("none"));
        let db_port = env::var("DB_PORT").unwrap_or(String::from("none"));
        let db_user = env::var("DB_USER").unwrap_or(String::from("none"));
        let db_pass = env::var("DB_PASS").unwrap_or(String::from("none"));

        Environment{
            db_host,
            db_port,
            db_user,
            db_pass
        }
    }
}