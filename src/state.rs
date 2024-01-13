use std::sync::{Arc, RwLock};

use crate::log;

pub type SharedState = Arc<RwLock<AppState>>;

#[derive(Default)]
pub struct AppState {
    request_counter: u32,
}

impl AppState {
    pub fn new_request(&mut self) {
        self.request_counter += 1;
        log::debug!("Request counter: {}", self.request_counter);
    }
}
