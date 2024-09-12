use axum::extract::{Path, State, WebSocketUpgrade};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use tracing::{event, Level};

use crate::models::rooms::{RoomId, RoomsDb};
use crate::models::websocket::websocket_task;

pub async fn websocket_upgrade_handler(
    ws: WebSocketUpgrade,
    State(rooms_db): State<RoomsDb>,
    Path(room_id): Path<String>,
) -> impl IntoResponse {
    let room = rooms_db.get_room(&RoomId::new(room_id));

    match room {
        Ok(room) => ws
            .on_failed_upgrade(|e| {
                let message = format!("error: [{:?}]", e);
                event!(Level::WARN, message)
            })
            .on_upgrade(|socket| websocket_task(socket, room)),
        Err(_e) => StatusCode::NOT_FOUND.into_response(),
    }
}
