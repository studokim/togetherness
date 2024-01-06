use axum::response::Html;

pub async fn index() -> Html<&'static str> {
    // Include an utf-8 file contents as `&'static str` in compile time.
    // This macro is relative to current `handlers.rs` file.
    Html(include_str!("../../html/index.html"))
}
