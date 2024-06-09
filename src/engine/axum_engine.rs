use axum::{routing::get, Router};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub async fn run() {
    let routes_all = Router::new().merge(routes());
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, routes_all).await.unwrap();
}

fn routes() -> Router {
    Router::new().route("/ping", get(ping))
}

// basic handler that responds with a static string
async fn ping() -> &'static str {
    "pong!"
}
