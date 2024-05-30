pub mod config;
pub mod repo;
pub use config::environment;
use crate::{config::db::{Connection, Sources}, repo::{ interface::DBInterface, model::User}};

// environment config test
#[tokio::test]
 async fn check_db_connection() -> Result<(), Box<dyn std::error::Error>>{

    let mut surreal_db = config::db::DatabaseSource { 
        db_type: config::db::DatabaseType::SurrealDB
    };
       
    let conn = surreal_db.connect().await?;

    assert_eq!("Pong!", conn.ping());

    Ok(())
}

#[tokio::test]
 async fn test_insert_user_record() -> Result<(), Box<dyn std::error::Error>>{

    let mut surreal_db = config::db::DatabaseSource { 
        db_type: config::db::DatabaseType::SurrealDB
    };
       
    let conn = surreal_db.connect().await?;

    let user = User{ 
        username: String::from("koteka"), 
        user_type: String::from("gymnast"),
        email: String::from("abdul.haris.djafar@gmail.com"), 
        created_at: None,
        updated_at: None, 
        password: String::from("asoigeboi")
    };

    let result = conn.insert_record(String::from("user"), &user).await?;

    assert_eq!(true, result);

    Ok(())
}

#[tokio::test]
 async fn test_select_user_record() -> Result<(), Box<dyn std::error::Error>>{

    let mut surreal_db = config::db::DatabaseSource { 
        db_type: config::db::DatabaseType::SurrealDB
    };
       
    let conn = surreal_db.connect().await?;

    let result:Vec<User> = conn.select(String::from("user")).await?;

    assert_ne!(0,result.len());

    Ok(())
}

#[tokio::test]
 async fn test_delete_user_record() -> Result<(), Box<dyn std::error::Error>>{

    let mut surreal_db = config::db::DatabaseSource { 
        db_type: config::db::DatabaseType::SurrealDB
    };
       
    let conn = surreal_db.connect().await?;

    let result = conn.delete(String::from("user"), String::from("id")).await?;

    assert_eq!(true,result);

    Ok(())
}