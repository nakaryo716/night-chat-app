use crate::{middleware::auth::UserDataDb, models::rooms::RoomsDb};

#[derive(Debug, Clone)]
pub struct AppState {
    pub rooms_db: RoomsDb,
    pub users_pool: UserDataDb,
}

impl AppState {
    pub fn new(rooms_db: RoomsDb, users_pool: UserDataDb) -> Self {
        Self {
            rooms_db,
            users_pool,
        }
    }
}
