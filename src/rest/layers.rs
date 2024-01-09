use axum::{extract::Request, http::Method, middleware::Next, response::Response};
use tower_http::cors::{Any, CorsLayer};

use crate::log;

pub async fn auth(request: Request, next: Next) -> Response {
    // do something with `request`...
    log::debug!("authorized access to {}", request.uri());

    let response = next.run(request).await;
    // do something with `response`...
    response
}

pub fn cors() -> CorsLayer {
    CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST])
        // allow requests from any origin
        .allow_origin(Any)
        .allow_headers(Any)
}
