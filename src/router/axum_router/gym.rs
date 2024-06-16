use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};

use crate::{
    errors::Result,
    repo::model::{PayloadIdResponses, PayloadUser, User},
    services::gym::GymServices,
};

pub async fn register(
    State(svc): State<GymServices>,
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

    let user_id = svc.register_profile(&user).await?.unwrap();

    let payload_id_responses = PayloadIdResponses {
        id: format!("{}:{}", user_id.id.tb, user_id.id.id.to_string()),
    };

    Ok(Json(payload_id_responses))
}

pub async fn get_profile(
    State(svc): State<GymServices>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse> {
    let data = svc.profile_details(id).await?;

    Ok(Json(data))
}
