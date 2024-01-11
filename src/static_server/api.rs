use axum::response::Html;

use crate::log;

pub async fn index() -> Html<&'static str> {
    log::debug!("api.html");
    // Include an utf-8 file contents as `&'static str` in compile time.
    // This macro is relative to current `api.rs` file.
    Html(include_str!("api.html"))
}
