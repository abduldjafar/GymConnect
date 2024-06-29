use crate::{
    authorization::jwt::{generate_jwt_token, save_token_data_to_redis},
    engine::axum_engine::AppState,
    errors::Result,
    repo::model::{PayloadGymRequest, PayloadIdResponses, PayloadUser, User},
};
use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use rand_core::OsRng;
use serde_json::json;

pub async fn register(
    State(app_state): State<AppState>,
    payload: Json<PayloadUser>,
) -> Result<impl IntoResponse> {
    let cloned_app_state = app_state.clone();

    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Argon2::default()
        .hash_password(payload.password.as_bytes(), &salt)?
        .to_string();

    let user = User {
        username: payload.username.clone(),
        user_type: payload.user_type.clone(),
        email: payload.email.clone(),
        created_at: None,
        updated_at: None,
        password: hashed_password,
    };

    let svc = app_state.gym_services;
    let user_id = svc.register_profile(&user).await?.unwrap();

    let access_token_details = generate_jwt_token(
        format!("{}:{}", user_id.id.tb, user_id.id.id.to_string()),
        60,
        app_state.environment.access_token_private_key,
    )
    .await?;

    let refresh_token_details = generate_jwt_token(
        format!("{}:{}", user_id.id.tb, user_id.id.id.to_string()),
        60,
        app_state.environment.refresh_token_private_key,
    )
    .await?;

    let payload_id_responses = PayloadIdResponses {
        id: format!("{}:{}", user_id.id.tb, user_id.id.id.to_string()),
    };

    save_token_data_to_redis(&cloned_app_state, &access_token_details, 60).await?;
    save_token_data_to_redis(&cloned_app_state, &refresh_token_details, 60).await?;

    Ok(Json(json!({
        "status":"success",
        "token":access_token_details.token,
        "refresh_token":refresh_token_details.token,
        "response":payload_id_responses
    }
    )))
}

pub async fn get_profile(
    State(app_state): State<AppState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse> {
    let svc = app_state.gym_services;
    let data = svc.profile_details(id).await?;

    Ok(Json(data))
}

pub async fn update_profile(
    State(app_state): State<AppState>,
    Path(id): Path<String>,
    payload: Json<PayloadGymRequest>,
) -> Result<impl IntoResponse> {
    let svc = app_state.gym_services;
    svc.update_profile(&payload, id).await?;

    Ok(Json(json!({
        "status": "success",
    })))
}
