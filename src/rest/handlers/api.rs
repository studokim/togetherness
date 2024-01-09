use axum::{extract::Path, Json};

use crate::db::repository;
use crate::rest::types;
use crate::{log, model};

pub async fn post_player(player: Json<types::PlayerRequest>) -> Json<types::DefaultResponse> {
    let repo = repository::Repository::new();
    log::debug!("Registering player: name={}, id={}", player.name, player.id);
    Json(types::DefaultResponse {
        ok: true,
        timer: repo.get_timer().remaining(),
        error: types::Error::None,
    })
}

pub async fn get_player(Path(id): Path<String>) -> Json<types::PlayerResponse> {
    let repo = repository::Repository::new();
    log::debug!("Returning player: id={}", id);
    Json(types::PlayerResponse {
        player: model::Player::new(&id, "John", 0, 11),
        error: types::Error::None,
        timer: repo.get_timer().remaining(),
    })
}

pub async fn post_action(action: Json<types::ActionRequest>) -> Json<types::DefaultResponse> {
    let repo = repository::Repository::new();
    log::debug!(
        "Making action: id={}, subject={}, object={}",
        action.action_id,
        action.subject_id,
        action.object_id
    );
    Json(types::DefaultResponse {
        ok: true,
        timer: repo.get_timer().remaining(),
        error: types::Error::None,
    })
}

pub async fn get_action(Path(id): Path<String>) -> Json<types::ActionResponse> {
    let repo = repository::Repository::new();
    log::debug!("Returning actions of player: id={}", id);
    Json(types::ActionResponse {
        actions: vec![
            model::Action {
                action: model::ActionType::Blackmail,
                subject_id: id.clone(),
                object_id: "Miserable Johnny".to_string(),
            },
            model::Action {
                action: model::ActionType::Crime,
                subject_id: "Mighty Arthur".to_string(),
                object_id: id.clone(),
            },
        ],
        error: types::Error::None,
        timer: repo.get_timer().remaining(),
    })
}

pub async fn get_gold(Path(id): Path<String>) -> Json<types::GoldResponse> {
    log::debug!("Returning gold of player: id={}", id);
    Json(types::GoldResponse {
        gold: id
            .chars()
            .count()
            .try_into()
            .expect("No way could one have so much gold"),
        error: types::Error::None,
    })
}
