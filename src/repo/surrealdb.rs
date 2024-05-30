use std::error::Error;

use axum::async_trait;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use surrealdb::{engine::remote::ws::Client, sql::Thing, Surreal};

use crate::config::db::SurrealDb;

use interface::DBInterface;

use super::interface;

#[derive(Debug, Deserialize)]
pub struct Record {
    #[allow(dead_code)]
    id: Thing,
}


#[async_trait]
impl DBInterface for SurrealDb {
    async fn insert_record<T: Serialize + Sync>(&self, tab_name: String, data: &T) -> Result<bool, Box<dyn Error>> {
        let client = <std::option::Option<Surreal<Client>> as Clone>::clone(&self.client).unwrap();
        let created: Vec<Record> = client.insert(tab_name).content(data).await?;
        
        dbg!(created);
        
        Ok(true)
    }
    
    async fn select<T: DeserializeOwned + Sync>(&self, tab_name: String) -> Result<Vec<T>, Box<dyn Error>>{
        let client = <std::option::Option<Surreal<Client>> as Clone>::clone(&self.client).unwrap();
        let data: Vec<T> = client.select(tab_name).await?;
        Ok(data)
    }
}