use std::time;
use std::thread;

#[derive(Debug)]
pub struct FpsClock {
    last_tick_time: time::Instant,
    fps: u32,
    fps_in_nanos: f32
}

impl FpsClock {
    pub fn new(fps: u32) -> FpsClock {
        FpsClock {
            last_tick_time: time::Instant::now(),
            fps: fps,
            fps_in_nanos: (1. / fps as f32) * 1_000_000_000.
        }
    }

    pub fn fps(&self) -> u32 {
        self.fps
    }

    pub fn tick(&mut self) -> f32 {
        let t = self.last_tick_time.elapsed();
        let total_nanos = t.as_secs() * 1_000_000_000 + t.subsec_nanos() as u64;
        let diff = self.fps_in_nanos - (total_nanos as f32);
        if diff > 0. {
            thread::sleep(time::Duration::new(0, diff as u32))
        };
        self.last_tick_time = time::Instant::now();
        diff
    }
}