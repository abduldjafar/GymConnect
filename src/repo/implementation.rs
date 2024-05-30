use std::error::Error;

use crate::config::db::DatabaseConnection;

use super::{db::DBInterface, model::User};

impl <'a>User <'a> {
    pub async fn create(&self,db_conn:DatabaseConnection,tb_name:String) -> Result<bool,Box<dyn Error>>{
        db_conn.insert_record(tb_name, &self).await?;
        Ok(true)
    }
}