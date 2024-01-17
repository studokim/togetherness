use axum::extract::State;
use axum::Json;

use crate::log;
use crate::model::ActionType;
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
        Ok(state) => Json(types::StatsResponse {
            hug: state.count_actions_of_type(ActionType::Hug),
            eavesdropping: state.count_actions_of_type(ActionType::Eavesdropping),
            blackmail: state.count_actions_of_type(ActionType::Blackmail),
            gossip: state.count_actions_of_type(ActionType::Gossip),
            crime: state.count_actions_of_type(ActionType::Crime),
            error: types::Error::None,
        }),
        Err(err) => Json(types::StatsResponse::default(types::Error::MultiThread(
            err.to_string(),
        ))),
    }
}
