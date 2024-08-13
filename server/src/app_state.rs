use std::{collections::HashMap, sync::Mutex};

use crate::{auth::UserDataDb, rooms::Room};

#[derive(Debug)]
pub struct AppState {
    pub rooms: Mutex<HashMap<String, Room>>,
    pub users_pool: UserDataDb,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            rooms: Mutex::default(),
            users_pool: UserDataDb::new()
        }
    }
}
