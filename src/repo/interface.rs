use std::error::Error;

use axum::async_trait;
use serde::{de::DeserializeOwned, Serialize};

#[async_trait]
pub trait DBInterface {
    async fn insert_record<T: Serialize + Sync>(&self, tab_name: String, data: &T) -> Result<bool, Box<dyn Error>>;
    async fn select<T: DeserializeOwned + Sync>(&self, tab_name: String) -> Result<Vec<T>, Box<dyn Error>>;
}