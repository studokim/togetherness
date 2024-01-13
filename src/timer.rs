use time::Duration;

pub type Seconds = i64;

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

    pub fn remaining(&self) -> Option<Seconds> {
        match self.started {
            Some(started) => {
                Some((time::OffsetDateTime::now_utc() - started + self.duration).whole_seconds())
            }
            None => None,
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
