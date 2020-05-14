pub mod fps;
use crate::image::gif::Frame;
use crate::image::Image;
use crate::record::fps::{FpsClock, TimeUnit};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

/* Sender and main thread of the Recorder */
#[derive(Debug)]
pub struct Record {
	sender: mpsc::Sender<()>,
	pub thread: thread::JoinHandle<Vec<Frame>>,
}

impl Record {
	/**
	 * Create a new Record object.
	 *
	 * @param  sender
	 * @param  thread
	 * @return Record
	 */
	pub fn new(
		sender: mpsc::Sender<()>,
		thread: thread::JoinHandle<Vec<Frame>>,
	) -> Self {
		Self { sender, thread }
	}

	/**
	 * Terminate the recording thread.
	 *
	 * @return Result
	 */
	pub fn finish(&self) -> Result<(), mpsc::SendError<()>> {
		self.sender.send(())?;
		Ok(())
	}
}

/* Recorder with FPS clock and channel */
pub struct Recorder {
	clock: FpsClock,
	channel: (mpsc::Sender<()>, mpsc::Receiver<()>),
	get_image: &'static (dyn Fn() -> Option<Image> + Sync + Send),
}

impl Recorder {
	/**
	 * Create a new Recorder object.
	 *
	 * @param  fps
	 * @return Recorder
	 */
	pub fn new(
		fps: u32,
		get_image: impl Fn() -> Option<Image> + Sync + Send + 'static,
	) -> Self {
		Self {
			clock: FpsClock::new(fps),
			channel: mpsc::channel(),
			get_image: Box::leak(Box::new(get_image)),
		}
	}

	pub fn get_frame(&mut self) -> Frame {
		match (self.get_image)() {
			Some(image) => Frame::new(
				image,
				(self.clock.get_fps(TimeUnit::Millisecond) / 10.) as u16,
			),
			None => panic!("Failed to get the image"),
		}
	}

	pub fn record_sync(&mut self) -> Vec<Frame> {
		let mut frames = Vec::new();
		let recording = Arc::new(AtomicBool::new(true));
		let rec_state = recording.clone();
		ctrlc::set_handler(move || {
			rec_state.store(false, Ordering::SeqCst);
		})
		.expect("Failed to set the signal handler");
		while recording.load(Ordering::SeqCst) {
			self.clock.tick();
			frames.push(self.get_frame());
		}
		frames
	}

	/**
	 * Start recording the frames.
	 *
	 * @param  get_image (Fn)
	 * @return Record
	 */
	pub fn record_async(mut self) -> Record {
		let mut frames = Vec::new();
		Record::new(
			self.channel.0.clone(),
			thread::spawn(move || {
				thread::sleep(Duration::from_millis(
					self.clock.get_fps(TimeUnit::Millisecond) as u64,
				));
				while self.channel.1.try_recv().is_err() {
					self.clock.tick();
					frames.push(self.get_frame())
				}
				frames
			}),
		)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::image::Geometry;
	use std::thread;
	use std::time::Duration;
	#[test]
	fn test_record_mod() {
		let recorder = Recorder::new(100, move || {
			Some(Image::new(
				vec![0, 0, 0, 255, 255, 255],
				Geometry::new(0, 0, 1, 1),
			))
		});
		let record = recorder.record_async();
		thread::sleep(Duration::from_millis(50));
		record.finish().unwrap();
		assert!(record.thread.join().unwrap().len() > 0);
	}
}
