use crate::app_state::AppState;
use axum::extract::ws::Message;
use axum::extract::ws::WebSocket;
use axum::extract::{Path, State, WebSocketUpgrade};
use axum::response::IntoResponse;
use futures::{SinkExt, StreamExt};
use std::sync::{Arc, Mutex, MutexGuard};
use tracing::{event, warn, Level};

pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(app_state): State<Arc<AppState>>,
    Path(room_id): Path<String>,
) -> impl IntoResponse {
    ws.on_failed_upgrade(|e| {
        let message = format!("error: [{:?}]", e);
        event!(Level::WARN, message)
    })
    .on_upgrade(|socket| websocket_task(socket, app_state, room_id))
}

fn acquire_lock<T>(mutex: &Mutex<T>) -> Option<MutexGuard<T>> {
    match mutex.lock() {
        Ok(a) => Some(a),
        Err(_e) => None,
    }
}

async fn websocket_task(socket: WebSocket, state: Arc<AppState>, room_id: String) {
    let (mut sender, mut receiver) = socket.split();

    let tx = match acquire_lock(&state.rooms).and_then(|guard| guard.get(&room_id).cloned()) {
        Some(room) => room.get_sender(),
        None => return,
    };

    let mut receive_task = tokio::task::spawn(async move {
        while let Some(Ok(Message::Text(text))) = receiver.next().await {
            if let Err(e) = tx.send(text) {
                warn!("[error]{:?}", e);
                break;
            }
        }
    });

    let mut rx = match acquire_lock(&state.rooms).and_then(|guard| guard.get(&room_id).cloned()) {
        Some(room) => room.get_sender().subscribe(),
        None => return,
    };

    let mut send_task = tokio::task::spawn(async move {
        while let Ok(text) = rx.recv().await {
            if let Err(e) = sender.send(Message::Text(text)).await {
                warn!("[error]:{:?}", e);
                break;
            }
        }
    });

    tokio::select! {
        _ = &mut send_task => send_task.abort(),
        _ = &mut receive_task => receive_task.abort(),
    }
}
