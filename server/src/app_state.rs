use std::{collections::HashMap, sync::Mutex};

use crate::rooms::Room;

#[derive(Debug)]
pub struct AppState {
    pub rooms: Mutex<HashMap<String, Room>>,
    pub users_pool: Mutex<HashMap<i32, String>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            rooms: Mutex::default(),
            users_pool: Mutex::default(),
        }
    }
}
