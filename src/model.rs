use serde::{Deserialize, Serialize};

use crate::log;

#[derive(Serialize, Deserialize, Debug)]
pub struct Player {
    pub id: String,
    pub name: String,
    pub avatar_id: u32,
    pub faction_id: u32,
    #[serde(skip)]
    pub gold: u32,
}

impl Player {
    pub fn new(id: &str, name: &str, avatar_id: u32, faction_id: u32) -> Player {
        Player {
            id: id.to_string(),
            name: name.to_string(),
            avatar_id,
            faction_id,
            gold: u32::MAX,
        }
    }

    pub fn act(&mut self, action: ActionType, role: Role) {
        log::debug!(
            "Acting: player={}, role={:?}, action={:?}",
            self.name,
            action,
            role
        );
    }
}

#[derive(Debug)]
pub enum Role {
    Subject,
    Object,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ActionType {
    Hug,
    Eavesdropping,
    Blackmail,
    Gossip,
    Crime,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Action {
    pub subject_id: String,
    pub object_id: String,
    pub action: ActionType,
}
