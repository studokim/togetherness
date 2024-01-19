use axum::{
    extract::State,
    http::{header, HeaderValue},
    response::{Html, IntoResponse},
};
use minijinja::render;

use crate::{log, rest::shared_state::SharedState, static_server::template};

// Files below are included in compile time.
// The macro is relative to current `html.rs` file.

pub async fn api() -> Html<&'static str> {
    log::debug!("api.html");
    Html(include_str!("api.html"))
}

pub async fn admin(State(state): State<SharedState>) -> Html<String> {
    log::debug!("admin.html");
    let html = include_str!("admin.html");
    match state.read() {
        Ok(state) => {
            let timer = template::Timer::new(&state.timer);
            log::debug!("Timer ok: {}:{}", timer.minutes, timer.seconds);
            Html(render!(html, timer => timer))
        }
        Err(_) => {
            log::debug!("Timer not ok");
            Html(html.to_string())
        }
    }
}

pub async fn favicon() -> impl IntoResponse {
    log::debug!("favicon.ico");
    let ico = include_bytes!("favicon.ico");
    (
        [(
            header::CONTENT_TYPE,
            HeaderValue::from_static("image/x-icon"),
        )],
        ico,
    )
}
