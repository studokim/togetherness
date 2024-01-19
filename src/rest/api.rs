use axum::extract::{Query, State};
use axum::{extract::Path, Json};

use crate::rest::shared_state::SharedState;
use crate::rest::types;
use crate::state::{ActResult, GetPlayerResult, RegisterResult};
use crate::timer::GetTimerResult;
use crate::{log, model};

pub async fn get_timer(State(state): State<SharedState>) -> Json<types::TimerResponse> {
    // log::debug!("Returning timer");
    match state.read() {
        Ok(state) => match state.timer.get() {
            GetTimerResult::Remaining(seconds) => Json(types::TimerResponse {
                seconds: Some(seconds),
                error: types::Error::None,
            }),
            GetTimerResult::NotStarted => Json(types::TimerResponse {
                seconds: None,
                error: types::Error::NotStarted,
            }),
            GetTimerResult::Expired => Json(types::TimerResponse {
                seconds: None,
                error: types::Error::AlreadyFinished,
            }),
        },
        Err(err) => {
            log::debug!("Error::MultiThread: {}", err);
            Json(types::TimerResponse {
                seconds: None,
                error: types::Error::MultiThread,
            })
        }
    }
}

pub async fn post_player(
    State(state): State<SharedState>,
    player: Json<types::PostPlayerRequest>,
) -> Json<types::DefaultResponse> {
    log::debug!("Registering player: name={}, id={}", player.name, player.id);
    let player = model::Player::new(
        &player.id,
        &player.name,
        player.avatar_id,
        player.faction_id,
    );
    match state.write() {
        Ok(mut state) => match state.register(player) {
            RegisterResult::Ok => Json(types::DefaultResponse {
                ok: true,
                error: types::Error::None,
            }),
            RegisterResult::AlreadyExists => Json(types::DefaultResponse {
                ok: false,
                error: types::Error::PlayerAlreadyExists,
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

pub async fn get_player(
    State(state): State<SharedState>,
    Path(id): Path<String>,
) -> Json<types::PlayerResponse> {
    log::debug!("Returning player: id={}", id);
    match state.read() {
        Ok(state) => match state.get_player(&id) {
            GetPlayerResult::Ok(player) => Json(types::PlayerResponse {
                player: Some(player),
                error: types::Error::None,
            }),
            GetPlayerResult::NotFound => Json(types::PlayerResponse {
                player: None,
                error: types::Error::PlayerNotFound,
            }),
        },
        Err(err) => {
            log::debug!("Error::MultiThread: {}", err);
            Json(types::PlayerResponse {
                player: None,
                error: types::Error::MultiThread,
            })
        }
    }
}

pub async fn post_action(
    State(state): State<SharedState>,
    action: Json<types::PostActionRequest>,
) -> Json<types::DefaultResponse> {
    log::debug!(
        "Making action: id={}, subject={}, object={}",
        action.action_id,
        action.subject_id,
        action.object_id
    );
    match state.read() {
        Ok(state) => match state.timer.get() {
            GetTimerResult::Remaining(_) => {}
            GetTimerResult::NotStarted => {
                return Json(types::DefaultResponse {
                    ok: false,
                    error: types::Error::NotStarted,
                })
            }
            GetTimerResult::Expired => {
                return Json(types::DefaultResponse {
                    ok: false,
                    error: types::Error::AlreadyFinished,
                })
            }
        },
        Err(err) => {
            log::debug!("Error::MultiThread: {}", err);
            return Json(types::DefaultResponse {
                ok: false,
                error: types::Error::MultiThread,
            });
        }
    };
    let action = model::Action {
        action: action.action_id.into(),
        object_id: action.object_id.clone(),
        subject_id: action.subject_id.clone(),
    };
    match state.write() {
        Ok(mut state) => {
            if state.count_actions(
                Some(action.subject_id.clone()),
                Some(action.object_id.clone()),
                Some(action.action.clone()),
            ) >= 1
            {
                return Json(types::DefaultResponse {
                    ok: false,
                    error: types::Error::AlreadyActed,
                });
            }
            match state.act(&action) {
                ActResult::Ok => Json(types::DefaultResponse {
                    ok: true,
                    error: types::Error::None,
                }),
                ActResult::SubjectNotFound => Json(types::DefaultResponse {
                    ok: false,
                    error: types::Error::SubjectNotFound,
                }),
                ActResult::ObjectNotFound => Json(types::DefaultResponse {
                    ok: false,
                    error: types::Error::ObjectNotFound,
                }),
            }
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

pub async fn get_action(
    State(state): State<SharedState>,
    filter: Query<types::GetActionRequest>,
) -> Json<types::ActionResponse> {
    log::debug!(
        "Returning filtered actions: subject_id={:?}, object_id={:?}, action_id={:?}",
        filter.subject_id,
        filter.object_id,
        filter.action_id
    );
    let action_type: Option<model::ActionType> = match filter.action_id {
        Some(action_id) => Some(action_id.into()),
        None => None,
    };
    match state.read() {
        Ok(state) => {
            let subject_id = match &filter.subject_id {
                Some(subject_id) => match state.get_player(&subject_id) {
                    GetPlayerResult::Ok(_) => Some(subject_id.clone()),
                    GetPlayerResult::NotFound => {
                        return Json(types::ActionResponse {
                            count: None,
                            error: types::Error::SubjectNotFound,
                        })
                    }
                },
                None => None,
            };
            let object_id = match &filter.object_id {
                Some(object_id) => match state.get_player(&object_id) {
                    GetPlayerResult::Ok(_) => Some(object_id.clone()),
                    GetPlayerResult::NotFound => {
                        return Json(types::ActionResponse {
                            count: None,
                            error: types::Error::ObjectNotFound,
                        })
                    }
                },
                None => None,
            };
            Json(types::ActionResponse {
                count: Some(state.count_actions(subject_id, object_id, action_type)),
                error: types::Error::None,
            })
        }
        Err(err) => {
            log::debug!("Error::MultiThread: {}", err);
            Json(types::ActionResponse {
                count: None,
                error: types::Error::MultiThread,
            })
        }
    }
}

pub async fn get_gold(
    State(state): State<SharedState>,
    Path(id): Path<String>,
) -> Json<types::GoldResponse> {
    // log::debug!("Returning gold of player: id={}", id);
    match state.read() {
        Ok(state) => match state.get_player(&id) {
            GetPlayerResult::Ok(player) => Json(types::GoldResponse {
                gold: Some(player.gold),
                error: types::Error::None,
            }),
            GetPlayerResult::NotFound => Json(types::GoldResponse {
                gold: None,
                error: types::Error::PlayerNotFound,
            }),
        },
        Err(err) => {
            log::debug!("Error::MultiThread: {}", err);
            Json(types::GoldResponse {
                gold: None,
                error: types::Error::MultiThread,
            })
        }
    }
}

fn new_player_status_tuple(
    id: &model::PlayerId,
    action: model::ActionType,
    state: &crate::state::AppState,
) -> types::PlayerStatusTuple {
    types::PlayerStatusTuple {
        action_id: action.clone().into(),
        as_subject: state.count_actions(Some(id.clone()), None, Some(action.clone())),
        as_object: state.count_actions(None, Some(id.clone()), Some(action.clone())),
    }
}

fn new_player_status_response(
    id: &model::PlayerId,
    state: &crate::state::AppState,
) -> types::PlayerStatusResponse {
    types::PlayerStatusResponse {
        status: Some([
            new_player_status_tuple(id, model::ActionType::Hug, state),
            new_player_status_tuple(id, model::ActionType::Eavesdropping, state),
            new_player_status_tuple(id, model::ActionType::Blackmail, state),
            new_player_status_tuple(id, model::ActionType::Gossip, state),
            new_player_status_tuple(id, model::ActionType::Crime, state),
        ]),
        error: types::Error::None,
    }
}

pub async fn get_status(
    State(state): State<SharedState>,
    Path(player_id): Path<String>,
) -> Json<types::PlayerStatusResponse> {
    match state.read() {
        Ok(state) => {
            match state.get_player(&player_id) {
                GetPlayerResult::Ok(_) => {}
                GetPlayerResult::NotFound => {
                    return Json(types::PlayerStatusResponse {
                        status: None,
                        error: types::Error::PlayerNotFound,
                    })
                }
            };

            Json(new_player_status_response(&player_id, &state))
        }
        Err(err) => {
            log::debug!("Error::MultiThread: {}", err);
            Json(types::PlayerStatusResponse {
                status: None,
                error: types::Error::MultiThread,
            })
        }
    }
}
