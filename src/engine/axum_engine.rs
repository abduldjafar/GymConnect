use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    config::{self, db::Sources},
    errors::Result,
    router::axum_router::gym,
    services::gym::GymServices,
};

pub async fn run() -> Result<()> {
    let mut surreal_db = config::db::DatabaseSource {
        db_type: config::db::DatabaseType::SurrealDB,
    };

    // Connect to the database
    let conn = surreal_db.connect().await?;
    let gym_services = GymServices { repo: conn };

    let routes_all = Router::new().merge(gym_routes(gym_services));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, routes_all).await.unwrap();

    Ok(())
}

pub fn gym_routes(gym_services: GymServices) -> Router {
    Router::new()
        .route("/api/v1/gym", post(gym::register))
        .route("/api/v1/gym/:id", get(gym::get_profile))
        .with_state(gym_services)
}
