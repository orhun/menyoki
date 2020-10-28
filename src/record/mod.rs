pub mod fps;
pub mod settings;
pub mod window;

use crate::image::Image;
use crate::record::fps::FpsClock;
use crate::record::settings::RecordSettings;
use crate::util::state::InputState;
use std::io::{self, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc};
use std::thread;

/* Methods for capturing a window */
pub trait Capture {
	fn get_image(&self) -> Option<Image>;
	fn show_countdown(&self);
	fn release(&self);
}

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
	 * @return Option
	 */
	pub fn get(self) -> Option<thread::Result<T>> {
		if self.sender.send(()).is_ok() {
			Some(self.thread.join())
		} else {
			None
		}
	}
}

/* Recorder with FPS clock and channel */
pub struct Recorder<Window> {
	window: Window,
	clock: FpsClock,
	channel: (mpsc::Sender<()>, mpsc::Receiver<()>),
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
	 * @param  settings
	 * @return Recorder
	 */
	pub fn new(window: Window, fps: u32, settings: RecordSettings) -> Self {
		Self {
			window,
			clock: FpsClock::new(fps),
			channel: mpsc::channel(),
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
			if cfg!(features = "ski") {
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
	pub fn record_sync(&mut self, input_state: Option<&InputState>) -> Vec<Image> {
		let mut frames = Vec::new();
		let recording = Arc::new(AtomicBool::new(true));
		let rec_state = recording.clone();
		ctrlc::set_handler(move || {
			rec_state.store(false, Ordering::SeqCst);
		})
		.expect("Failed to set the signal handler");
		self.window.show_countdown();
		let max_frames = self.get_max_frames();
		while recording.load(Ordering::SeqCst) && frames.len() < max_frames {
			if let Some(state) = input_state {
				if state.check_cancel_keys() {
					frames.clear();
					debug!("\n");
					warn!("User interrupt detected.");
					break;
				} else if state.check_action_keys() {
					break;
				}
			}
			self.clock.tick();
			frames.push(self.window.get_image().expect("Failed to get the image"));
			debug!("Frames: {}\r", frames.len());
			io::stdout().flush().expect("Failed to flush stdout");
		}
		debug!("\n");
		frames
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
						frames.push(
							self.window
								.get_image()
								.expect("Failed to get the image"),
						);
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
	use crate::record::settings::RecordSettings;
	use crate::record::window::TestWindow;
	use pretty_assertions::assert_ne;
	use std::thread;
	use std::time::Duration;
	#[test]
	fn test_record() {
		let window = TestWindow::default();
		let recorder = Recorder::new(window, 10, RecordSettings::default());
		let record = recorder.record_async();
		thread::sleep(Duration::from_millis(200));
		assert!(record.get().unwrap().unwrap().len() > 0);
		let mut recorder = Recorder::new(window, 10, RecordSettings::default());
		recorder.settings.time.duration = Some(0.2);
		assert_ne!(0, recorder.record_sync(Some(&InputState::default())).len());
	}
}
