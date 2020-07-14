use crate::gif::decoder::Decoder;
use crate::gif::encoder::Encoder;
#[cfg(feature = "ski")]
use crate::gif::ski::Gif;
#[cfg(not(feature = "ski"))]
use crate::gif::Gif;
use crate::image::Image;
use crate::record::{Record, Recorder};
use crate::settings::AppSettings;
use crate::util::file::FileFormat;
use bytesize::ByteSize;
use image::bmp::BMPEncoder;
use image::farbfeld::FarbfeldEncoder;
use image::jpeg::JPEGEncoder;
use image::png::PNGEncoder;
use image::tiff::TiffEncoder;
use image::ColorType;
use image::ImageEncoder;
use std::fmt::Debug;
use std::fs::{self, File};
use std::io::{Error, Read, Seek, Write};
use std::thread;

/* Window system functions */
pub trait WindowAccess<'a, Window: Record + Send + Sync + Copy + Debug + 'static> {
	fn init(settings: &'a AppSettings<'a>) -> Option<Self>
	where
		Self: Sized;
	fn get_window(&mut self) -> Option<Window>;
}

/* Application and main functionalities */
#[derive(Clone, Copy, Debug)]
pub struct App<'a, Window> {
	window: Window,
	settings: &'a AppSettings<'a>,
}

impl<'a, Window> App<'a, Window>
where
	Window: Record + Send + Sync + Copy + Debug + 'static,
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
		trace!("{:?}", self.window);
		debug!("{:?}", self.settings.save.file);
		debug!("Command: {:?}", self.settings.get_command());
		match self.settings.save.file.format {
			FileFormat::Gif => {
				if self.settings.args.is_present("edit") {
					debug!("{:?}", self.settings.edit);
					self.edit_gif(File::open(self.settings.edit.file)?, output);
				} else {
					debug!("{:?}", self.settings.gif);
					self.save_gif(self.record(), output)?;
				}
			}
			FileFormat::Png => {
				debug!("{:?}", self.settings.png);
				self.capture(
					PNGEncoder::new_with_quality(
						output,
						self.settings.png.compression,
						self.settings.png.filter,
					),
					ColorType::Rgba8,
				)
			}
			FileFormat::Jpg => {
				debug!("{:?}", self.settings.jpg);
				self.capture(
					JPEGEncoder::new_with_quality(
						&mut output,
						self.settings.jpg.quality,
					),
					ColorType::Rgb8,
				)
			}
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
		info!(
			"{} saved to: {:?} ({})",
			self.settings.save.file.format.to_string().to_uppercase(),
			self.settings.save.file.path,
			ByteSize(fs::metadata(&self.settings.save.file.path)?.len())
		);
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
			self.settings
				.get_command()
				.expect("No command specified to run")
				.execute()
				.expect("Failed to run the command");
			image_thread
				.join()
				.expect("Failed to join the image thread")
		} else {
			self.window.show_countdown();
			info!("Capturing an image...");
			self.window.get_image()
		}
		.expect("Failed to get the window image");
		info!(
			"Encoding the image as {}...",
			self.settings.save.file.format.to_string().to_uppercase()
		);
		debug!("{:?}", image);
		debug!("Color type: {:?}", color_type);
		encoder
			.write_image(
				&image.get_data(color_type),
				image.geometry.width,
				image.geometry.height,
				color_type,
			)
			.expect("Failed to encode the image");
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
				.expect("No command specified to run")
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
		Gif::new(
			frames.first().expect("No frames found to save").geometry,
			output,
			self.settings.record.fps,
			self.settings.gif,
		)?
		.save(frames, &self.settings.input_state)?;
		Ok(())
	}

	/**
	 * Edit the GIF and save.
	 *
	 * @param  input
	 * @param  output
	 * @return Result
	 */
	fn edit_gif<Input: Read, Output: Write>(self, input: Input, output: Output) {
		Decoder::new(input, output, self.settings.edit)
			.expect("Failed to decode the GIF")
			.edit()
			.expect("Failed to edit the GIF");
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
