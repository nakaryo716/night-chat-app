use axum::extract::ws::Message;
use axum::extract::ws::WebSocket;
use chrono::DateTime;
use chrono::Utc;
use futures::{SinkExt, StreamExt};
use serde::Serialize;
use tracing::warn;

use super::rooms::Room;

pub async fn websocket_task(socket: WebSocket, room: Room, user_name: String) {
    let (mut ws_sender, mut ws_receiver) = socket.split();

    let tx = room.get_sender();
    let tx_clone = tx.clone();
    let mut receive_task = tokio::task::spawn(async move {
        while let Some(Ok(Message::Text(text_msg))) = ws_receiver.next().await {
            let json_send_message = WsText::json_from_ws_message(&user_name, &text_msg);
            let Ok(serialized_send_message) = serde_json::to_string(&json_send_message) else {
                break;
            };

            if let Err(e) = tx_clone.send(serialized_send_message) {
                warn!("[error]{:?}", e);
                break;
            }
        }
    });

    let mut rx = tx.subscribe();
    let mut send_task = tokio::task::spawn(async move {
        while let Ok(text) = rx.recv().await {
            if let Err(e) = ws_sender.send(Message::Text(text)).await {
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

#[derive(Debug, Clone, Serialize)]
struct WsText {
    user_name: String,
    text: String,
    time_stamp: DateTime<Utc>,
}

impl WsText {
    pub fn json_from_ws_message(user_name: &str, text: &str) -> Self {
        WsText {
            user_name: user_name.to_string(),
            text: text.to_string(),
            time_stamp: Utc::now(),
        }
    }
}
