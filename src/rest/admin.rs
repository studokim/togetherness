use axum::extract::State;
use axum::Json;
use time::Duration;

use crate::log;
use crate::model::ActionType;
use crate::rest::shared_state::AppState;
use crate::rest::types;
use crate::timer::*;

pub async fn post_start(State(state): State<AppState>) -> Json<types::DefaultResponse> {
    log::debug!("Game started");
    match state.write() {
        Ok(mut state) => match state.timer.start() {
            StartTimerResult::Ok => Json(types::DefaultResponse {
                ok: true,
                error: types::Error::None,
            }),
            StartTimerResult::AlreadyStarted => Json(types::DefaultResponse {
                ok: false,
                error: types::Error::AlreadyStarted,
            }),
            StartTimerResult::SetToZero => Json(types::DefaultResponse {
                ok: false,
                error: types::Error::SetToZero,
            }),
        },
        Err(err) => {
            log::debug!("Error::MultiThread: {}", err);
            Json(types::DefaultResponse {
                ok: false,
                error: types::Error::MultiThread,
            })
        }
    }
}

pub async fn post_stop(State(state): State<AppState>) -> Json<types::DefaultResponse> {
    log::debug!("Game stopped");
    match state.write() {
        Ok(mut state) => match state.timer.stop() {
            StopTimerResult::Ok => Json(types::DefaultResponse {
                ok: true,
                error: types::Error::None,
            }),
            StopTimerResult::NotStarted => Json(types::DefaultResponse {
                ok: false,
                error: types::Error::NotStarted,
            }),
        },
        Err(err) => {
            log::debug!("Error::MultiThread: {}", err);
            Json(types::DefaultResponse {
                ok: false,
                error: types::Error::MultiThread,
            })
        }
    }
}

pub async fn post_reset(State(state): State<AppState>) -> Json<types::DefaultResponse> {
    log::debug!("Game reset");
    match state.write() {
        Ok(mut state) => match state.timer.get() {
            GetTimerResult::NotStarted | GetTimerResult::Expired => {
                state.reset();
                Json(types::DefaultResponse {
                    ok: true,
                    error: types::Error::None,
                })
            }
            GetTimerResult::Remaining(_) => Json(types::DefaultResponse {
                ok: false,
                error: types::Error::AlreadyStarted,
            }),
        },
        Err(err) => {
            log::debug!("Error::MultiThread: {}", err);
            Json(types::DefaultResponse {
                ok: false,
                error: types::Error::MultiThread,
            })
        }
    }
}

pub async fn post_repeated_actions(
    State(state): State<AppState>,
    Json(repeated_actions): Json<bool>,
) -> Json<types::DefaultResponse> {
    log::debug!("Modifying repeated_actions setting");
    match state.write() {
        Ok(mut state) => {
            if repeated_actions {
                state.allow_repeated_actions();
            } else {
                state.forbid_repeated_actions();
            }
            Json(types::DefaultResponse {
                ok: true,
                error: types::Error::None,
            })
        }
        Err(err) => {
            log::debug!("Error::MultiThread: {}", err);
            Json(types::DefaultResponse {
                ok: false,
                error: types::Error::MultiThread,
            })
        }
    }
}

pub async fn post_duration(
    State(state): State<AppState>,
    Json(minutes): Json<crate::timer::Seconds>,
) -> Json<types::DefaultResponse> {
    log::debug!("Set game duration={}", minutes);
    match state.write() {
        Ok(mut state) => match state.timer.set(Duration::minutes(minutes)) {
            SetTimerResult::Ok => Json(types::DefaultResponse {
                ok: true,
                error: types::Error::None,
            }),
            SetTimerResult::AlreadyStarted => Json(types::DefaultResponse {
                ok: false,
                error: types::Error::AlreadyStarted,
            }),
        },
        Err(err) => {
            log::debug!("Error::MultiThread: {}", err);
            Json(types::DefaultResponse {
                ok: false,
                error: types::Error::MultiThread,
            })
        }
    }
}

pub async fn get_stats(State(state): State<AppState>) -> Json<types::StatsResponse> {
    log::debug!("Returning stats");
    match state.read() {
        Ok(state) => Json(types::StatsResponse {
            hug: state.count_actions(None, None, Some(ActionType::Hug)),
            stealing: state.count_actions(None, None, Some(ActionType::Stealing)),
            blackmail: state.count_actions(None, None, Some(ActionType::Blackmail)),
            bribery: state.count_actions(None, None, Some(ActionType::Bribery)),
            lobbying: state.count_actions(None, None, Some(ActionType::Lobbying)),
            error: types::Error::None,
        }),
        Err(err) => {
            log::debug!("Error::MultiThread: {}", err);
            Json(types::StatsResponse::default(types::Error::MultiThread))
        }
    }
}
