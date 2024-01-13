use std::sync::{Arc, RwLock};

use crate::state::AppState;

pub type SharedState = Arc<RwLock<AppState>>;
