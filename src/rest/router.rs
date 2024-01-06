use axum::{routing::get, Router};

use super::handlers;

pub fn new() -> Router {
    Router::new().route("/", get(handlers::index))
}
