
use std::error::Error;

use axum::async_trait;
use serde::Serialize;
use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::{config::db::{DatabaseConnection, SurrealDb}, repo::model::Record};

#[async_trait]
pub trait DBInterface {
    async fn insert_record<T: Serialize + Sync>(&self, tab_name: String, data: &T) -> Result<bool, Box<dyn Error>>;
    async fn update_record<T: Serialize + Sync>(&self, tab_name: String, data: &T) -> Result<bool, Box<dyn Error>>;
}

#[async_trait]
impl DBInterface for SurrealDb {
    async fn insert_record<T: Serialize + Sync>(&self, tab_name: String, data: &T) -> Result<bool, Box<dyn Error>> {
        let client = <std::option::Option<Surreal<Client>> as Clone>::clone(&self.client).unwrap();
        let created: Vec<Record> = client.insert(tab_name).content(data).await?;
        dbg!(created);
        Ok(true)
    }

    async fn update_record<T: Serialize + Sync>(&self, tab_name: String, data: &T) -> Result<bool, Box<dyn Error>> {
        Ok(true)
    }
}

#[async_trait]
impl DBInterface for DatabaseConnection {
    async fn insert_record<T: Serialize + Sync>(&self, tab_name: String, data: &T) -> Result<bool, Box<dyn Error>> {
        match self {
            DatabaseConnection::Surreal(surrealdb) => surrealdb.insert_record(tab_name, data).await,
        }
    }

    async fn update_record<T: Serialize + Sync>(&self, tab_name: String, data: &T) -> Result<bool, Box<dyn Error>> {
        match self {
            DatabaseConnection::Surreal(surrealdb) => surrealdb.insert_record(tab_name, data).await,
        }
    }
}
