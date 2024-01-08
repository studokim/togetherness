use axum::{response::Html, Json};

use crate::log;
use crate::model;

pub async fn index() -> Html<&'static str> {
    log::debug!("index.html");
    // Include an utf-8 file contents as `&'static str` in compile time.
    // This macro is relative to current `handlers.rs` file.
    Html(include_str!("index.html"))
}

pub async fn act() -> Json<String> {
    Json("acted".to_string())
}

pub async fn player() -> Json<model::Player> {
    Json(model::Player {
        name: "John".to_string(),
        party: 9,
    })
}
