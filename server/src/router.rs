use std::sync::Arc;

use crate::rooms::{create_room_handler, room_list_handler};
use crate::websocket::websocket_handler;
use crate::{app_state::AppState, handlers::index};
use axum::routing::post;
use axum::{routing::get, Router};

pub fn app(state: AppState) -> Router {
    Router::new()
        .route("/", get(index))
        .route("/create_room", post(create_room_handler))
        .route("/room_ls", get(room_list_handler))
        .route("/websocket/:room_id", get(websocket_handler))
        .with_state(Arc::new(state))
}
