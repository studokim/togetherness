use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};

use crate::log;
use crate::rest::shared_state::AdminState;
use crate::static_server::html::auth;

pub async fn admin_auth(State(state): State<AdminState>, request: Request, next: Next) -> Response {
    // do something with `request`...
    let password_from_cookies = if request.headers().contains_key("Cookie") {
        match request.headers()["Cookie"].to_str() {
            Ok(cookie) => match cookie.find("password=") {
                Some(begin) => {
                    let shift = "password=".len();
                    match &cookie[begin + shift..].find(";") {
                        Some(end) => Some(&cookie[begin + shift..*end + shift]),
                        None => Some(&cookie[begin + shift..]),
                    }
                }
                None => None,
            },
            Err(_) => None,
        }
    } else {
        None
    };
    let authorized = match state.read() {
        Ok(state) => match password_from_cookies {
            Some(password) => {
                log::debug!(
                    "Password in cookies: `{}`, correct: `{}`",
                    password,
                    state.password
                );
                if password == state.password {
                    true
                } else {
                    false
                }
            }
            None => false,
        },
        Err(_) => {
            log::debug!("Error reading state");
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };
    if authorized {
        log::debug!("authorized access to `{}`", request.uri());
        let response = next.run(request).await;
        // do something with `response`...
        response
    } else {
        log::debug!("denied access to `{}`", request.uri());
        auth().await.into_response()
    }
}
