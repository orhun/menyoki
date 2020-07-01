use crate::gif::encoder::Encoder;
#[cfg(feature = "ski")]
use crate::gif::ski::Gif;
#[cfg(not(feature = "ski"))]
use crate::gif::Gif;
use crate::image::Image;
use crate::record::{Record, Recorder};
use crate::settings::AppSettings;
use crate::util::file::FileFormat;
use image::bmp::BMPEncoder;
use image::farbfeld::FarbfeldEncoder;
use image::jpeg::JPEGEncoder;
use image::png::PNGEncoder;
use image::tiff::TiffEncoder;
use image::ColorType;
use image::ImageEncoder;
use std::io::{Error, Seek, Write};
use std::thread;

/* Application and main functionalities */
#[derive(Clone, Copy, Debug)]
pub struct App<'a, Window> {
	window: Window,
	settings: &'a AppSettings<'a>,
}

impl<'a, Window> App<'a, Window>
where
	Window: Record + Send + Sync + Copy + 'static,
{
	/**
	 * Create a new App object.
	 *
	 * @param  window
	 * @param  settings
	 * @return App
	 */
	pub fn new(window: Window, settings: &'a AppSettings<'a>) -> Self {
		Self { window, settings }
	}

	/**
	 * Start the application.
	 *
	 * @param  output
	 * @return Result
	 */
	pub fn start<Output: Write + Seek>(
		&self,
		mut output: Output,
	) -> Result<(), Error> {
		match self.settings.save.file.format {
			FileFormat::Gif => {
				self.save_gif(self.record(), output)?;
			}
			FileFormat::Png => self.capture(
				PNGEncoder::new_with_quality(
					output,
					self.settings.png.compression,
					self.settings.png.filter,
				),
				ColorType::Rgba8,
			),
			FileFormat::Jpg => self.capture(
				JPEGEncoder::new_with_quality(
					&mut output,
					self.settings.jpg.quality,
				),
				ColorType::Rgb8,
			),
			FileFormat::Bmp => {
				self.capture(BMPEncoder::new(&mut output), ColorType::Rgba8)
			}
			FileFormat::Tiff => {
				self.capture(TiffEncoder::new(output), ColorType::Rgba8)
			}
			FileFormat::Ff => {
				self.capture(FarbfeldEncoder::new(output), ColorType::Rgba16)
			}
		}
		Ok(())
	}

	/**
	 * Capture the image of window and save it to a file.
	 *
	 * @param encoder
	 * @param color_type
	 */
	fn capture<Encoder: ImageEncoder>(
		self,
		encoder: Encoder,
		color_type: ColorType,
	) {
		let image = if self.settings.args.is_present("command") {
			let window = self.window;
			let image_thread = thread::spawn(move || {
				window.show_countdown();
				info!("Capturing an image...");
				window.get_image()
			});
			info!("Running the command...");
			self.settings
				.get_command()
				.execute()
				.expect("Failed to run the command");
			image_thread
				.join()
				.expect("Failed to join the image thread")
		} else {
			info!("Capturing an image...");
			self.window.get_image()
		}
		.expect("Failed to get the window image");
		info!(
			"Encoding the image as {}...",
			self.settings.save.file.format.to_string().to_uppercase()
		);
		encoder
			.write_image(
				&image.get_data(color_type),
				image.geometry.width,
				image.geometry.height,
				color_type,
			)
			.expect("Failed to encode the image");
		info!("Image saved to: {}", self.settings.save.file.name);
	}

	/**
	 * Start recording the frames.
	 *
	 * @return Vector of Image
	 */
	fn record(self) -> Vec<Image> {
		let mut recorder = Recorder::new(self.window, self.settings.record);
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
		frames: Vec<Image>,
		output: Output,
	) -> Result<(), Error> {
		info!("frames: {}", frames.len());
		Gif::new(
			frames.first().expect("No frames found to save").geometry,
			output,
			self.settings.record.fps,
			self.settings.gif,
		)?
		.save(frames, &self.settings.input_state)?;
		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::args::Args;
	use crate::image::Image;
	use crate::test::TestWindow;
	use crate::util::file::FileFormat;
	use image::Bgra;
	use std::io::Cursor;
	#[test]
	fn test_app_mod() -> Result<(), Error> {
		let args = Args::parse();
		let mut settings = AppSettings::new(&args);
		let window = TestWindow::default();
		for format in vec![
			FileFormat::Png,
			FileFormat::Jpg,
			FileFormat::Bmp,
			FileFormat::Tiff,
			FileFormat::Ff,
		] {
			settings.save.file.format = format;
			let app = App::new(window, &settings);
			let mut output = Vec::new();
			app.start(Cursor::new(&mut output))?;
			assert!(output.len() > 0);
		}
		settings.save.file.format = FileFormat::Gif;
		let app = App::new(window, &settings);
		let mut images = app.record();
		images.push(Image::new(
			vec![Bgra::from([0, 0, 0, 0])],
			false,
			window.geometry,
		));
		let mut output = Vec::new();
		app.save_gif(images, &mut output)?;
		assert!(output.len() > 0);
		Ok(())
	}
}
