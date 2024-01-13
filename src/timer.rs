use time::Duration;

pub type Seconds = i64;

pub struct Timer {
    // we're interested in durations only, so using UTC
    started: time::OffsetDateTime,
    duration: Duration,
}

impl Timer {
    pub fn new(duration: Duration) -> Self {
        Self {
            started: time::OffsetDateTime::now_utc(),
            duration: duration,
        }
    }

    pub fn remaining(&self) -> Seconds {
        (time::OffsetDateTime::now_utc() - self.started + self.duration).whole_seconds()
    }
}
