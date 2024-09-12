use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::models::rooms::{CreateRoom, RoomId, RoomInfo, RoomsDb};

pub async fn create_room_handler(
    State(rooms_db): State<RoomsDb>,
    Json(payload): Json<CreateRoom>,
) -> Result<impl IntoResponse, StatusCode> {
    let new_room = rooms_db
        .create_room(payload)
        .map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((StatusCode::OK, Json(RoomInfo::new(new_room))))
}

pub async fn room_list_handler(
    State(rooms_db): State<RoomsDb>,
) -> Result<impl IntoResponse, StatusCode> {
    let rooms = rooms_db
        .get_all_room_info()
        .map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((StatusCode::OK, Json(rooms)))
}

pub async fn delete_room_handler(
    State(rooms_db): State<RoomsDb>,
    Path(room_id_url): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    let room_id = RoomId::new(room_id_url);
    rooms_db
        .delete_room(room_id)
        .map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::NO_CONTENT)
}
