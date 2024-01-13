use axum::{
    http::{header, HeaderValue},
    response::{Html, IntoResponse},
};

use crate::log;

// Files below are included in compile time.
// The macro is relative to current `html.rs` file.

pub async fn api() -> Html<&'static str> {
    log::debug!("api.html");
    Html(include_str!("api.html"))
}

pub async fn not_found() -> Html<&'static str> {
    log::debug!("not_found.html");
    Html(include_str!("not_found.html"))
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