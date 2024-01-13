use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

use crate::log;

pub type PlayerId = String;
pub type FactionId = u32;
pub type AvatarId = u32;

#[derive(Serialize, Deserialize, Debug)]
pub struct Player {
    pub id: PlayerId,
    pub name: String,
    pub avatar_id: AvatarId,
    pub faction_id: FactionId,
    #[serde(skip)]
    pub gold: u32,
}

impl Player {
    pub fn new(id: &PlayerId, name: &str, avatar_id: AvatarId, faction_id: FactionId) -> Self {
        Self {
            id: id.clone(),
            name: name.to_string(),
            avatar_id,
            faction_id,
            gold: u32::MAX,
        }
    }

    pub fn act(&mut self, action: ActionType, object_id: PlayerId) {
        log::debug!(
            "Acting: player={}, object_id={:?}, action={:?}",
            self.name,
            object_id,
            action,
        );
    }
}

impl Clone for Player {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            name: self.name.clone(),
            avatar_id: self.avatar_id,
            faction_id: self.faction_id,
            gold: self.gold,
        }
    }
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Player {}

impl Hash for Player {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
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
    pub subject_id: PlayerId,
    pub object_id: PlayerId,
    pub action: ActionType,
}
