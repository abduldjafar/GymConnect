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
        let client = self.client.clone().unwrap();
        let created: Vec<Record> = client.insert(tb_name).content(data).await?;
        
        dbg!(created); // Debug output for created records
        
        Ok(true)
    }
    
    /* Method to select records from the database */
    async fn select<T: DeserializeOwned + Sync>(&self, tb_name: String) -> Result<Vec<T>, Box<dyn Error>> {
        let client = self.client.clone().unwrap();
        let data: Vec<T> = client.select(tb_name).await?;
        Ok(data)
    }

    /* Method to delete a record from the database */
    async fn delete(&self, id: String) -> Result<bool, Box<dyn Error>> {
        let client = self.client.clone().unwrap();
        let result = client.query(format!("DELETE {}", id)).await?.check();

        match result {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    /* Method to update a record in the database */
    async fn update_record<T>(&self, id: String, tb_name: String, data: &T) -> Result<bool, Box<dyn Error>>
    where
        T: Serialize + for<'de> Deserialize<'de> + Sync,
    {
        let data_id: Vec<&str> = id.split(':').collect();
        let client = self.client.clone().unwrap();
        
        let updated_result: Option<T> = client
            .update((tb_name, data_id[1]))
            .content(data)
            .await?;

        Ok(updated_result.is_some())
    }

    /* Method to select records with parameters from the database */
    async fn select_with_params<T: DeserializeOwned + Sync>(&self, tb_name: String, param: String) -> Result<Vec<T>, Box<dyn Error>> {
        let client = self.client.clone().unwrap();

        let filtered_query = if param.is_empty() {
            String::new()
        } else {
            format!("where {}", param)
        };

        let sql = format!("SELECT * FROM {} {}", tb_name, filtered_query);
        let mut results = client.query(&sql).await?;
        
        let data: Vec<T> = results.take(1)?;
        Ok(data)
    }
}
