use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Error {
    None,
    NoDatabase,
    AlreadyExists,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Player {
    pub id: String,
    pub name: String,
    pub avatar_id: u32,
    pub faction_id: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerResponse {
    pub player: Player,
    pub error: Error,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DefaultResponse {
    pub ok: bool,
    pub error: Error,
    pub timer: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Action {
    pub subject_id: String,
    pub object_id: String,
    pub action_id: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ActionResponse {
    pub actions: Vec<Action>,
    pub error: Error,
    pub timer: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GoldResponse {
    pub gold: u32,
    pub error: Error,
}
