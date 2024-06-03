use crate::{
    config::{self, db::Sources},
    repo::model::User,
    services::gym::GymServices,
};

#[tokio::test]
async fn services_gym_test_register() -> Result<(), Box<dyn std::error::Error>> {
    let mut surreal_db = config::db::DatabaseSource {
        db_type: config::db::DatabaseType::SurrealDB,
    };

    // Connect to the database
    let conn = surreal_db.connect().await?;
    let gym_services = GymServices { repo: conn };

    // Create a new user
    let user = User {
        username: String::from("koteka"),
        user_type: String::from("gym"),
        email: String::from("koteka@asoi.com"),
        created_at: None,
        updated_at: None,
        password: String::from("asoigeboi"),
    };

    let register_gym_user = gym_services.register_profile(&user).await?;

    assert_ne!(None, register_gym_user);

    Ok(())
}

#[tokio::test]
async fn services_gym_test_update() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

#[tokio::test]
async fn services_gym_test_delete_by_id() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

#[tokio::test]
async fn services_gym_test_select_by_id() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
