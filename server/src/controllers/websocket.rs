use axum::extract::{Path, State, WebSocketUpgrade};
use axum::response::IntoResponse;
use tracing::{event, Level};

use crate::models::rooms::RoomsDb;
use crate::models::websocket::websocket_task;

pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(rooms_db): State<RoomsDb>,
    Path(room_id): Path<String>,
) -> impl IntoResponse {
    ws.on_failed_upgrade(|e| {
        let message = format!("error: [{:?}]", e);
        event!(Level::WARN, message)
    })
    .on_upgrade(|socket| websocket_task(socket, rooms_db, room_id))
}
