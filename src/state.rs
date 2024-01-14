use std::{collections::HashMap, sync::Mutex};

use crate::{log, model::*, timer::Timer};

pub enum RegisterResult {
    Ok,
    AlreadyExists,
}

pub enum GetPlayerResult {
    Ok(Player),
    NotFound,
}

pub enum ActResult {
    Ok,
    NotFound,
}

pub enum GetActionsResult<'a> {
    Ok(Vec<&'a Action>),
    NotFound,
}

pub enum GetActionStatsResult {
    Ok([(ActionType, Count); 5]),
}

#[derive(Default)]
pub struct AppState {
    pub timer: Timer,
    registered_players: HashMap<PlayerId, Mutex<Player>>,
    actions: Vec<Action>,
}

impl AppState {
    pub fn register(&mut self, player: Player) -> RegisterResult {
        match self.registered_players.get(&player.id) {
            Some(_) => {
                log::debug!("Player id={} already registered", player.id);
                RegisterResult::AlreadyExists
            }
            None => {
                log::debug!("Player id={} registered", player.id);
                self.registered_players
                    .insert(player.id.clone(), Mutex::new(player));
                RegisterResult::Ok
            }
        }
    }

    pub fn get_player(&self, id: &PlayerId) -> GetPlayerResult {
        match self.registered_players.get(id) {
            Some(player) => {
                log::debug!("Found player id={}", id);
                GetPlayerResult::Ok(player.lock().unwrap().clone())
            }
            None => {
                log::debug!("Player id={} not found", id);
                GetPlayerResult::NotFound
            }
        }
    }

    pub fn act(&mut self, action: &Action) -> ActResult {
        match self.registered_players.get(&action.subject_id) {
            Some(subject) => {
                log::debug!("Found subject id={}", action.subject_id);
                match self.registered_players.get(&action.object_id) {
                    Some(object) => {
                        log::debug!("Found object id={}", action.object_id);
                        let mut subject = subject.lock().unwrap();
                        subject.act(&action.action, object.lock().as_mut().unwrap());
                        self.actions.push(action.clone());
                        ActResult::Ok
                    }
                    None => {
                        log::debug!("Object id={} not found", action.object_id);
                        ActResult::NotFound
                    }
                }
            }
            None => {
                log::debug!("Subject id={} not found", action.subject_id);
                ActResult::NotFound
            }
        }
    }

    pub fn get_actions(&self, id: &PlayerId) -> GetActionsResult {
        match self.registered_players.get(id) {
            Some(_) => {
                log::debug!("Got actions of player id={}", id);
                GetActionsResult::Ok(
                    self.actions
                        .iter()
                        .filter(|action| -> bool {
                            action.subject_id == *id || action.object_id == *id
                        })
                        .collect(),
                )
            }
            None => {
                log::debug!("No actions: player id={} not found", id);
                GetActionsResult::NotFound
            }
        }
    }

    pub fn get_action_stats(&self) -> GetActionStatsResult {
        log::debug!("Got all action stats");

        GetActionStatsResult::Ok([
            (
                ActionType::Hug,
                self.actions
                    .iter()
                    .filter(|action| -> bool { action.action == ActionType::Hug })
                    .count(),
            ),
            (
                ActionType::Eavesdropping,
                self.actions
                    .iter()
                    .filter(|action| -> bool { action.action == ActionType::Eavesdropping })
                    .count(),
            ),
            (
                ActionType::Blackmail,
                self.actions
                    .iter()
                    .filter(|action| -> bool { action.action == ActionType::Blackmail })
                    .count(),
            ),
            (
                ActionType::Gossip,
                self.actions
                    .iter()
                    .filter(|action| -> bool { action.action == ActionType::Gossip })
                    .count(),
            ),
            (
                ActionType::Crime,
                self.actions
                    .iter()
                    .filter(|action| -> bool { action.action == ActionType::Crime })
                    .count(),
            ),
        ])
    }
}
