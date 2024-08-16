use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use super::{AuthManageDb, Credential, UserData, UserDataDb};

pub async fn create_user_handle(
    State(user_data_db): State<UserDataDb>,
    Json(payload): Json<Credential>,
) -> Result<impl IntoResponse, StatusCode> {
    let new_user = UserData::new(payload);
    user_data_db
        .insert_new_user(new_user)
        .await
        .map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::OK)
}

pub async fn verify_password_handle(
    State(user_data_db): State<UserDataDb>,
    Json(payload): Json<Credential>,
) -> Result<impl IntoResponse, StatusCode> {
    user_data_db
        .verify_password(payload)
        .await
        .map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::OK)
}

pub async fn delete_user_handle(
    State(user_data_db): State<UserDataDb>,
    Json(payload): Json<Credential>,
) -> Result<impl IntoResponse, StatusCode> {
    user_data_db
        .delete_user(payload)
        .await
        .map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::OK)
}
