use serde::{Deserialize, Serialize};

use crate::model;

#[derive(Serialize, Deserialize, Debug)]
pub enum Error {
    None,
    NoDatabase,
    AlreadyExists,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DefaultResponse {
    pub ok: bool,
    pub error: Error,
    pub timer: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerResponse {
    pub player: model::Player,
    pub error: Error,
    pub timer: i64,
}

impl From<model::ActionType> for u32 {
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
    pub action: u32,
    pub as_subject: u32,
    pub as_object: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ActionResponse {
    pub actions: Vec<ActionsCount>,
    pub error: Error,
    pub timer: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GoldResponse {
    pub gold: u32,
    pub error: Error,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerRequest {
    pub id: String,
    pub name: String,
    pub avatar_id: u32,
    pub faction_id: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ActionRequest {
    pub subject_id: String,
    pub object_id: String,
    pub action_id: u32,
}
