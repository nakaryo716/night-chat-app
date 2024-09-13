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

#[derive(Debug, Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct RoomId(String);

impl RoomId {
    pub fn new(id: String) -> Self {
        RoomId(id)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomName(String);

#[derive(Debug, Clone, Deserialize)]
pub struct CreateRoom {
    room_name: RoomName,
}

#[derive(Debug, Clone)]
pub struct Room {
    room_id: RoomId,
    room_name: RoomName,
    sender: Sender<String>,
}

impl Room {
    pub fn new(create_room: CreateRoom) -> Self {
        let room_id = Uuid::new_v4().to_string();
        Self {
            room_id: RoomId(room_id),
            room_name: create_room.room_name,
            sender: Sender::new(128),
        }
    }

    pub fn get_room_id(&self) -> &RoomId {
        &self.room_id
    }

    pub fn get_room_name(&self) -> &RoomName {
        &self.room_name
    }

    pub fn get_sender(&self) -> Sender<String> {
        self.sender.clone()
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct RoomInfo {
    room_id: RoomId,
    room_name: RoomName,
}

impl RoomInfo {
    pub fn new(room: Room) -> Self {
        Self {
            room_id: room.get_room_id().to_owned(),
            room_name: room.get_room_name().to_owned(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct RoomsDb {
    pool: Arc<Mutex<HashMap<RoomId, Room>>>,
}

impl RoomsDb {
    pub fn new() -> Self {
        Self {
            pool: Arc::default(),
        }
    }

    pub fn get_room(&self, room_id: &RoomId) -> Result<Room, RoomError> {
        let lock = self.pool.lock().map_err(|_e| RoomError::LockError)?;
        let room = lock.get(room_id).ok_or(RoomError::NotFound)?.to_owned();
        Ok(room)
    }

    pub fn create_room(&self, room_payload: CreateRoom) -> Result<Room, RoomError> {
        let new_room = Room::new(room_payload);

        let mut lock = self.pool.lock().map_err(|_e| RoomError::LockError)?;
        lock.insert(new_room.get_room_id().to_owned(), new_room.clone());

        Ok(new_room)
    }

    pub fn get_all_room_info(&self) -> Result<Vec<RoomInfo>, RoomError> {
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

    pub fn delete_room(&self, room_id: RoomId) -> Result<(), RoomError> {
        self.pool
            .lock()
            .map_err(|_e| RoomError::LockError)?
            .remove(&room_id);
        Ok(())
    }

    // pub fn delete_all_room(&self) -> Result<(), RoomError> {
    //     let mut lock = self.pool.lock().map_err(|_e| RoomError::LockError)?;

    //     let all_room_id: Vec<RoomId> = lock.iter().map(|(id, _room)| id.to_owned()).collect();

    //     all_room_id.iter().for_each(|e| {
    //         lock.remove(e);
    //     });
    //     Ok(())
    // }
}

impl FromRef<AppState> for RoomsDb {
    fn from_ref(input: &AppState) -> Self {
        input.rooms_db.clone()
    }
}

#[derive(Debug, Clone, Error)]
pub enum RoomError {
    #[error("room not found")]
    NotFound,
    #[error("mutex lock error")]
    LockError,
}
