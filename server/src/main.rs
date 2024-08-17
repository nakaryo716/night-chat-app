use app_state::AppState;
use auth::UserDataDb;
use rooms::RoomsDb;

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
    let rooms_db = RoomsDb::new();
    let users_pool = UserDataDb::new("database_url").await.unwrap();
    
    let state = AppState::new(rooms_db, users_pool);
    let app = router::app(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
