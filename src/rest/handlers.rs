use axum::response::Html;

use crate::log;

pub async fn index() -> Html<&'static str> {
    log::debug!("index.html");
    // Include an utf-8 file contents as `&'static str` in compile time.
    // This macro is relative to current `handlers.rs` file.
    Html(include_str!("index.html"))
}
