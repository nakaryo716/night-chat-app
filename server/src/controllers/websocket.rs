use axum::extract::{Path, Query, State, WebSocketUpgrade};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use tracing::{event, Level};

use crate::models::rooms::{RoomId, RoomsDb};
use crate::models::user_name::UserNameForCoockie;
use crate::models::websocket::websocket_task;

pub async fn websocket_upgrade_handler(
    Path(room_id): Path<String>,
    Query(user_name): Query<UserNameForCoockie>,
    State(rooms_db): State<RoomsDb>,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    let user_name = user_name.get_user_name().to_string();
    let room = rooms_db.get_room(&RoomId::new(room_id));
    match room {
        Ok(room) => ws
            .on_failed_upgrade(|e| {
                let message = format!("error: [{:?}]", e);
                event!(Level::WARN, message)
            })
            .on_upgrade(move |socket| websocket_task(socket, room, user_name)),
        Err(_e) => StatusCode::NOT_FOUND.into_response(),
    }
}
