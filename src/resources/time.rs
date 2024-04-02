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
		self.dt = std::cmp::min(current - self.previous, self.max_duration);
		self.previous = current;
	}

	pub fn get_runtime(&self) -> Duration {
		self.start.elapsed()
	}

	pub fn get_dt(&self) -> f32 {
		dbg!(self.dt.as_secs_f32())
	}
}