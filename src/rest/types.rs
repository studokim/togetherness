use serde::{Deserialize, Serialize};

use crate::{model::*, timer::Seconds};

#[derive(Serialize, Deserialize, Debug)]
pub enum Error {
    None,
    PlayerAlreadyExists,
    PlayerNotFound,
    SubjectNotFound,
    ObjectNotFound,
    ActionNotFound,
    NotEnoughGold,
    NotStarted,
    AlreadyStarted,
    AlreadyFinished,
    AlreadyActed,
    SetToZero,
    MultiThread,
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
            ActionType::Hug => 1,
            ActionType::Stealing => 2,
            ActionType::Blackmail => 3,
            ActionType::Bribery => 4,
            ActionType::Lobbying => 5,
        }
    }
}

impl Into<ActionType> for ActionId {
    fn into(self) -> ActionType {
        match self {
            1 => ActionType::Hug,
            2 => ActionType::Stealing,
            3 => ActionType::Blackmail,
            4 => ActionType::Bribery,
            5 => ActionType::Lobbying,
            _ => panic!("There are only 5 available actions"), // TODO: use ActionType::Undefined instead
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ActionResponse {
    pub possible: Option<bool>,
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
    pub stealing: Count,
    pub blackmail: Count,
    pub bribery: Count,
    pub lobbying: Count,
    pub error: Error,
}

impl StatsResponse {
    pub fn default(error: Error) -> Self {
        Self {
            hug: 0,
            stealing: 0,
            blackmail: 0,
            bribery: 0,
            lobbying: 0,
            error,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerStatusTuple {
    pub action_id: ActionId,
    pub as_subject: Count,
    pub as_object: Count,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerStatusResponse {
    pub status: Option<[PlayerStatusTuple; 5]>,
    pub error: Error,
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
    pub subject_id: PlayerId,
    pub object_id: PlayerId,
}
