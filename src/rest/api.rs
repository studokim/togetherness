use axum::extract::State;
use axum::{extract::Path, Json};

use crate::rest::shared_state::SharedState;
use crate::rest::types;
use crate::state::{ActResult, GetActionsResult, GetPlayerResult, RegisterResult};
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
            GetPlayerResult::Ok(player) => Json(types::PlayerResponse {
                player: Some(player),
                error: types::Error::None,
            }),
            GetPlayerResult::NotFound => Json(types::PlayerResponse {
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
    let action = model::Action {
        action: action.action_id.into(),
        object_id: action.object_id.clone(),
        subject_id: action.subject_id.clone(),
    };
    match state.write() {
        Ok(mut state) => match state.act(&action) {
            ActResult::Ok => Json(types::DefaultResponse {
                ok: true,
                error: types::Error::None,
            }),
            ActResult::NotFound => Json(types::DefaultResponse {
                ok: false,
                error: types::Error::NotFound,
            }),
        },
        Err(err) => Json(types::DefaultResponse {
            ok: false,
            error: types::Error::MultiThread(err.to_string()),
        }),
    }
}

pub async fn get_action(
    State(state): State<SharedState>,
    Path(id): Path<String>,
) -> Json<types::ActionResponse> {
    log::debug!("Returning actions of player: id={}", id);
    match state.write() {
        Ok(state) => match state.get_actions(&id) {
            GetActionsResult::Ok(actions) => Json(types::ActionResponse {
                actions: Some(types::ActionsCounted::new(&id, actions)),
                error: types::Error::None,
            }),
            GetActionsResult::NotFound => Json(types::ActionResponse {
                actions: None,
                error: types::Error::NotFound,
            }),
        },
        Err(err) => Json(types::ActionResponse {
            actions: None,
            error: types::Error::MultiThread(err.to_string()),
        }),
    }
}

pub async fn get_gold(
    State(state): State<SharedState>,
    Path(id): Path<String>,
) -> Json<types::GoldResponse> {
    log::debug!("Returning gold of player: id={}", id);
    match state.read() {
        Ok(state) => match state.get_player(&id) {
            GetPlayerResult::Ok(player) => Json(types::GoldResponse {
                gold: Some(player.gold),
                error: types::Error::None,
            }),
            GetPlayerResult::NotFound => Json(types::GoldResponse {
                gold: None,
                error: types::Error::NotFound,
            }),
        },
        Err(err) => {
            return Json(types::GoldResponse {
                gold: None,
                error: types::Error::MultiThread(err.to_string()),
            })
        }
    }
}
