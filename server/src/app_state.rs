use crate::{auth::UserDataDb, rooms::RoomsDb};

#[derive(Debug, Clone)]
pub struct AppState {
    pub rooms: RoomsDb,
    pub users_pool: UserDataDb,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            rooms: RoomsDb::new(),
            users_pool: UserDataDb::new(),
        }
    }
}
