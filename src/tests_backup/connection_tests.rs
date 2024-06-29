use crate::{
    config::{
        self,
        db::{Connection, Sources},
    },
    errors::Result,
};

#[tokio::test]
async fn check_db_connection() -> Result<()> {
    // Initialize database source
    let mut surreal_db = config::db::DatabaseSource {
        db_type: config::db::DatabaseType::SurrealDB,
    };

    // Connect to the database
    let conn = surreal_db.connect().await?;

    // Assert connection is successful
    assert_eq!("Pong!", conn.ping());

    Ok(())
}
