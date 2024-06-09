use super::interface::DBInterface;
use crate::{config::db::DatabaseClient, errors::Result};
use axum::async_trait;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::error::Error;

/* Implementation of the DBInterface trait for DatabaseClient */
#[async_trait]
impl DBInterface for DatabaseClient {
    /* Method to insert a record into the database */
    async fn insert_record<T: Serialize + Sync, U: DeserializeOwned + Sync + Clone>(
        &self,
        tb_name: String,
        data: &T,
    ) -> Result<Option<U>> {
        match self {
            DatabaseClient::Surreal(surrealdb) => surrealdb.insert_record(tb_name, data).await,
            // Add other database client implementations here
        }
    }

    /* Method to select records from the database */
    async fn select<T: DeserializeOwned + Sync>(&self, tb_name: String) -> Result<Vec<T>> {
        match self {
            DatabaseClient::Surreal(surrealdb) => surrealdb.select(tb_name).await,
            // Add other database client implementations here
        }
    }

    /* Method to delete a record from the database */
    async fn delete(&self, id: String) -> Result<bool> {
        match self {
            DatabaseClient::Surreal(surrealdb) => surrealdb.delete(id).await,
            // Add other database client implementations here
        }
    }

    async fn update_record<T: Serialize + for<'de> Deserialize<'de> + Sync>(
        &self,
        id: String,
        tb_name: String,
        data: &T,
    ) -> Result<bool> {
        match self {
            DatabaseClient::Surreal(surrealdb) => surrealdb.update_record(id, tb_name, data).await,
        }
    }

    async fn select_where<T: DeserializeOwned + Sync>(
        &self,
        tb_name: String,
        filter: String,
        columns: String, // separate columns by ',' in string format
    ) -> Result<Vec<T>> {
        match self {
            DatabaseClient::Surreal(surrealdb) => {
                surrealdb.select_where(tb_name, filter, columns).await
            }
        }
    }
}
