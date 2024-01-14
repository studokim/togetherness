use axum::extract::State;
use axum::Json;

use crate::log;
use crate::rest::shared_state::SharedState;
use crate::rest::types;

pub async fn get_start(State(state): State<SharedState>) -> Json<types::DefaultResponse> {
    log::debug!("Game started");
    Json(types::DefaultResponse {
        ok: true,
        error: types::Error::None,
    })
}

pub async fn get_stats(State(state): State<SharedState>) -> Json<types::StatsResponse> {
    log::debug!("Returning stats");
    match state.read() {
        Ok(state) => match state.get_action_stats() {
            crate::state::GetActionStatsResult::Ok(stats) => Json(types::StatsResponse {
                hugs: stats[0].1,
                eavesdrops: stats[1].1,
                blackmails: stats[2].1,
                gossips: stats[3].1,
                crimes: stats[4].1,
                error: types::Error::None,
            }),
        },
        Err(err) => Json(types::StatsResponse {
            hugs: 0,
            eavesdrops: 0,
            blackmails: 0,
            gossips: 0,
            crimes: 0,
            error: types::Error::MultiThread(err.to_string()),
        }),
    }
}
