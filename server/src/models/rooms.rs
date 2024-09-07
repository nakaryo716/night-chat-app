use axum::extract::FromRef;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use thiserror::Error;
use tokio::sync::broadcast::Sender;
use uuid::Uuid;

use crate::app::AppState;

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

#[derive(Debug, Clone)]
pub struct RoomsDb {
    pool: Arc<Mutex<HashMap<String, Room>>>,
}

impl RoomsDb {
    pub fn new() -> Self {
        Self {
            pool: Arc::default(),
        }
    }

    pub fn pool_ref(&self) -> Arc<Mutex<HashMap<String, Room>>> {
        Arc::clone(&self.pool)
    }
}

impl FromRef<AppState> for RoomsDb {
    fn from_ref(input: &AppState) -> Self {
        input.rooms_db.clone()
    }
}

pub trait RoomsManage {
    fn create_room(&self, payload: CreateRoom) -> Result<Room, RoomError>;
    fn get_all_room(&self) -> Result<Vec<RoomInfo>, RoomError>;
    fn delete_room(&self, room_id: String) -> Result<(), RoomError>;
}

#[derive(Debug, Clone, Error)]
pub enum RoomError {
    #[error("room not found")]
    NotFound,
    #[error("mutex lock error")]
    LockError,
}

impl RoomsManage for RoomsDb {
    fn create_room(&self, room_payload: CreateRoom) -> Result<Room, RoomError> {
        let new_room = Room::new(room_payload);

        let mut lock = self.pool.lock().map_err(|_e| RoomError::LockError)?;
        lock.insert(new_room.get_room_id().to_string(), new_room.clone());

        Ok(new_room)
    }

    fn get_all_room(&self) -> Result<Vec<RoomInfo>, RoomError> {
        let rooms: Vec<_>;
        {
            let lock = self.pool.lock().map_err(|_e| RoomError::LockError)?;

            rooms = lock
                .iter()
                .map(|(_id, room)| RoomInfo::new(room.to_owned()))
                .collect();
        }
        Ok(rooms)
    }

    fn delete_room(&self, room_id: String) -> Result<(), RoomError> {
        self.pool
            .lock()
            .map_err(|_e| RoomError::LockError)?
            .remove(&room_id);

        Ok(())
    }
}
