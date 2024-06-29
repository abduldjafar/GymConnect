use base64::{engine::general_purpose, Engine};
use redis::AsyncCommands;
use uuid::Uuid;

use super::token::{TokenClaims, TokenDetails};
use crate::{
    engine::axum_engine::AppState,
    errors::{self, Result},
};

pub async fn generate_jwt_token(
    user_id: String,
    ttl: i64,
    private_key: String,
) -> Result<TokenDetails> {
    let bytes_private_key = general_purpose::STANDARD.decode(private_key)?;

    let decoded_private_key = String::from_utf8(bytes_private_key)?;

    let now = chrono::Utc::now();
    let mut token_details = TokenDetails {
        user_id,
        token_uuid: Uuid::new_v4(),
        expires_in: Some((now + chrono::Duration::minutes(ttl)).timestamp()),
        token: None,
    };

    let claims = TokenClaims {
        sub: token_details.user_id.to_string(),
        token_uuid: token_details.token_uuid.to_string(),
        exp: token_details.expires_in.unwrap(),
        iat: now.timestamp(),
        nbf: now.timestamp(),
    };

    let header = jsonwebtoken::Header::new(jsonwebtoken::Algorithm::RS256);
    let token = jsonwebtoken::encode(
        &header,
        &claims,
        &jsonwebtoken::EncodingKey::from_rsa_pem(decoded_private_key.as_bytes())?,
    )?;

    token_details.token = Some(token);

    Ok(token_details)
}

pub async fn verify_jwt_token(public_key: String, token: &str) -> Result<TokenDetails> {
    let bytes_public_key = general_purpose::STANDARD.decode(public_key)?;
    let decoded_public_key = String::from_utf8(bytes_public_key)?;

    let validation = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::RS256);

    let decoded = jsonwebtoken::decode::<TokenClaims>(
        token,
        &jsonwebtoken::DecodingKey::from_rsa_pem(decoded_public_key.as_bytes())?,
        &validation,
    )?;

    let user_id = decoded.claims.sub.to_string();
    let token_uuid = Uuid::parse_str(decoded.claims.token_uuid.as_str())?;

    Ok(TokenDetails {
        token: None,
        token_uuid,
        user_id,
        expires_in: None,
    })
}

pub async fn save_token_data_to_redis(
    data: &AppState,
    token_details: &TokenDetails,
    max_age: i64,
) -> Result<()> {
    let mut redis_client = match data.redis_client.get_async_connection().await {
        Ok(client) => client,
        Err(_) => {
            return Err(errors::Error::DatabaseError(format!(
                "internal  server error"
            )))
        }
    };

    redis_client
        .set_ex(
            token_details.token_uuid.to_string(),
            token_details.user_id.to_string(),
            (max_age * 60) as u64,
        )
        .await?;

    Ok(())
}
