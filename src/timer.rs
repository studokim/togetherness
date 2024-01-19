use time::Duration;

pub type Seconds = i64;

pub enum SetTimerResult {
    Ok,
    AlreadyStarted,
}

pub enum StartTimerResult {
    Ok,
    AlreadyStarted,
    SetToZero,
}

pub enum StopTimerResult {
    Ok,
    NotStarted,
}

pub enum GetTimerResult {
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
    pub fn set(&mut self, duration: Duration) -> SetTimerResult {
        match self.started {
            Some(_) => SetTimerResult::AlreadyStarted,
            None => {
                self.duration = duration;
                SetTimerResult::Ok
            }
        }
    }

    pub fn start(&mut self) -> StartTimerResult {
        match self.started {
            Some(_) => StartTimerResult::AlreadyStarted,
            None => {
                if self.duration.whole_seconds() > 0 {
                    self.started = Some(time::OffsetDateTime::now_utc());
                    StartTimerResult::Ok
                } else {
                    StartTimerResult::SetToZero
                }
            }
        }
    }

    pub fn stop(&mut self) -> StopTimerResult {
        match self.started {
            Some(_) => {
                self.started = None;
                StopTimerResult::Ok
            }
            None => StopTimerResult::NotStarted,
        }
    }

    pub fn get(&self) -> GetTimerResult {
        match self.started {
            Some(started) => {
                let seconds =
                    (started + self.duration - time::OffsetDateTime::now_utc()).whole_seconds();
                if seconds > 0 {
                    GetTimerResult::Remaining(seconds)
                } else {
                    GetTimerResult::Expired
                }
            }
            None => GetTimerResult::NotStarted,
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
