use std::sync::Arc;

use crate::{
    engine::axum_engine::AppState,
    errors::{self, Result},
    repo::model::{PayloadIdResponses, PayloadUser, User},
};
use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Extension, Json,
};
use rand_core::OsRng;
use serde_json::json;

use super::midleware::jwt_auth::JWTAuthMiddleware;

pub async fn register(
    State(app_state): State<Arc<AppState>>,
    payload: Json<PayloadUser>,
) -> Result<impl IntoResponse> {
    // Generate salt and hash the password
    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Argon2::default()
        .hash_password(payload.password.as_bytes(), &salt)?
        .to_string();

    // Create user with hashed password
    let user = User {
        username: payload.username.clone(),
        user_type: String::from("gymnast"),
        email: payload.email.clone(),
        created_at: None,
        updated_at: None,
        password: hashed_password,
    };

    // Register user profile
    let svc = &app_state.gymnast_services;
    let user_id = svc.register_profile(&user).await?.unwrap();

    // Create response payload
    let payload_id_responses = PayloadIdResponses {
        id: format!("{}:{}", user_id.id.tb, user_id.id.id.to_string()),
    };

    Ok(Json(json!({
        "status": "success",
        "response": payload_id_responses
    })))
}

pub async fn get_profile(
    State(app_state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Extension(jwt): Extension<JWTAuthMiddleware>,
) -> Result<impl IntoResponse> {
    // Get profile details
    let svc = &app_state.gymnast_services;
    let data = svc.profile_details(id).await?;

    if jwt.user_id != data.id {
        return Err(errors::Error::UserUnauthorized(String::from(
            "user unauthorized to get profile",
        )));
    }

    Ok(Json(data))
}
