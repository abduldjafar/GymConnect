use koteka_gym::{config::{self, db::Sources}, engine::engine::{self, Cmd, EngineType}, errors::Result, repo::{interface::DBInterface, model::{Id, User}}};



#[tokio::main]
async fn main() -> Result<()>{
    let engine = EngineType::Axum;
    engine.run().await?;
    Ok(())
}

