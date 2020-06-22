use crate::gif::{Frame, Gif};
use crate::record::{Record, Recorder};
use crate::settings::AppSettings;
use crate::util::file::FileFormat;
use image::bmp::BMPEncoder;
use image::farbfeld::FarbfeldEncoder;
use image::jpeg::JPEGEncoder;
use image::png::PNGEncoder;
use image::ColorType;
use image::ImageEncoder;
use std::io::{Error, Write};

/* Application and main functionalities */
#[derive(Clone, Copy, Debug)]
pub struct App<'a, Window> {
	settings: &'a AppSettings<'a>,
	window: Window,
}

impl<'a, Window> App<'a, Window>
where
	Window: Record + Send + Sync + Copy + 'static,
{
	/**
	 * Create a new App object.
	 *
	 * @param  settings
	 * @param  window
	 * @return App
	 */
	pub fn new(settings: &'a AppSettings<'a>, window: Window) -> Self {
		Self { settings, window }
	}

	/**
	 * Start the application.
	 *
	 * @param  output
	 * @return Result
	 */
	pub fn start<Output: Write>(&self, mut output: Output) -> Result<(), Error> {
		match self.settings.save.file.format {
			FileFormat::Gif => {
				let frames = self.record();
				info!("frames: {}", frames.len());
				self.save_gif(frames, output)?;
			}
			FileFormat::Png => self.capture(PNGEncoder::new_with_quality(
				output,
				self.settings.png.compression,
				self.settings.png.filter,
			)),
			FileFormat::Jpg => self.capture(JPEGEncoder::new_with_quality(
				&mut output,
				self.settings.jpg.quality,
			)),
			FileFormat::Bmp => self.capture(BMPEncoder::new(&mut output)),
			FileFormat::Ff => self.capture(FarbfeldEncoder::new(output)),
		}
		Ok(())
	}

	/**
	 * Capture the image of window and save it to a file.
	 *
	 * @param encoder
	 */
	fn capture<Encoder: ImageEncoder>(self, encoder: Encoder) {
		self.window.show_countdown();
		let image = self
			.window
			.get_image()
			.expect("Failed to get the window image");
		encoder
			.write_image(
				&image.data,
				image.geometry.width,
				image.geometry.height,
				ColorType::Rgba8,
			)
			.expect("Failed to encode the image");
	}

	/**
	 * Start recording the frames.
	 *
	 * @return Vector of Frame
	 */
	fn record(self) -> Vec<Frame> {
		let mut recorder = Recorder::new(self.settings.record, self.window);
		if self.settings.args.is_present("command") {
			let record = recorder.record_async();
			self.settings
				.get_command()
				.execute()
				.expect("Failed to run the command");
			match record.get() {
				Some(frames) => frames.expect("Failed to retrieve the frames"),
				None => Vec::new(),
			}
		} else {
			recorder.record_sync(&self.settings.input_state)
		}
	}

	/**
	 * Save frames to a GIF file.
	 *
	 * @param  frames
	 * @param  output
	 * @return Result
	 */
	fn save_gif<Output: Write>(
		self,
		frames: Vec<Frame>,
		output: Output,
	) -> Result<(), Error> {
		let mut gif = Gif::new(
			frames
				.first()
				.expect("No frames found to save")
				.image
				.geometry,
			output,
			self.settings.gif,
		)?;
		gif.save(frames, &self.settings.input_state)?;
		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::args::Args;
	use crate::image::Image;
	use crate::test::TestWindow;
	use crate::util::cmd::Command;
	use std::fs::File;
	#[test]
	fn test_app_mod() -> Result<(), Error> {
		let args = Args::parse();
		let settings = AppSettings::new(&args);
		let output = File::create(&settings.save.file.name).unwrap();
		let window = TestWindow::default();
		let app = App::new(&settings, window);
		let mut frames = app.record();
		frames.push(Frame::new(Image::new(vec![0, 0, 0], window.geometry), 0));
		app.save_gif(frames, output)?;
		Command::new(String::from("rm"), vec![String::from("t.gif")]).execute()?;
		Ok(())
	}
}
