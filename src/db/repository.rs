use crate::timer;

pub struct Repository {
    timer: Option<timer::Timer>,
}

impl Repository {
    pub fn new() -> Self {
        Self { timer: None }
    }

    pub fn save_timer(&mut self, timer: timer::Timer) {
        unimplemented!()
    }

    pub fn get_timer(&self) -> timer::Timer {
        let duration = time::Duration::minutes(20);
        let timer = timer::Timer::new(duration);
        timer
    }
}
