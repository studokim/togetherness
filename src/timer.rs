use time::Duration;

pub type Seconds = i64;

pub enum TimerResult {
    NotStarted,
    Remaining(Seconds),
    Expired,
}

pub struct Timer {
    // we're interested in durations only, so using UTC
    started: Option<time::OffsetDateTime>,
    duration: Duration,
}

impl Timer {
    pub fn set(&mut self, duration: Duration) {
        self.duration = duration;
    }

    pub fn start(&mut self) {
        self.started = Some(time::OffsetDateTime::now_utc());
    }

    pub fn get(&self) -> TimerResult {
        match self.started {
            Some(started) => {
                let seconds =
                    (started + self.duration - time::OffsetDateTime::now_utc()).whole_seconds();
                if seconds > 0 {
                    TimerResult::Remaining(seconds)
                } else {
                    TimerResult::Expired
                }
            }
            None => TimerResult::NotStarted,
        }
    }
}

impl Default for Timer {
    fn default() -> Self {
        Self {
            started: None,
            duration: Default::default(), // zero
        }
    }
}
