use axum::{
    extract::State,
    response::IntoResponse,
    Json,
};

use crate::{
    errors::Result,
    repo::model::{PayloadUser, User},
    services::gym::GymServices,
};

pub async fn register_gym_user(
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

    Ok(Json(user_id))
}
