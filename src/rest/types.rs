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
            _ => panic!("There are only 5 available actions"), // TODO: use ActionType::Undefined instead
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ActionResponse {
    pub count: Option<Count>,
    pub error: Error,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GoldResponse {
    pub gold: Option<Count>,
    pub error: Error,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StatsResponse {
    pub hug: Count,
    pub eavesdropping: Count,
    pub blackmail: Count,
    pub gossip: Count,
    pub crime: Count,
    pub error: Error,
}

impl StatsResponse {
    pub fn default(error: Error) -> Self {
        Self {
            hug: 0,
            eavesdropping: 0,
            blackmail: 0,
            gossip: 0,
            crime: 0,
            error,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PostPlayerRequest {
    pub id: PlayerId,
    pub name: String,
    pub avatar_id: AvatarId,
    pub faction_id: FactionId,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PostActionRequest {
    pub subject_id: PlayerId,
    pub object_id: PlayerId,
    pub action_id: ActionId,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetActionRequest {
    pub subject_id: Option<PlayerId>,
    pub object_id: Option<PlayerId>,
    pub action_id: Option<ActionId>,
}
