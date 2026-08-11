use crate::image::geometry::Geometry;
use crate::image::Image;
use crate::wayland::display::Display;
use crate::window::Capture;
use std::fmt;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

/* Wayland output to capture, with its geometric properties */
#[derive(Clone, Copy, Debug)]
pub struct Window {
	display: &'static Display,
	output: usize,
	pub geometry: Geometry,
	pub area: Geometry,
}

/* Display implementation for user-facing output */
impl fmt::Display for Window {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"\n Output name   -> \"{}\"\n Output size   -> [{}x{}]",
			self.get_name().unwrap_or_else(|| String::from("(?)")),
			self.geometry.width,
			self.geometry.height,
		)
	}
}

impl Window {
	/**
	 * Create a new Window object.
	 *
	 * @param  display
	 * @param  output
	 * @param  geometry
	 * @param  area
	 * @return Window
	 */
	pub fn new(
		display: &'static Display,
		output: usize,
		geometry: Geometry,
		area: Geometry,
	) -> Self {
		Self {
			display,
			output,
			geometry,
			area,
		}
	}

	/**
	 * Get the name of the output.
	 *
	 * @return String (Option)
	 */
	pub fn get_name(&self) -> Option<String> {
		self.display
			.outputs
			.get(self.output)
			.map(|output| output.name.clone())
	}
}

/* Capture implementation for a Wayland output */
impl Capture for Window {
	/**
	 * Get the image of the output.
	 *
	 * @return Image (Option)
	 */
	fn get_image(&self) -> Option<Image> {
		match self.display.capture(self.output, self.area) {
			Ok(data) => Some(Image::new(
				data,
				self.display.settings.flag.alpha,
				self.area,
			)),
			Err(e) => {
				error!("{}", e);
				None
			}
		}
	}

	/* Show a countdown before capturing. */
	fn show_countdown(&self) {
		let countdown = self.display.settings.time.countdown;
		if countdown == 0 {
			return;
		}
		for i in (1..=countdown).rev() {
			info!(
				"Starting in {}{}\r",
				i,
				if countdown > 9 { " " } else { "" }
			);
			io::stdout().flush().expect("Failed to flush stdout");
			thread::sleep(Duration::from_secs(1));
		}
		info!("\r");
	}

	/* Nothing to release, the connection is closed when the process exits. */
	fn release(&self) {
		trace!("Display closed.");
	}
}
