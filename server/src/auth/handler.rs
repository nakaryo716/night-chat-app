use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use super::{AuthManage, Credential, UserDataDb};

pub async fn create_user_handle(
    State(app_state): State<UserDataDb>,
    Json(payload): Json<Credential>,
) -> Result<impl IntoResponse, StatusCode> {
    app_state
        .add_user(payload)
        .await
        .map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::OK)
}

pub async fn verify_user_handle(
    State(app_state): State<UserDataDb>,
    Json(payload): Json<Credential>,
) -> Result<impl IntoResponse, StatusCode> {
    app_state
        .verify_user(payload)
        .await
        .map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::UNAUTHORIZED)?;

    Ok(StatusCode::OK)
}

pub async fn delete_user_handle(
    State(app_state): State<UserDataDb>,
    Json(payload): Json<Credential>,    
) -> Result<impl IntoResponse, StatusCode> {
    app_state
        .delete_user(payload)
        .await
        .map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::UNAUTHORIZED)?;

    Ok(StatusCode::OK)
}
