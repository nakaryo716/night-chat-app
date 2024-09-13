use axum::response::{Html, IntoResponse};

pub async fn index() -> impl IntoResponse {
    Html(include_str!("../../index.html"))
}

pub async fn user_view() -> impl IntoResponse {
    Html(include_str!("../../user.html"))
}
