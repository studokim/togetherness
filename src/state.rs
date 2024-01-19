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
    SubjectNotFound,
    ObjectNotFound,
}

#[derive(Default)]
pub struct AppState {
    pub timer: Timer,
    repeated_actions: bool, // default = false
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
                        ActResult::ObjectNotFound
                    }
                }
            }
            None => {
                log::debug!("Subject id={} not found", action.subject_id);
                ActResult::SubjectNotFound
            }
        }
    }

    pub fn count_actions(
        &self,
        subject_id: Option<PlayerId>,
        object_id: Option<PlayerId>,
        action_type: Option<ActionType>,
    ) -> Count {
        self.actions
            .iter()
            .filter(|action| {
                (subject_id.is_none() || action.subject_id == subject_id.clone().unwrap())
                    && (object_id.is_none() || action.object_id == object_id.clone().unwrap())
                    && (action_type.is_none() || action.action == action_type.clone().unwrap())
            })
            .count()
    }

    pub fn count_members(&self, id: FactionId) -> Count {
        self.registered_players
            .values()
            .filter(|player| player.lock().unwrap().faction_id == id)
            .count()
    }

    pub fn count_gold(&self, id: FactionId) -> Count {
        self.registered_players
            .values()
            .filter(|player| player.lock().unwrap().faction_id == id)
            .map(|player| player.lock().unwrap().gold)
            .sum()
    }

    pub fn allow_repeated_actions(&mut self) {
        self.repeated_actions = true;
    }

    pub fn forbid_repeated_actions(&mut self) {
        self.repeated_actions = false;
    }

    pub fn repeated_actions_allowed(&self) -> bool {
        self.repeated_actions
    }

    pub fn reset(&mut self) {
        self.timer = Timer::default();
        self.repeated_actions = false;
        self.registered_players.clear();
        self.actions.clear();
    }
}
