use crate::models::rooms::RoomsDb;

#[derive(Debug, Clone)]
pub struct AppState {
    pub rooms_db: RoomsDb,
}

impl AppState {
    pub fn new(rooms_db: RoomsDb) -> Self {
        Self { rooms_db }
    }
}
