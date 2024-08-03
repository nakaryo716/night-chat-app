use crate::app_state::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::broadcast::Sender;
use uuid::Uuid;

pub async fn create_room_handler(
    State(app_state): State<Arc<AppState>>,
    Json(payload): Json<CreateRoom>,
) -> Result<impl IntoResponse, StatusCode> {
    let new_room = Room::new(payload);

    let mut lock = app_state
        .rooms
        .lock()
        .map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)?;
    lock.insert(new_room.get_room_id().to_string(), new_room.clone());

    Ok((StatusCode::OK, Json(RoomInfo::new(new_room))))
}

pub async fn room_list_handler(
    State(app_state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, StatusCode> {
    let data: Vec<_>;
    {
        let  lock = app_state
            .rooms
            .lock()
            .map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)?;

        data = lock
            .iter()
            .map(|(_id, room)| RoomInfo::new(room.to_owned()))
            .collect();
    }

    Ok((StatusCode::OK, Json(data)))
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateRoom {
    room_name: String,
    room_time: u32,
}

#[derive(Debug, Clone)]
pub struct Room {
    room_id: String,
    room_name: String,
    room_time: u32,
    sender: Sender<String>,
}

impl Room {
    pub fn new(create_room: CreateRoom) -> Self {
        let room_id = Uuid::new_v4().to_string();
        Self {
            room_id,
            room_name: create_room.room_name,
            room_time: create_room.room_time,
            sender: Sender::new(128),
        }
    }

    pub fn get_room_id(&self) -> &str {
        &self.room_id
    }

    pub fn get_room_name(&self) -> &str {
        &self.room_name
    }

    pub fn get_room_time(&self) -> u32 {
        self.room_time
    }

    pub fn get_sender(&self) -> Sender<String> {
        self.sender.clone()
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct RoomInfo {
    room_id: String,
    room_name: String,
    room_time: u32,
}

impl RoomInfo {
    pub fn new(room: Room) -> Self {
        Self {
            room_id: room.get_room_id().to_string(),
            room_name: room.get_room_name().to_string(),
            room_time: room.get_room_time(),
        }
    }
}
