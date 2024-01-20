use std::sync::Arc;

use crate::common::helper::*;
use togetherness::*;

#[derive(Clone)]
pub struct Settings {
    pub root: reqwest::Url,
    pub repeated_actions_allowed: bool,
    pub game_started: bool,
    pub iterations: usize,
}

struct Defaults {
    settings: Settings,
    helper: Helper,
}

pub async fn run(settings: Settings, client: Arc<reqwest::Client>) {
    let s = settings.clone();
    let d = Defaults {
        settings: settings,
        helper: Helper::new(s.root.clone(), client),
    };
    for _ in 0..s.iterations {
        get_timer_test(&d).await;
    }
}

async fn get_timer_test(d: &Defaults) {
    let timer = d.helper.get_timer().await;
    match timer {
        Ok(timer) => {
            if d.settings.game_started {
                assert!(matches!(timer.error, types::Error::None));
            } else {
                assert!(
                    matches!(timer.error, types::Error::NotStarted)
                        || matches!(timer.error, types::Error::AlreadyFinished)
                );
            }
        }
        Err(error) => log::error!("{:?}", error),
    }
}

async fn get_player_test(d: &Defaults) {
    let player = d.helper.get_player(model::PlayerId::from("1")).await;
    match player {
        Ok(player) => log::info!("{:?}", player),
        Err(error) => log::error!("{:?}", error),
    }
}
