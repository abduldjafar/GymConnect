use crate::{
    config::{
        self,
        db::{Connection, Sources},
    },
    environment::Environment,
    errors::Result,
    router::axum_router::gym,
    services::{gym::GymServices, gymnast::GymnastServices},
};
use axum::{
    routing::{get, post},
    Router,
};
use redis::Client;

#[derive(Clone)]
pub struct AppState {
    pub gym_services: GymServices,
    pub gymnast_services: GymnastServices,
    pub redis_client: Client,
    pub environment: Environment,
}

pub async fn run() -> Result<()> {
    let mut surreal_db = config::db::DatabaseSource {
        db_type: config::db::DatabaseType::SurrealDB,
    };

    let redis_client = match Client::open("redis://localhost") {
        Ok(client) => {
            println!("✅ Connection to Redis is successful!");
            client
        }
        Err(e) => {
            println!("🔥 Error connecting to Redis: {}", e);
            std::process::exit(1);
        }
    };

    // Connect to the database
    let conn = surreal_db.connect().await?;
    let ping_db = conn.ping();

    if ping_db == String::from("Pong!") {
        println!("✅ {} from database!", ping_db);
    } else {
        println!("🔥 {} from database!", ping_db);
        std::process::exit(1);
    }

    let gym_services = GymServices { repo: conn.clone() };
    let gymnast_services = GymnastServices { repo: conn.clone() };
    let environment = Environment::new();

    let app_state = AppState {
        gym_services,
        gymnast_services,
        redis_client,
        environment,
    };

    let routes_all = Router::new().merge(gym_routes(app_state));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, routes_all).await.unwrap();

    Ok(())
}

pub fn gym_routes(app_state: AppState) -> Router {
    Router::new()
        .route("/api/v1/gym", post(gym::register))
        .route(
            "/api/v1/gym/:id",
            get(gym::get_profile).put(gym::update_profile),
        )
        .with_state(app_state)
}
