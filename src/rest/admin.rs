use axum::Json;

use crate::log;
use crate::rest::types;

pub async fn get_startgame() -> Json<types::DefaultResponse> {
    log::debug!("Game started");
    Json(types::DefaultResponse {
        ok: true,
        error: types::Error::None,
    })
}
