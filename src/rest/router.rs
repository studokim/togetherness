use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use tower_http::cors::CorsLayer;

use super::handlers;
use super::layers;

pub fn new() -> Router {
    let api = Router::new()
        .route("/player", post(handlers::post_player))
        .route("/player/:id", get(handlers::get_player))
        .route("/action", post(handlers::post_action))
        .route("/action/:id", get(handlers::get_action))
        .route("/gold/:id", get(handlers::get_gold))
        .layer(CorsLayer::permissive())
        .layer(middleware::from_fn(layers::auth));

    Router::new()
        .route("/", get(handlers::index))
        .nest("/api", api)
}
