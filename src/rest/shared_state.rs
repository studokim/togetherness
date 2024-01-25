use std::sync::{Arc, RwLock};

use crate::state;

type Password = String;

pub struct AdminAccount {
    pub password: Password,
}

impl Default for AdminAccount {
    fn default() -> Self {
        Self {
            password: "changeme".to_string(),
        }
    }
}

pub type AppState = Arc<RwLock<state::AppState>>;
pub type AdminState = Arc<RwLock<AdminAccount>>;
