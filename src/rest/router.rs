use axum::{middleware, routing::get, Router};
use tower_http::cors::CorsLayer;

use super::handlers;
use super::layers;

pub fn new() -> Router {
    let api = Router::new()
        .route("/act", get(handlers::act))
        .route("/player", get(handlers::player))
        .layer(CorsLayer::permissive())
        .layer(middleware::from_fn(layers::auth));

    Router::new()
        .route("/", get(handlers::index))
        .nest("/api", api)
}
