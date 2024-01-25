use axum::{
    extract::State,
    http::{header, HeaderValue},
    response::{Html, IntoResponse},
};
use minijinja::render;

use crate::{log, model, rest::shared_state::AppState, static_server::template};

// Files below are included in compile time.
// The macro is relative to current `html.rs` file.

pub async fn api() -> Html<&'static str> {
    log::debug!("api.html");
    Html(include_str!("api.html"))
}

pub async fn admin(State(state): State<AppState>) -> Html<String> {
    log::debug!("admin.html");
    let html = include_str!("admin.html");
    match state.read() {
        Ok(state) => {
            let timer = template::Timer::new(&state.timer);
            let stats = template::Stats {
                hug: state.count_actions(None, None, Some(model::ActionType::Hug)),
                eavesdropping: state.count_actions(
                    None,
                    None,
                    Some(model::ActionType::Eavesdropping),
                ),
                blackmail: state.count_actions(None, None, Some(model::ActionType::Blackmail)),
                gossip: state.count_actions(None, None, Some(model::ActionType::Gossip)),
                crime: state.count_actions(None, None, Some(model::ActionType::Crime)),
            };
            let factions: Vec<model::Faction> = (1..5)
                .map(|id: model::FactionId| model::Faction {
                    id,
                    name: model::Faction::name(id),
                    members: state.count_members(id),
                    gold: state.count_gold(id),
                })
                .collect();
            let repeated_actions = if state.repeated_actions_allowed() {
                template::RepeatedActions {
                    string: "Разрешены".to_string(),
                    checked: Some("checked".to_string()),
                }
            } else {
                template::RepeatedActions {
                    string: "Запрещены".to_string(),
                    checked: None,
                }
            };
            Html(
                render!(html, timer => timer, stats => stats, factions => factions, status => "Ok", repeated_actions => repeated_actions),
            )
        }
        Err(err) => Html(render!(html, status => err.to_string())),
    }
}

pub async fn password() -> Html<String> {
    log::debug!("password.html");
    let html = include_str!("password.html");
    Html(render!(html, status => "Ok"))
}

pub async fn favicon() -> impl IntoResponse {
    log::debug!("favicon.ico");
    let ico = include_bytes!("favicon.ico");
    (
        [(
            header::CONTENT_TYPE,
            HeaderValue::from_static("image/x-icon"),
        )],
        ico,
    )
}
