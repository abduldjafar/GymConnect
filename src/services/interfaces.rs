use std::error::Error;

use axum::async_trait;
use serde::{de::DeserializeOwned, Serialize};

use crate::config::db::DatabaseClient;

#[async_trait]
pub trait ServicesInterface {
    async fn register<T: Serialize + Sync, U: DeserializeOwned + Sync + Clone>(
        &self,
        tb_name: String,
        data: &T,
    ) -> Result<Option<U>, Box<dyn Error>>;
}
