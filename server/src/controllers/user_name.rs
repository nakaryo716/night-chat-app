use axum::{http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::CookieJar;

use crate::models::user_name::{
    add_cookie_to_jar, delete_cookie_from_jar, get_user_name_from_cookie, UserNameForCoockie,
};

pub async fn register_user_name_as_cookie_handler(
    jar: CookieJar,
    Json(usr_name_payload): Json<UserNameForCoockie>,
) -> Result<impl IntoResponse, StatusCode> {
    let user_name = usr_name_payload.get_user_name().to_owned();
    let updated_cookie_jar = add_cookie_to_jar(user_name, jar);
    Ok((StatusCode::OK, updated_cookie_jar))
}

pub async fn get_user_name_from_cookie_handler(
    jar: CookieJar,
) -> Result<impl IntoResponse, StatusCode> {
    match get_user_name_from_cookie(jar) {
        Some(user) => {
            let user_name = UserNameForCoockie::new(user);
            Ok((StatusCode::OK, Json(user_name)))
        }
        None => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn delete_user_name_from_cookie_handler(
    jar: CookieJar,
) -> Result<impl IntoResponse, StatusCode> {
    let updated_cookie_jar = delete_cookie_from_jar(jar);
    Ok((StatusCode::NO_CONTENT, updated_cookie_jar))
}
