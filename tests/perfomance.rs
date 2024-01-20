use std::sync::Arc;
use togetherness::*;

mod common;
use common::*;

#[tokio::test]
async fn test_async() {
    log::configure(tracing::Level::INFO);

    let root = "https://togetherness.muravev.keenetic.name/";
    let root = reqwest::Url::parse(root).unwrap();
    let settings = scenario::Settings {
        root,
        repeated_actions_allowed: false,
        game_started: false,
        iterations: 500,
    };
    let client = Arc::new(reqwest::Client::new());

    const INSTANCES: usize = 1;
    let mut futures = Vec::with_capacity(INSTANCES);
    let mut handles = Vec::with_capacity(INSTANCES);
    let mut results = Vec::with_capacity(INSTANCES);

    for _ in 0..INSTANCES {
        let instance = scenario::run(settings.clone(), client.clone());
        futures.push(instance);
    }
    for fut in futures {
        handles.push(tokio::spawn(fut));
    }
    for handle in handles {
        results.push(handle.await.unwrap());
    }
}
