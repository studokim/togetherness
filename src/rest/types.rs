use serde::{Deserialize, Serialize};

use crate::{
    model::{self, AvatarId, FactionId, PlayerId},
    timer::Seconds,
};

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
    pub player: Option<model::Player>,
    pub error: Error,
}

pub type ActionId = u8;

impl From<model::ActionType> for ActionId {
    fn from(action: model::ActionType) -> Self {
        match action {
            model::ActionType::Hug => 0,
            model::ActionType::Eavesdropping => 1,
            model::ActionType::Blackmail => 2,
            model::ActionType::Gossip => 3,
            model::ActionType::Crime => 4,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ActionsCount {
    pub action_id: ActionId,
    pub as_subject: u32,
    pub as_object: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ActionResponse {
    pub actions: Option<Vec<ActionsCount>>,
    pub error: Error,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GoldResponse {
    pub gold: Option<u32>,
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
