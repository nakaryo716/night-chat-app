use axum::routing::{delete, post};
use axum::{routing::get, Router};
use http::header::{ACCESS_CONTROL_ALLOW_CREDENTIALS, ACCESS_CONTROL_ALLOW_ORIGIN, CONTENT_TYPE};
use http::Method;
use tower_http::cors::CorsLayer;

use crate::app::AppState;
use crate::controllers::rooms::{create_room_handler, delete_room_handler, get_room_info_handler, room_list_handler};
use crate::controllers::user_name::{
    delete_user_name_from_cookie_handler, get_user_name_from_cookie_handler,
    register_user_name_as_cookie_handler,
};
use crate::controllers::websocket::websocket_upgrade_handler;
use crate::middleware::time_limit::time_limit_check;

const ORIGINURL: &str = "http://localhost:5173";
const ORIGINUR2: &str = "http://127.0.0.1:5173";

pub fn app(state: AppState) -> Router {
    Router::new()
        // ------Set User name------
        // METHOD : POST
        // URI    : 'http://localhost:3000/user_name'
        // PAYLOAD:
        // {
        //    "user_name": "user name here"
        // }
        //
        // ------Get User name------
        // METHOD : GET
        // URI    : 'http://localhost:3000/user_name'
        .route(
            "/user_name",
            post(register_user_name_as_cookie_handler)
                .get(get_user_name_from_cookie_handler)
                .delete(delete_user_name_from_cookie_handler),
        )
        //
        //
        // ------Create room------
        // METHOD : POST
        // URI    : 'http://localhost:3000/create_room'
        // PAYLOAD:
        // {
        //    "room_name": "room name here"
        // }
        .route("/create_room", post(create_room_handler))
        //
        //
        // ------Get room infomation------
        // METHOD : GET
        // URI    : 'http://localhost:3000/room/:room_id'
        .route("/room/:room_id", get(get_room_info_handler))
        //
        //
        // ------Get all room infomation------
        // METHOD : GET
        // URI    : 'http://localhost:3000/room_ls/'
        .route("/room_ls", get(room_list_handler))
        //
        //
        .route("/delete_room/:room_id", delete(delete_room_handler))
        .route("/websocket/:room_id", get(websocket_upgrade_handler))
        //
        //
        // Application State 
        .with_state(state)
        //
        // time limit layer  
        .route_layer(axum::middleware::from_fn(time_limit_check))
        //
        // cors layer
        .layer(
            CorsLayer::new()
                .allow_origin([ORIGINURL.parse().unwrap(), ORIGINUR2.parse().unwrap()])
                .allow_headers([
                    CONTENT_TYPE,
                    ACCESS_CONTROL_ALLOW_ORIGIN,
                    ACCESS_CONTROL_ALLOW_CREDENTIALS,
                ])
                .allow_methods([
                    Method::POST,
                    Method::GET,
                    Method::PUT,
                    Method::DELETE,
                    Method::OPTIONS,
                ])
                .allow_credentials(true),
        )
}
