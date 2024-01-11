use axum::Json;

use crate::db::repository;
use crate::log;
use crate::rest::types;

pub async fn get_startgame() -> Json<types::DefaultResponse> {
    let repo = repository::Repository::new();
    log::debug!("Game started");
    Json(types::DefaultResponse {
        ok: true,
        timer: repo.get_timer().remaining(),
        error: types::Error::None,
    })
}
