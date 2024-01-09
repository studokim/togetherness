use axum::{extract::Request, middleware::Next, response::Response};

use crate::log;

pub async fn auth(request: Request, next: Next) -> Response {
    // do something with `request`...
    log::debug!("authorized access to {}", request.uri());

    let response = next.run(request).await;
    // do something with `response`...
    response
}
