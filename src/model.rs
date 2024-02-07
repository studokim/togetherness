use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

use crate::log;

pub type PlayerId = String;
pub type FactionId = u32;
pub type AvatarId = u32;
pub type Count = usize;
pub type DifferenceInGold = i32;

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
            ActionType::Stealing => match object.gold {
                0 => {}
                1 => {
                    self.give_gold(1);
                    object.take_gold(1);
                }
                _ => {
                    self.give_gold(2);
                    object.take_gold(2);
                }
            },
            ActionType::Blackmail => {
                self.take_gold(1);
                object.take_gold(4);
            }
            ActionType::Bribery => {
                self.take_gold(1);
                object.give_gold(3);
            }
            ActionType::Lobbying => {
                object.give_gold(2);
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Faction {
    pub id: FactionId,
    pub name: String,
    pub members: Count,
    pub gold: Count,
}

impl Faction {
    pub fn name(id: FactionId) -> String {
        match id {
            1 => "Принцесса".to_string(),
            2 => "Герцог".to_string(),
            3 => "Солдат".to_string(),
            4 => "Шут".to_string(),
            _ => panic!("Существует только 4 фракции"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub enum ActionType {
    Hug,
    Stealing,
    Blackmail,
    Bribery,
    Lobbying,
}

// TODO: combine duplicated logic
impl ActionType {
    pub fn as_subject(&self) -> DifferenceInGold {
        match self {
            ActionType::Hug => 1,
            ActionType::Stealing => 2,
            ActionType::Blackmail => 3,
            ActionType::Bribery => 3,
            ActionType::Lobbying => 4,
        }
    }

    pub fn as_object(&self) -> DifferenceInGold {
        match self {
            ActionType::Hug => 1,
            ActionType::Stealing => 0,
            ActionType::Blackmail => -1,
            ActionType::Bribery => 0,
            ActionType::Lobbying => -2,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub struct Action {
    pub subject_id: PlayerId,
    pub object_id: PlayerId,
    pub action: ActionType,
}
