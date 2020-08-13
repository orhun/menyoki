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
use std::time::Instant;

/* Required window methods for recording */
pub trait Record {
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
	Window: Record + Send + Sync + 'static,
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
		let start_time = Instant::now();
		let duration = if let Some(duration) = self.settings.time.duration {
			info!(
				"Recording {} FPS for {} seconds...",
				self.clock.fps, duration
			);
			duration
		} else {
			info!("Recording {} FPS...", self.clock.fps);
			f64::MAX
		};
		while recording.load(Ordering::SeqCst)
			&& (start_time.elapsed().as_nanos() as f64 / 1e9) < duration
		{
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
		trace!(
			"{:.3} < {:?}",
			start_time.elapsed().as_nanos() as f64 / 1e9,
			self.settings.time.duration
		);
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
				info!("Recording {} FPS...", self.clock.fps);
				while self.channel.1.try_recv().is_err() {
					self.clock.tick();
					frames.push(
						self.window.get_image().expect("Failed to get the image"),
					);
					debug!("Frames: {}\r", frames.len());
					io::stdout().flush().expect("Failed to flush stdout");
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
	use enigo::*;
	use std::thread;
	use std::time::Duration;
	#[test]
	#[ignore]
	fn test_record_mod() {
		let window = TestWindow::default();
		let recorder = Recorder::new(window, 10, RecordSettings::default());
		let record = recorder.record_async();
		thread::sleep(Duration::from_millis(200));
		assert!(record.get().unwrap().unwrap().len() > 0);
		let mut recorder = Recorder::new(window, 10, RecordSettings::default());
		thread::spawn(|| {
			thread::sleep(Duration::from_millis(200));
			Enigo::new().key_down(Key::Escape);
		});
		assert_eq!(0, recorder.record_sync(Some(&InputState::new())).len());
		Enigo::new().key_up(Key::Escape);
	}
}
