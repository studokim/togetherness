use serde::{Deserialize, Serialize};

use crate::{model::*, timer::Seconds};

#[derive(Serialize, Deserialize, Debug)]
pub enum Error {
    None,
    NotFound,
    NotSet,
    AlreadyExists,
    MultiThread(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DefaultResponse {
    pub ok: bool,
    pub error: Error,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TimerResponse {
    pub seconds: Option<Seconds>,
    pub error: Error,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerResponse {
    pub player: Option<Player>,
    pub error: Error,
}

pub type ActionId = u8;

impl Into<ActionId> for ActionType {
    fn into(self) -> ActionId {
        match self {
            ActionType::Hug => 0,
            ActionType::Eavesdropping => 1,
            ActionType::Blackmail => 2,
            ActionType::Gossip => 3,
            ActionType::Crime => 4,
        }
    }
}

impl Into<ActionType> for ActionId {
    fn into(self) -> ActionType {
        match self {
            0 => ActionType::Hug,
            1 => ActionType::Eavesdropping,
            2 => ActionType::Blackmail,
            3 => ActionType::Gossip,
            4 => ActionType::Crime,
            _ => panic!("There are only 5 available actions"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct ActionCounted {
    pub action_id: ActionId,
    pub as_subject: Count,
    pub as_object: Count,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ActionsCounted([ActionCounted; 5]);

impl ActionsCounted {
    pub fn new(id: &PlayerId, actions: Vec<&Action>) -> Self {
        let count =
            |id: &PlayerId, actions: &Vec<&Action>, action_type: ActionType, role: Role| -> Count {
                actions
                    .iter()
                    .filter(|action| -> bool {
                        action.action == action_type
                            && match role {
                                Role::Subject => action.subject_id == *id,
                                Role::Object => action.object_id == *id,
                            }
                    })
                    .count()
            };

        Self([
            ActionCounted {
                action_id: ActionType::Hug.into(),
                as_subject: count(id, &actions, ActionType::Hug, Role::Subject),
                as_object: count(id, &actions, ActionType::Hug, Role::Object),
            },
            ActionCounted {
                action_id: ActionType::Eavesdropping.into(),
                as_subject: count(id, &actions, ActionType::Eavesdropping, Role::Subject),
                as_object: count(id, &actions, ActionType::Eavesdropping, Role::Object),
            },
            ActionCounted {
                action_id: ActionType::Blackmail.into(),
                as_subject: count(id, &actions, ActionType::Blackmail, Role::Subject),
                as_object: count(id, &actions, ActionType::Blackmail, Role::Object),
            },
            ActionCounted {
                action_id: ActionType::Gossip.into(),
                as_subject: count(id, &actions, ActionType::Gossip, Role::Subject),
                as_object: count(id, &actions, ActionType::Gossip, Role::Object),
            },
            ActionCounted {
                action_id: ActionType::Crime.into(),
                as_subject: count(id, &actions, ActionType::Crime, Role::Subject),
                as_object: count(id, &actions, ActionType::Crime, Role::Object),
            },
        ])
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ActionResponse {
    pub actions: Option<ActionsCounted>,
    pub error: Error,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GoldResponse {
    pub gold: Option<Count>,
    pub error: Error,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerRequest {
    pub id: PlayerId,
    pub name: String,
    pub avatar_id: AvatarId,
    pub faction_id: FactionId,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ActionRequest {
    pub subject_id: PlayerId,
    pub object_id: PlayerId,
    pub action_id: ActionId,
}
