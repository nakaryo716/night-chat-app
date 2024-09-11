use app::AppState;
use models::rooms::RoomsDb;

mod app;
mod controllers;
mod models;
mod router;
mod utility;

#[tokio::main]
async fn main() {
    let rooms_db = RoomsDb::new();

    let app_state = AppState::new(rooms_db);
    let app = router::app(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
