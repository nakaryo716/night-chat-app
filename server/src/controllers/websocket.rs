use axum::extract::{Path, State, WebSocketUpgrade};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum_extra::extract::CookieJar;
use tracing::{event, Level};

use crate::models::rooms::{RoomId, RoomsDb};
use crate::models::user_name::get_user_name_from_cookie;
use crate::models::websocket::websocket_task;

pub async fn websocket_upgrade_handler(
    ws: WebSocketUpgrade,
    State(rooms_db): State<RoomsDb>,
    Path(room_id): Path<String>,
    jar: CookieJar,
) -> impl IntoResponse {
    let room = rooms_db.get_room(&RoomId::new(room_id));

    let user_name = match get_user_name_from_cookie(jar) {
        Some(cookie_val) => cookie_val,
        None => return StatusCode::UNAUTHORIZED.into_response(),
    };

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
