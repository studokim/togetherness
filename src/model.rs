use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

use crate::log;

pub type PlayerId = String;
pub type FactionId = u32;
pub type AvatarId = u32;
pub type Count = usize;

#[derive(Serialize, Deserialize, Debug)]
pub struct Player {
    pub id: PlayerId,
    pub name: String,
    pub avatar_id: AvatarId,
    pub faction_id: FactionId,
    #[serde(skip)]
    pub gold: Count,
}

impl Player {
    pub fn new(id: &PlayerId, name: &str, avatar_id: AvatarId, faction_id: FactionId) -> Self {
        Self {
            id: id.clone(),
            name: name.to_string(),
            avatar_id,
            faction_id,
            gold: 0,
        }
    }

    pub fn act(&mut self, action: &ActionType, object: &mut Player) {
        log::debug!(
            "Acted: player={}, object_id={}, action={:?}",
            self.id,
            object.id,
            action,
        );
        match action {
            ActionType::Hug => {
                self.give_gold(1);
                object.give_gold(1);
            }
            ActionType::Eavesdropping => {
                self.give_gold(2);
            }
            ActionType::Blackmail => {
                self.give_gold(3);
                object.take_gold(1);
            }
            ActionType::Gossip => {
                self.give_gold(3);
            }
            ActionType::Crime => {
                self.give_gold(4);
                object.take_gold(2);
            }
        }
    }

    fn give_gold(&mut self, value: Count) {
        self.gold = if self.gold > Count::MAX - value {
            Count::MAX
        } else {
            self.gold + value
        }
    }

    fn take_gold(&mut self, value: Count) {
        self.gold = if self.gold < value {
            0
        } else {
            self.gold - value
        }
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

pub enum Role {
    Subject,
    Object,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub enum ActionType {
    Hug,
    Eavesdropping,
    Blackmail,
    Gossip,
    Crime,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub struct Action {
    pub subject_id: PlayerId,
    pub object_id: PlayerId,
    pub action: ActionType,
}
