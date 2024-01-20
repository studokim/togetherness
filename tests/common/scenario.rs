use rand::Rng;
use std::{collections::HashSet, sync::Arc, time::Duration};
use tokio::time::sleep;

use crate::common::helper::*;
use togetherness::{model::PlayerId, *};

#[derive(Clone)]
pub struct Settings {
    pub root: reqwest::Url,
    pub game_started: bool,
    pub players: usize,
    pub iterations: usize,
    pub delays_between_iterations: bool,
    pub check_gold: bool,
    pub repeated_actions_allowed: bool,
}

struct Data {
    settings: Settings,
    helper: Helper,
    id: PlayerId,
    acted_with: HashSet<PlayerId>,
}

pub async fn register_player(id: usize, root: reqwest::Url, client: Arc<reqwest::Client>) {
    let helper = Helper::new(root, client);
    let player = model::Player {
        id: id.to_string(),
        name: id.to_string(),
        avatar_id: 0,
        faction_id: (id % 4 + 1).try_into().unwrap(),
        gold: 0,
    };
    let response = helper.post_player(player).await.unwrap();
    if !matches!(response.error, types::Error::None) {
        log::error!("{:?}", response);
        assert!(false);
    } else {
        log::info!("Player {} registered", id);
    }
}

pub async fn run(id: PlayerId, settings: Settings, client: Arc<reqwest::Client>) {
    let s = settings.clone();
    let mut d = Data {
        settings,
        helper: Helper::new(s.root.clone(), client),
        id,
        acted_with: HashSet::new(),
    };
    for _ in 0..s.iterations {
        if s.delays_between_iterations {
            let millis = rand::thread_rng().gen_range(500..2000);
            sleep(Duration::from_millis(millis)).await;
        }
        get_timer_test(&d).await;
        act_test(&mut d).await;
    }
}

async fn get_timer_test(d: &Data) {
    match d.helper.get_timer().await {
        Ok(timer) => {
            if d.settings.game_started {
                if !matches!(timer.error, types::Error::None) {
                    log::error!("{:?}", timer);
                    assert!(false);
                }
            } else {
                assert!(
                    matches!(timer.error, types::Error::NotStarted)
                        || matches!(timer.error, types::Error::AlreadyFinished)
                );
                log::debug!("Game is not running, timer ok");
            }
        }
        Err(error) => {
            log::error!("{:?}", error);
            assert!(false)
        }
    }
}

async fn act_test(d: &mut Data) {
    let action = random_action();
    let subject_id = d.id.to_string();
    let object_id = random_object(subject_id.clone(), d);

    let gold = get_gold(subject_id.clone(), d).await;
    let their_gold = get_gold(object_id.clone(), d).await;
    log::debug!(
        "Current: {}={}g, {}={}g",
        subject_id,
        gold,
        object_id,
        their_gold
    );
    let gold_expected = std::cmp::max(gold + action.as_subject(), 0);
    let their_gold_expected = std::cmp::max(their_gold + action.as_object(), 0);
    log::debug!(
        "Expected: {}={}g, {}={}g",
        subject_id,
        gold_expected,
        object_id,
        their_gold_expected
    );

    match d
        .helper
        .post_action(model::Action {
            action,
            subject_id: subject_id.clone(),
            object_id: object_id.clone(),
        })
        .await
    {
        Ok(response) => {
            if d.settings.check_gold {
                log::debug!(
                    "Checking difference in gold {}->{}...",
                    subject_id.clone(),
                    object_id.clone()
                );
                if !matches!(response.error, types::Error::None) {
                    log::error!("{:?}", response);
                    assert!(false);
                }
                let gold_actual = get_gold(subject_id.clone(), d).await;
                let their_gold_actual = get_gold(object_id.clone(), d).await;
                log::debug!(
                    "Current: {}={}g, {}={}g",
                    subject_id,
                    gold_actual,
                    object_id,
                    their_gold_actual
                );
                assert_eq!(gold_actual, gold_expected);
                assert_eq!(their_gold_actual, their_gold_expected);
            } else {
                log::debug!("Difference in gold is not checked");
                assert!(true)
            }
        }
        Err(error) => {
            log::error!("Could not post an action: {:?}", error);
            assert!(false)
        }
    }
}

async fn get_gold(id: PlayerId, d: &Data) -> i32 {
    i32::try_from(d.helper.get_gold(id.clone()).await.unwrap().gold.unwrap()).unwrap()
}

fn random_object(subject_id: PlayerId, d: &mut Data) -> model::PlayerId {
    loop {
        let id: model::PlayerId = rand::thread_rng()
            .gen_range(0..d.settings.players)
            .to_string();

        // acting with self leads to deadlock
        if id != subject_id {
            if d.settings.repeated_actions_allowed || !d.acted_with.contains(&id) {
                d.acted_with.insert(id.clone());
                return id;
            }
        }
    }
}

fn random_action() -> model::ActionType {
    let action: types::ActionId = rand::thread_rng().gen_range(1..6);
    action.into()
}
