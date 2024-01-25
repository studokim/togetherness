use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use tower_http::trace::TraceLayer;

use crate::rest;
use crate::rest::shared_state::{AdminState, AppState};
use crate::static_server;

pub fn new() -> Router {
    let state = AppState::default();
    let admin_state = AdminState::default();

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
                .with_state(state.clone()),
        );

    let admin = Router::new()
        .route("/admin", get(static_server::html::admin))
        .nest(
            "/admin/api",
            Router::new()
                .route("/duration", post(rest::admin::post_duration))
                .route("/start", post(rest::admin::post_start))
                .route("/stop", post(rest::admin::post_stop))
                .route("/reset", post(rest::admin::post_reset))
                .route(
                    "/repeated_actions",
                    post(rest::admin::post_repeated_actions),
                )
                .route("/stats", get(rest::admin::get_stats)),
        )
        .layer(middleware::from_fn_with_state(
            admin_state.clone(),
            rest::layers::admin_auth,
        ))
        .with_state(state.clone())
        .route("/admin/password", get(static_server::html::password));

    let react = static_server::react::assets();

    Router::new()
        .route("/favicon.ico", get(static_server::html::favicon))
        .merge(api)
        .merge(admin)
        .merge(react)
    // .layer(TraceLayer::new_for_http())
}
