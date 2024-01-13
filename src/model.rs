use serde::{Deserialize, Serialize};

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
            id: id.to_string(),
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
