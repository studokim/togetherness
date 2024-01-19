use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use tower_http::trace::TraceLayer;

use crate::rest;
use crate::rest::shared_state::SharedState;
use crate::static_server;

pub fn new() -> Router {
    let state = SharedState::default();

    let api = Router::new()
        .route("/api", get(static_server::html::api))
        .nest(
            "/api",
            Router::new()
                .route("/timer", get(rest::api::get_timer))
                .route("/player", post(rest::api::post_player))
                .route("/player/:id", get(rest::api::get_player))
                .route("/action", post(rest::api::post_action))
                .route("/action", get(rest::api::get_action))
                .route("/gold/:id", get(rest::api::get_gold))
                .route("/status/:id", get(rest::api::get_status))
                .with_state(Arc::clone(&state)),
        );

    let admin = Router::new()
        .route("/admin", get(static_server::html::admin))
        .nest(
            "/api/admin",
            Router::new()
                .route("/duration", post(rest::admin::post_duration))
                .route("/start", post(rest::admin::post_start))
                .route("/stop", post(rest::admin::post_stop))
                .route("/stats", get(rest::admin::get_stats))
                .layer(middleware::from_fn(rest::layers::admin_auth)),
        )
        .with_state(Arc::clone(&state));

    let react = static_server::react::assets();

    Router::new()
        .route("/favicon.ico", get(static_server::html::favicon))
        .merge(api)
        .merge(admin)
        .merge(react)
    // .layer(TraceLayer::new_for_http())
}
