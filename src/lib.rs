pub mod config;
pub mod repo;
pub use config::environment;

// environment config test
#[tokio::test]
 async fn check_db_connection() -> Result<(), Box<dyn std::error::Error>>{
    let environment = environment::Environment::new();

    let db = config::db::Db{
        environment
    };
    
    let _conn = db.init().await?;
    Ok(())
}