pub mod fps;
use crate::image::gif::Frame;
use crate::image::Image;
use crate::record::fps::{FpsClock, TimeUnit};
use std::sync::mpsc;
use std::thread;

pub struct RecordResult {
	pub sender: mpsc::Sender<()>,
	pub thread: thread::JoinHandle<Vec<Frame>>,
}

impl RecordResult {
	pub fn new(
		sender: mpsc::Sender<()>,
		thread: thread::JoinHandle<Vec<Frame>>,
	) -> Self {
		Self { sender, thread }
	}
}

pub struct Recorder {
	clock: FpsClock,
	channel: (mpsc::Sender<()>, mpsc::Receiver<()>),
}

impl Recorder {
	pub fn new(fps: u32) -> Self {
		Self {
			clock: FpsClock::new(fps),
			channel: mpsc::channel(),
		}
	}

	pub fn record(
		self,
		get_image: impl Fn() -> Option<Image> + Sync + Send + 'static,
	) -> RecordResult {
		let recorder = Box::leak(Box::new(self));
		let mut frames = Vec::new();
		RecordResult::new(
			recorder.channel.0.clone(),
			thread::spawn(move || {
				while recorder.channel.1.try_recv().is_err() {
					frames.push(Frame::new(
						get_image().unwrap(),
						(recorder.clock.get_fps(TimeUnit::Millisecond) / 10.) as u16,
					));
					recorder.clock.tick();
				}
				frames
			}),
		)
	}
}
