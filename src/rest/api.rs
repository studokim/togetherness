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
        action.action,
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
            types::ActionsCount {
                action: u32::from(model::ActionType::Blackmail),
                as_subject: 8,
                as_object: 19,
            },
            types::ActionsCount {
                action: u32::from(model::ActionType::Gossip),
                as_subject: 5,
                as_object: 2,
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
