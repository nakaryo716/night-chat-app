use crate::utility::acquire_lock;
use axum::extract::ws::Message;
use axum::extract::ws::WebSocket;
use futures::{SinkExt, StreamExt};
use tracing::warn;

use super::rooms::RoomsDb;

pub async fn websocket_task(socket: WebSocket, room_db: RoomsDb, room_id: String) {
    let (mut sender, mut receiver) = socket.split();

    let tx = match acquire_lock(&room_db.pool_ref()).and_then(|guard| guard.get(&room_id).cloned())
    {
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

    let mut rx =
        match acquire_lock(&room_db.pool_ref()).and_then(|guard| guard.get(&room_id).cloned()) {
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
