use axum::async_trait;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::error::Error;

/* Trait for database interface operations */
#[async_trait]
pub trait DBInterface {
    /* Method to insert a record into the database */
    async fn insert_record<T: Serialize + Sync>(
        &self,
        tb_name: String,
        data: &T,
    ) -> Result<bool, Box<dyn Error>>;

    /* Method to select records from the database */
    async fn select<T: DeserializeOwned + Sync>(
        &self,
        tb_name: String,
    ) -> Result<Vec<T>, Box<dyn Error>>;

    /* Method to delete a record from the database */
    async fn delete(&self, id: String) -> Result<bool, Box<dyn Error>>;

    /* Method to update a record into the database */
    async fn update_record<T: Serialize + for<'de> Deserialize<'de> + Sync>(
        &self,
        id: String,
        tb_name: String,
        data: &T,
    ) -> Result<bool, Box<dyn Error>>;

    /* Method to select records from the database */
    async fn select_with_params<T: DeserializeOwned + Sync>(
        &self,
        tb_name: String,
        param: String,
    ) -> Result<Vec<T>, Box<dyn Error>>;
}
