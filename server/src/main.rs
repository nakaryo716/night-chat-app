use app_state::AppState;

mod app_state;
mod auth;
mod handlers;
mod rooms;
mod router;
mod session;
mod utility;
mod websocket;

#[tokio::main]
async fn main() {
    let state = AppState::new();

    let app = router::app(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
