use std::sync::Arc;
use togetherness::*;

mod common;
use common::*;

#[tokio::test]
async fn test_async() {
    log::configure(tracing::Level::INFO, false);

    const PLAYERS: usize = 100;
    const ROOT: &'static str = "https://togetherness.muravev.keenetic.name/";
    let root = reqwest::Url::parse(ROOT).unwrap();
    let client = Arc::new(reqwest::Client::new());
    let settings = scenario::Settings {
        root: root.clone(),
        game_started: true,
        players: PLAYERS,
        iterations: 100,
        delays_between_iterations: true,
        check_gold: false,
        repeated_actions_allowed: false,
    };

    let mut futures = Vec::with_capacity(PLAYERS);
    let mut handles = Vec::with_capacity(PLAYERS);
    let mut results = Vec::with_capacity(PLAYERS);

    // reset game
    helper::Helper::new(root.clone(), client.clone())
        .reset_game(settings.repeated_actions_allowed)
        .await;
    // register all players (sync)
    for id in 0..PLAYERS {
        scenario::register_player(id, root.clone(), client.clone()).await;
    }
    // start acting (async)
    for id in 0..PLAYERS {
        let id: model::PlayerId = id.to_string();
        let instance = scenario::run(id, settings.clone(), client.clone());
        futures.push(instance);
    }
    for fut in futures {
        handles.push(tokio::spawn(fut));
    }
    for handle in handles {
        results.push(handle.await.unwrap());
    }
}
