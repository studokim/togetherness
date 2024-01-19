use crate::timer;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Timer {
    pub seconds: String,
    pub minutes: String,
}

impl Timer {
    pub fn new(timer: &timer::Timer) -> Self {
        match timer.get() {
            timer::GetTimerResult::Remaining(seconds) => Self {
                minutes: (seconds / 60).to_string(),
                seconds: (seconds % 60).to_string(),
            },
            timer::GetTimerResult::Expired | timer::GetTimerResult::NotStarted => Self {
                minutes: "XX".to_string(),
                seconds: "XX".to_string(),
            },
        }
    }
}
