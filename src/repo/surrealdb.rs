use std::error::Error;
use axum::async_trait;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use surrealdb::sql::Thing;
use crate::config::db::SurrealDb;
use super::interface;
use interface::DBInterface;

/* Struct for deserialization of records */
#[derive(Debug, Deserialize)]
pub struct Record {
    #[allow(dead_code)]
    id: Thing,
}

/* Implementation of DBInterface for SurrealDb */
#[async_trait]
impl DBInterface for SurrealDb {
    /* Method to insert a record into the database */
    async fn insert_record<T: Serialize + Sync>(&self, tb_name: String, data: &T) -> Result<bool, Box<dyn Error>> {
        // Clone the client
        let client = self.client.clone().unwrap();
        // Insert the record into the database
        let created: Vec<Record> = client.insert(tb_name).content(data).await?;
        
        dbg!(created); // Debug output for created records
        
        Ok(true)
    }
    
    /* Method to select records from the database */
    async fn select<T: DeserializeOwned + Sync>(&self, tb_name: String) -> Result<Vec<T>, Box<dyn Error>> {
        // Clone the client
        let client = self.client.clone().unwrap();
        // Select records from the database
        let data: Vec<T> = client.select(tb_name).await?;
        Ok(data)
    }

    /* Method to delete a record from the database */
    async fn delete(&self, _tb_name: String, _id: String) -> Result<bool, Box<dyn Error>> {
        // Placeholder implementation
        Ok(false)
    } 
}
