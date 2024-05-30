use std::error::Error;
use axum::async_trait;
use serde::{de::DeserializeOwned, Serialize};

/* Trait for database interface operations */
#[async_trait]
pub trait DBInterface {
    /* Method to insert a record into the database */
    async fn insert_record<T: Serialize + Sync>(&self, tb_name: String, data: &T) -> Result<bool, Box<dyn Error>>;

    /* Method to select records from the database */
    async fn select<T: DeserializeOwned + Sync>(&self, tb_name: String) -> Result<Vec<T>, Box<dyn Error>>;

    /* Method to delete a record from the database */
    async fn delete(&self, tb_name: String, id: String) -> Result<bool, Box<dyn Error>>;
}
