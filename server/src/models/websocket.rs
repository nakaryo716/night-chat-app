use axum::extract::ws::Message;
use axum::extract::ws::WebSocket;
use futures::{SinkExt, StreamExt};
use tracing::warn;

use super::rooms::Room;

pub async fn websocket_task(socket: WebSocket, room: Room) {
    let (mut ws_sender, mut ws_receiver) = socket.split();

    let tx = room.get_sender();
    let tx_clone = tx.clone();
    let mut receive_task = tokio::task::spawn(async move {
        while let Some(Ok(Message::Text(text))) = ws_receiver.next().await {
            if let Err(e) = tx_clone.send(text) {
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
