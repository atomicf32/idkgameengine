use std::time::{Duration, Instant};

pub struct TimerResource {
	max_duration: Duration,
	start: Instant,
	previous: Instant,
	dt: Duration,
}

impl TimerResource {
	pub fn new(max_duration: Duration) -> Self {
		Self { max_duration, start: Instant::now(), previous: Instant::now(), dt: Duration::ZERO }
	}

	pub fn tick(&mut self) {
		let current = Instant::now();
		self.dt = current - self.previous;
		self.previous = current;
	}

	pub fn get_runtime(&self) -> Duration {
		self.start.elapsed()
	}

	pub fn get_dt(&self) -> f32 {
		std::cmp::min(self.dt, self.max_duration).as_secs_f32()
	}
}