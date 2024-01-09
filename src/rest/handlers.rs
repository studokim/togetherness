use axum::{extract::Path, response::Html, Json};

use super::types;
use crate::{log, model};

pub async fn index() -> Html<&'static str> {
    log::debug!("index.html");
    // Include an utf-8 file contents as `&'static str` in compile time.
    // This macro is relative to current `handlers.rs` file.
    Html(include_str!("index.html"))
}

pub async fn post_player(player: Json<types::PlayerRequest>) -> Json<types::DefaultResponse> {
    log::debug!("Registering player: name={}, id={}", player.name, player.id);
    Json(types::DefaultResponse {
        ok: true,
        timer: "Now".to_string(),
        error: types::Error::None,
    })
}

pub async fn get_player(Path(id): Path<String>) -> Json<types::PlayerResponse> {
    log::debug!("Returning player: id={}", id);
    Json(types::PlayerResponse {
        player: model::Player::new(&id, "John", 0, 11),
        error: types::Error::None,
        timer: "Now".to_string(),
    })
}

pub async fn post_action(action: Json<types::ActionRequest>) -> Json<types::DefaultResponse> {
    log::debug!(
        "Making action: id={}, subject={}, object={}",
        action.action_id,
        action.subject_id,
        action.object_id
    );
    Json(types::DefaultResponse {
        ok: true,
        timer: "Now".to_string(),
        error: types::Error::None,
    })
}

pub async fn get_action(Path(id): Path<String>) -> Json<types::ActionResponse> {
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
        timer: "Now".to_string(),
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
