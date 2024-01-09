use axum::{extract::Path, response::Html, Json};

use crate::log;
use crate::model;

pub async fn index() -> Html<&'static str> {
    log::debug!("index.html");
    // Include an utf-8 file contents as `&'static str` in compile time.
    // This macro is relative to current `handlers.rs` file.
    Html(include_str!("index.html"))
}

pub async fn post_player(player: Json<model::Player>) -> Json<model::DefaultResponse> {
    log::debug!("Registering player: name={}, id={}", player.name, player.id);
    Json(model::DefaultResponse {
        ok: true,
        timer: "Now".to_string(),
        error: model::Error::None,
    })
}

pub async fn get_player(Path(id): Path<String>) -> Json<model::PlayerResponse> {
    log::debug!("Returning player: id={}", id);
    Json(model::PlayerResponse {
        player: model::Player {
            id,
            name: "John".to_string(),
            avatar_id: 0,
            faction_id: 9,
        },
        error: model::Error::None,
    })
}

pub async fn post_action(action: Json<model::Action>) -> Json<model::DefaultResponse> {
    log::debug!(
        "Making action: id={}, subject={}, object={}",
        action.action_id,
        action.subject_id,
        action.object_id
    );
    Json(model::DefaultResponse {
        ok: true,
        timer: "Now".to_string(),
        error: model::Error::None,
    })
}

pub async fn get_action(Path(id): Path<String>) -> Json<model::Action> {
    Json(model::Action {
        action_id: 9,
        subject_id: id,
        object_id: "John_id".to_string(),
    })
}

pub async fn get_gold(Path(id): Path<String>) -> Json<model::GoldResponse> {
    Json(model::GoldResponse {
        gold: id
            .chars()
            .count()
            .try_into()
            .expect("No way could one have so much gold"),
        error: model::Error::None,
    })
}
