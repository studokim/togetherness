use axum::extract::State;
use axum::{extract::Path, Json};

use crate::rest::state::SharedState;
use crate::rest::types;
use crate::{log, model};

pub async fn get_timer(State(state): State<SharedState>) -> Json<types::TimerResponse> {
    log::debug!("Returning timer");
    match state.read() {
        Ok(state) => match state.timer.remaining() {
            Some(seconds) => Json(types::TimerResponse {
                seconds: Some(seconds),
                error: types::Error::None,
            }),
            None => Json(types::TimerResponse {
                seconds: None,
                error: types::Error::NotSet,
            }),
        },
        Err(err) => Json(types::TimerResponse {
            seconds: None,
            error: types::Error::MultiThread(err.to_string()),
        }),
    }
}

pub async fn post_player(
    State(state): State<SharedState>,
    player: Json<types::PlayerRequest>,
) -> Json<types::DefaultResponse> {
    log::debug!("Registering player: name={}, id={}", player.name, player.id);
    let player = model::Player {
        id: player.id.clone(),
        name: player.name.clone(),
        avatar_id: player.avatar_id,
        faction_id: player.faction_id,
        gold: 0,
    };
    match state.write() {
        Ok(mut state) => match state.register(player) {
            crate::state::Error::None => Json(types::DefaultResponse {
                ok: true,
                error: types::Error::None,
            }),
            crate::state::Error::AlreadyExists => Json(types::DefaultResponse {
                ok: false,
                error: types::Error::AlreadyExists,
            }),
        },
        Err(err) => Json(types::DefaultResponse {
            ok: false,
            error: types::Error::MultiThread(err.to_string()),
        }),
    }
}

pub async fn get_player(
    State(state): State<SharedState>,
    Path(id): Path<String>,
) -> Json<types::PlayerResponse> {
    log::debug!("Returning player: id={}", id);
    match state.read() {
        Ok(state) => match state.get_player(&id) {
            Some(player) => Json(types::PlayerResponse {
                player: Some(player),
                error: types::Error::None,
            }),
            None => Json(types::PlayerResponse {
                player: None,
                error: types::Error::NotFound,
            }),
        },
        Err(err) => {
            return Json(types::PlayerResponse {
                player: None,
                error: types::Error::MultiThread(err.to_string()),
            })
        }
    }
}

pub async fn post_action(
    State(state): State<SharedState>,
    action: Json<types::ActionRequest>,
) -> Json<types::DefaultResponse> {
    log::debug!(
        "Making action: id={}, subject={}, object={}",
        action.action_id,
        action.subject_id,
        action.object_id
    );
    Json(types::DefaultResponse {
        ok: true,
        error: types::Error::None,
    })
}

pub async fn get_action(
    State(state): State<SharedState>,
    Path(id): Path<String>,
) -> Json<types::ActionResponse> {
    log::debug!("Returning actions of player: id={}", id);
    Json(types::ActionResponse {
        actions: Some(vec![
            types::ActionsCount {
                action_id: types::ActionId::from(model::ActionType::Blackmail),
                as_subject: 8,
                as_object: 19,
            },
            types::ActionsCount {
                action_id: types::ActionId::from(model::ActionType::Gossip),
                as_subject: 5,
                as_object: 2,
            },
        ]),
        error: types::Error::None,
    })
}

pub async fn get_gold(
    State(state): State<SharedState>,
    Path(id): Path<String>,
) -> Json<types::GoldResponse> {
    log::debug!("Returning gold of player: id={}", id);
    Json(types::GoldResponse {
        gold: None,
        error: types::Error::NotFound,
    })
}
