
use std::error::Error;

use axum::async_trait;
use serde::{de::DeserializeOwned, Serialize};
use crate::config::db::DatabaseClient;
use super::interface::DBInterface;

#[async_trait]
impl DBInterface for DatabaseClient {
    async fn insert_record<T: Serialize + Sync>(&self, tab_name: String, data: &T) -> Result<bool, Box<dyn Error>> {
        match self {
            DatabaseClient::Surreal(surrealdb) => surrealdb.insert_record(tab_name, data).await,
        }
    }

    async fn select<T: DeserializeOwned + Sync>(&self, tab_name: String) -> Result<Vec<T>, Box<dyn Error>> {
        match self {
            DatabaseClient::Surreal(surrealdb) => surrealdb.select(tab_name).await,
        }
    }

}
