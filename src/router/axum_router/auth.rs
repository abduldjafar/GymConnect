use std::sync::Arc;

use argon2::{Argon2, PasswordHash, PasswordVerifier};
use axum::http::{header, HeaderMap, Response};
use axum::{extract::State, response::IntoResponse, Json};
use axum_extra::extract::cookie::{Cookie, SameSite};
use serde_json::json;

use crate::authorization::jwt::{generate_jwt_token, save_token_data_to_redis};
use crate::{adapter::model::LoginUserSchema, engine::axum_engine::AppState};

use crate::errors::{self, Result};

pub async fn login_user(
    State(app_state): State<Arc<AppState>>,
    Json(body): Json<LoginUserSchema>,
) -> Result<impl IntoResponse> {
    let auth_svc = &app_state.auth_services;
    let gym_svc = &app_state.gym_services;
    let gymnast_svc = &app_state.gymnast_services;
    let env = app_state.environment.clone();

    let user = auth_svc.login(body.email).await?;

    let is_valid = match PasswordHash::new(&user.password) {
        Ok(parsed_hash) => Argon2::default()
            .verify_password(body.password.as_bytes(), &parsed_hash)
            .map_or(false, |_| true),
        Err(_) => false,
    };

    if !is_valid {
        return Err(errors::Error::LoginFail);
    }

    let user_id = {
        if user.user_type == "gym" {
            let data = gym_svc.profile_details(user.id.to_string()).await?;
            data.id
        } else if user.user_type == "gymnast" {
            let data = gymnast_svc.profile_details(user.id.to_string()).await?;
            data.id
        } else {
            return Err(errors::Error::DataExist(format!("{} not found", user.id)));
        }
    };

    let access_token_details =
        generate_jwt_token(user_id.clone(), 60, env.access_token_private_key.to_owned()).await?;

    let refresh_token_details =
        generate_jwt_token(user_id, 60, env.refresh_token_private_key.to_owned()).await?;

    save_token_data_to_redis(&app_state, &access_token_details, 60).await?;
    save_token_data_to_redis(&app_state, &refresh_token_details, 60).await?;

    let access_cookie = Cookie::build((
        "access_token",
        access_token_details.token.clone().unwrap_or_default(),
    ))
    .path("/")
    .max_age(time::Duration::minutes(60 * 60))
    .same_site(SameSite::Lax)
    .http_only(true);

    let refresh_cookie = Cookie::build((
        "refresh_token",
        refresh_token_details.token.unwrap_or_default(),
    ))
    .path("/")
    .max_age(time::Duration::minutes(60 * 60))
    .same_site(SameSite::Lax)
    .http_only(true);

    let logged_in_cookie = Cookie::build(("logged_in", "true"))
        .path("/")
        .max_age(time::Duration::minutes(60 * 60))
        .same_site(SameSite::Lax)
        .http_only(false);

    let mut response = Response::new(
        json!({"status": "success", "access_token": access_token_details.token.unwrap()})
            .to_string(),
    );
    let mut headers = HeaderMap::new();
    headers.append(
        header::SET_COOKIE,
        access_cookie.to_string().parse().unwrap(),
    );
    headers.append(
        header::SET_COOKIE,
        refresh_cookie.to_string().parse().unwrap(),
    );
    headers.append(
        header::SET_COOKIE,
        logged_in_cookie.to_string().parse().unwrap(),
    );

    response.headers_mut().extend(headers);
    Ok(response)
}
