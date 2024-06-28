use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use serde_json::json;

use crate::{
    authorization::jwt::generate_jwt_token,
    engine::axum_engine::AppState,
    errors::Result,
    repo::model::{PayloadGymRequest, PayloadIdResponses, PayloadUser, User},
};

pub async fn register(
    State(app_state): State<AppState>,
    payload: Json<PayloadUser>,
) -> Result<impl IntoResponse> {
    let user = User {
        username: payload.username.clone(),
        user_type: payload.user_type.clone(),
        email: payload.email.clone(),
        created_at: None,
        updated_at: None,
        password: payload.password.clone(),
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

    Ok(Json(payload_id_responses))
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
