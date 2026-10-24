pub mod fps;
pub mod settings;

use crate::app::{AppError, AppResult};
use crate::image::Image;
use crate::record::fps::FpsClock;
use crate::record::settings::RecordSettings;
use crate::util::state::InputState;
use crate::window::Capture;
use std::io::{self, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc};
use std::thread;

/* Asynchronous recording result */
#[derive(Debug)]
pub struct RecordResult<T> {
	sender: mpsc::Sender<()>,
	thread: thread::JoinHandle<T>,
}

impl<T> RecordResult<T> {
	/**
	 * Create a new RecordResult object.
	 *
	 * @param  sender
	 * @param  thread
	 * @return RecordResult
	 */
	pub fn new(sender: mpsc::Sender<()>, thread: thread::JoinHandle<T>) -> Self {
		Self { sender, thread }
	}

	/**
	 * Stop the thread and retrieve values.
	 *
	 * @return Result
	 */
	pub fn get(self) -> thread::Result<T> {
		/* The thread might have already stopped on its own, which drops the
		 * receiver, but the frames that it recorded are still wanted. */
		let _ = self.sender.send(());
		self.thread.join()
	}
}

/* Recorder with FPS clock and channel */
pub struct Recorder<Window> {
	window: Window,
	clock: FpsClock,
	channel: (mpsc::Sender<()>, mpsc::Receiver<()>),
	gifski: bool,
	settings: RecordSettings,
}

impl<Window> Recorder<Window>
where
	Window: Capture + Send + Sync + 'static,
{
	/**
	 * Create a new Recorder object.
	 *
	 * @param  window
	 * @param  fps
	 * @param  gifski
	 * @param  settings
	 * @return Recorder
	 */
	pub fn new(
		window: Window,
		fps: u32,
		gifski: bool,
		settings: RecordSettings,
	) -> Self {
		Self {
			window,
			clock: FpsClock::new(fps),
			channel: mpsc::channel(),
			gifski,
			settings,
		}
	}

	/**
	 * Get the maximum number of frames to record.
	 *
	 * @return usize
	 */
	fn get_max_frames(&self) -> usize {
		if let Some(duration) = self.settings.time.duration {
			info!(
				"Recording {} FPS for {} seconds...",
				self.clock.fps, duration
			);
			if self.gifski {
				(duration * (self.clock.fps as f64)) as usize
			} else {
				(((duration * 100.) as u16) / (1e2 / self.clock.fps as f32) as u16)
					as usize
			}
		} else {
			info!("Recording {} FPS...", self.clock.fps);
			usize::MAX
		}
	}

	/**
	 * Record frames synchronously with blocking the current thread.
	 *
	 * @param  input_state (Option)
	 * @return Vector of Image
	 */
	pub fn record_sync(
		&mut self,
		input_state: Option<&InputState>,
	) -> AppResult<Vec<Image>> {
		let mut frames = Vec::new();
		let recording = Arc::new(AtomicBool::new(true));
		let rec_state = recording.clone();
		ctrlc::set_handler(move || {
			rec_state.store(false, Ordering::SeqCst);
		})?;
		self.window.show_countdown();
		if !recording.load(Ordering::SeqCst) {
			return Err(AppError::Cancelled);
		}
		let max_frames = self.get_max_frames();
		while recording.load(Ordering::SeqCst) && frames.len() < max_frames {
			if let Some(state) = input_state {
				if state.check_cancel_keys() {
					debug!("\n");
					warn!("User interrupt detected.");
					return Err(AppError::Cancelled);
				} else if state.check_action() {
					break;
				}
			}
			self.clock.tick();
			match self.window.get_image() {
				Some(image) => frames.push(image),
				None if frames.is_empty() => {
					return Err(AppError::FrameError(String::from(
						"Failed to get image",
					)))
				}
				/* The window is gone, but the frames that were recorded
				 * until now are still worth saving. */
				None => {
					debug!("\n");
					warn!("The recording ended early.");
					break;
				}
			}
			debug!("Frames: {}\r", frames.len());
			io::stdout().flush()?;
		}
		debug!("\n");
		Ok(frames)
	}

	/**
	 * Record frames asynchronously and without blocking.
	 *
	 * @return RecordResult
	 */
	pub fn record_async(mut self) -> RecordResult<Vec<Image>> {
		let mut frames = Vec::new();
		RecordResult::new(
			self.channel.0.clone(),
			thread::spawn(move || {
				self.window.show_countdown();
				let max_frames = self.get_max_frames();
				while self.channel.1.try_recv().is_err() {
					self.clock.tick();
					if frames.len() < max_frames {
						match self.window.get_image() {
							Some(image) => frames.push(image),
							None => {
								debug!("\n");
								warn!("The recording ended early.");
								break;
							}
						}
						debug!("Frames: {}\r", frames.len());
						io::stdout().flush().expect("Failed to flush stdout");
					}
				}
				debug!("\n");
				frames
			}),
		)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::image::geometry::Geometry;
	use crate::record::settings::RecordSettings;
	use crate::window::test::TestWindow;
	use image::Rgba;
	use pretty_assertions::{assert_eq, assert_ne};
	use std::sync::atomic::AtomicUsize;
	use std::thread;
	use std::time::Duration;

	/* Number of frames to hand over before the window is gone */
	const VANISHING_FRAMES: usize = 3;
	/* Set once the vanishing window has been asked for one frame too many */
	static GONE: AtomicBool = AtomicBool::new(false);

	/* Window that stops giving frames, like one that is closed mid-recording */
	#[derive(Debug)]
	struct VanishingWindow;

	impl Capture for VanishingWindow {
		fn get_image(&self) -> Option<Image> {
			static FRAMES: AtomicUsize = AtomicUsize::new(0);
			if FRAMES.fetch_add(1, Ordering::SeqCst) < VANISHING_FRAMES {
				Some(Image::new(
					vec![Rgba::from([255, 255, 255, 0])],
					false,
					Geometry::new(0, 0, 1, 1),
				))
			} else {
				GONE.store(true, Ordering::SeqCst);
				None
			}
		}
		fn show_countdown(&self) {}
		fn release(&self) {}
	}
	#[test]
	fn test_record() {
		let window = TestWindow::default();
		let recorder = Recorder::new(window, 10, false, RecordSettings::default());
		let record = recorder.record_async();
		thread::sleep(Duration::from_millis(200));
		assert!(!record.get().unwrap().is_empty());
		let mut recorder =
			Recorder::new(window, 10, false, RecordSettings::default());
		recorder.settings.time.duration = Some(0.2);
		assert_ne!(0, recorder.record_sync(None).unwrap().len());
	}
	#[test]
	fn test_record_window_gone() {
		let recorder =
			Recorder::new(VanishingWindow, 10, false, RecordSettings::default());
		let record = recorder.record_async();
		/* Waiting for the window instead of for a duration keeps the test off
		 * the speed of the machine that runs it. */
		for _ in 0..200 {
			if GONE.load(Ordering::SeqCst) {
				break;
			}
			thread::sleep(Duration::from_millis(10));
		}
		/* Give the thread the moment it needs to end and drop its receiver. */
		thread::sleep(Duration::from_millis(100));
		/* The thread stopped on its own, but the frames that it did record
		 * are still wanted. */
		assert_eq!(VANISHING_FRAMES, record.get().unwrap().len());
	}
}
