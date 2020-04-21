use std::thread;
use std::time;

#[derive(Debug)]
pub enum TimeUnit {
	Nanosecond,
	Millisecond,
}

#[derive(Debug)]
pub struct FpsClock {
	fps: u32,
	last_tick_time: time::Instant,
}

impl FpsClock {
	pub fn new(fps: u32) -> FpsClock {
		FpsClock {
			fps,
			last_tick_time: time::Instant::now(),
		}
	}

	pub fn get_fps(&self, unit: TimeUnit) -> f32 {
		match unit {
			TimeUnit::Nanosecond => (1. / self.fps as f32) * 1_000_000_000.,
			TimeUnit::Millisecond => (1. / self.fps as f32) * 1_000.,
		}
	}

	pub fn tick(&mut self) -> f32 {
		let t = self.last_tick_time.elapsed();
		let total_nanos = t.as_secs() * 1_000_000_000 + t.subsec_nanos() as u64;
		let diff = self.get_fps(TimeUnit::Nanosecond) - (total_nanos as f32);
		if diff > 0. {
			thread::sleep(time::Duration::new(0, diff as u32))
		};
		self.last_tick_time = time::Instant::now();
		diff
	}
}
