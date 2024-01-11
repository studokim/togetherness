use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use tower_http::cors::CorsLayer;

use crate::rest;
use crate::static_server;

pub fn new() -> Router {
    let api = Router::new()
        .route("/player", post(rest::api::post_player))
        .route("/player/:id", get(rest::api::get_player))
        .route("/action", post(rest::api::post_action))
        .route("/action/:id", get(rest::api::get_action))
        .route("/gold/:id", get(rest::api::get_gold))
        .layer(CorsLayer::permissive())
        .layer(middleware::from_fn(rest::layers::auth));

    let admin = Router::new()
        .route("/startgame", get(rest::admin::get_startgame))
        .layer(CorsLayer::permissive())
        .layer(middleware::from_fn(rest::layers::auth));

    Router::new()
        .route("/api", get(static_server::api::index))
        .nest("/api", api)
        .nest("/admin", admin)
}
