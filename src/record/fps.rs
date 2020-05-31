use std::thread;
use std::time;

/* Unit of time */
#[derive(Debug)]
pub enum TimeUnit {
	Nanosecond,
	Millisecond,
}

/* FPS controller */
#[derive(Debug)]
pub struct FpsClock {
	pub fps: u32,
	last_tick_time: time::Instant,
}

impl FpsClock {
	/**
	 * Create a new FpsClock object.
	 *
	 * @param  fps
	 * @return FpsClock
	 */
	pub fn new(fps: u32) -> Self {
		Self {
			fps,
			last_tick_time: time::Instant::now(),
		}
	}

	/**
	 * Get the FPS value in the given time unit.
	 *
	 * @param  unit
	 * @return f32
	 */
	pub fn get_fps(&self, unit: TimeUnit) -> f32 {
		match unit {
			TimeUnit::Nanosecond => (1. / self.fps as f32) * 1_000_000_000.,
			TimeUnit::Millisecond => (1. / self.fps as f32) * 1_000.,
		}
	}

	/**
	 * Sleep the thread to run at the correct FPS.
	 *
	 * @return f32
	 */
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

#[cfg(test)]
mod tests {
	use super::*;
	use std::thread;
	use std::time::Duration;
	#[test]
	fn test_fps_mod() {
		let mut fps_clock = FpsClock::new(100);
		assert_eq!(10., fps_clock.get_fps(TimeUnit::Millisecond));
		assert_eq!(10_000_000., fps_clock.get_fps(TimeUnit::Nanosecond));
		for i in 0..2 {
			thread::sleep(Duration::from_nanos(i));
			assert!(fps_clock.get_fps(TimeUnit::Nanosecond) > fps_clock.tick());
		}
	}
}
