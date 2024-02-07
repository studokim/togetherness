use crate::{model::Count, timer};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Timer(String);

impl Timer {
    pub fn new(timer: &timer::Timer) -> Self {
        match timer.get() {
            timer::GetTimerResult::Remaining(seconds) => {
                let minutes = (seconds / 60).to_string();
                let seconds = (seconds % 60).to_string();
                Self(format!("{}:{}", minutes, seconds))
            }
            timer::GetTimerResult::NotStarted => Self("Ночь не началась".to_string()),
            timer::GetTimerResult::Expired => Self("Ночь окончилась".to_string()),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Stats {
    pub hug: Count,
    pub stealing: Count,
    pub blackmail: Count,
    pub bribery: Count,
    pub lobbying: Count,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RepeatedActions {
    pub checked: Option<String>,
    pub string: String,
}
