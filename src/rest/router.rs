use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use tower_http::cors::CorsLayer;

use super::handlers::{admin, api, html};
use super::layers;

pub fn new() -> Router {
    let api = Router::new()
        .route("/player", post(api::post_player))
        .route("/player/:id", get(api::get_player))
        .route("/action", post(api::post_action))
        .route("/action/:id", get(api::get_action))
        .route("/gold/:id", get(api::get_gold))
        .layer(CorsLayer::permissive())
        .layer(middleware::from_fn(layers::auth));

    let admin = Router::new()
        .route("/startgame", get(admin::startgame))
        .layer(CorsLayer::permissive())
        .layer(middleware::from_fn(layers::auth));

    Router::new()
        .route("/", get(html::index))
        .nest("/api", api)
        .nest("/admin", admin)
}
