
use koteka_gym::config::db::{DatabaseSource, DatabaseType, Sources};

#[tokio::main]
async fn main() -> Result<(),Box<dyn std::error::Error> >{
    // Connect to the server
    let mut db_source = DatabaseSource {
        db_type: DatabaseType::SurrealDB,
    };
       
    let client = db_source.connect().await?;

    Ok(())
}