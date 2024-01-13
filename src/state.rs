use std::collections::HashMap;

use crate::{log, model::*, timer::Timer};

pub enum Error {
    None,
    AlreadyExists,
}

#[derive(Default)]
pub struct AppState {
    pub timer: Timer,
    registered_players: HashMap<PlayerId, Player>,
}

impl AppState {
    pub fn register(&mut self, player: Player) -> Error {
        match self.registered_players.get(&player.id) {
            Some(_) => {
                log::debug!("Player id={} already registered", player.id);
                Error::AlreadyExists
            }
            None => {
                log::debug!("Player id={} registered", player.id);
                self.registered_players.insert(player.id.clone(), player);
                Error::None
            }
        }
    }

    pub fn get_player(&self, id: &PlayerId) -> Option<Player> {
        match self.registered_players.get(id) {
            Some(player) => {
                log::debug!("Found player id={}", id);
                Some(player.clone())
            }
            None => {
                log::debug!("Player id={} not found", id);
                None
            }
        }
    }
}
