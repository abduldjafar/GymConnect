pub mod config;
pub mod repo;
pub use config::environment;
use crate::config::db::Sources;

// environment config test
#[tokio::test]
 async fn check_db_connection() -> Result<(), Box<dyn std::error::Error>>{

    let mut surreal_db = config::db::DatabaseSource { 
        db_type: config::db::DatabaseType::SurrealDB
    };
       
    let conn = surreal_db.connect().await?;

    Ok(())
}