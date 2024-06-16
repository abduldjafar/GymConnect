#![allow(unused)]

use anyhow::Result;
use serde_json::json;

#[tokio::test]
pub async fn rest_api_tests() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3000")?;

    let gym_register = hc.do_post(
        "/api/v1/gym",
        json!({
            "username": "admin",
            "password":"admin"
        }),
    );

    gym_register.await?.print().await?;

    hc.do_get("/api/v1/gym/user:050zgqplr0k2unh1stji")
        .await?
        .print()
        .await?;
    Ok(())
}
