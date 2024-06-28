use std::env;

#[derive(Debug, PartialEq, Clone)]
pub struct Environment {
    pub db_host: String,
    pub db_port: String,
    pub db_user: String,
    pub db_pass: String,
    pub db_name: String,
    pub db_namespace: String,
    pub host_ip: String,
    pub host_port: String,
    pub refresh_token_private_key: String,
    pub refresh_token_public_key: String,
    pub access_token_private_key: String,
    pub access_token_public_key: String,
}

impl Environment {
    pub fn new() -> Self {
        let db_host = env::var("DB_HOST").unwrap_or(String::from("none"));
        let db_port = env::var("DB_PORT").unwrap_or(String::from("none"));
        let db_user = env::var("DB_USER").unwrap_or(String::from("none"));
        let db_pass = env::var("DB_PASS").unwrap_or(String::from("none"));
        let db_name = env::var("DB_NAME").unwrap_or(String::from("none"));
        let db_namespace = env::var("DB_NAMESPACE").unwrap_or(String::from("none"));
        let host_ip = env::var("HOST_IP").unwrap_or(String::from("none"));
        let host_port = env::var("HOST_PORT").unwrap_or(String::from("none"));
        let refresh_token_private_key =
            env::var("REFRESH_TOKEN_PRIVATE_KEY").unwrap_or(String::from("none"));
        let refresh_token_public_key =
            env::var("REFRESH_TOKEN_PUBLIC_KEY").unwrap_or(String::from("none"));
        let access_token_private_key =
            env::var("ACCESS_TOKEN_PRIVATE_KEY").unwrap_or(String::from("none"));
        let access_token_public_key =
            env::var("ACCESS_TOKEN_PUBLIC_KEY").unwrap_or(String::from("none"));

        Environment {
            db_host,
            db_port,
            db_user,
            db_pass,
            db_name,
            db_namespace,
            host_ip,
            host_port,
            refresh_token_private_key,
            refresh_token_public_key,
            access_token_private_key,
            access_token_public_key,
        }
    }
}
