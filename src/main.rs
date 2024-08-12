use gymconnect::{
    engine::engine::{ Cmd, EngineType},
    errors::Result,
};

#[tokio::main]
async fn main() -> Result<()> {
    let engine = EngineType::Axum;
    engine.run().await?;
    Ok(())
}
