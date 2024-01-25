use clap::Parser;
use std::sync::{Arc, RwLock};

use crate::config::Args;
use crate::state;

type Password = String;

pub struct AdminAccount {
    pub password: Password,
}

impl Default for AdminAccount {
    fn default() -> Self {
        Self {
            password: Args::parse().admin_password,
        }
    }
}

pub type AppState = Arc<RwLock<state::AppState>>;
pub type AdminState = Arc<RwLock<AdminAccount>>;
