
use koteka_gym::config::db::{DatabaseSource, DatabaseType, Sources};
use koteka_gym::repo::db::DBInterface;
use koteka_gym::repo::model::User;

#[tokio::main]
async fn main() -> Result<(),Box<dyn std::error::Error> >{
    // Connect to the server
    let mut db_source = DatabaseSource {
        db_type: DatabaseType::SurrealDB,
    };
       
    let conn = db_source.connect().await?;
    let user = User{ 
            username: "abdul", 
            user_type: "gymnast",
            email: "abdul.haris.djafar@gmail.com", 
            created_at: None,
            updated_at: None, 
            password: "asoigeboi"
    };

    conn.insert_record("user".to_string(),&user).await?;
    Ok(())
}