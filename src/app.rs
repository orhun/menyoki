use crate::anim::decoder::AnimDecoder;
use crate::anim::Frames;
use crate::apng::ApngEncoder;
use crate::args::Args;
use crate::file::format::FileFormat;
use crate::file::File as FileUtil;
use crate::gif::encoder::{Encoder, EncoderConfig};
#[cfg(feature = "ski")]
use crate::gif::ski::GifskiEncoder;
use crate::gif::GifEncoder;
use crate::image::Image;
use crate::record::Recorder;
use crate::settings::AppSettings;
use crate::view::ImageViewer;
use crate::window::Capture;
use bytesize::ByteSize;
use image::codecs::bmp::BmpEncoder;
use image::codecs::farbfeld::FarbfeldEncoder;
use image::codecs::gif::GifDecoder;
use image::codecs::ico::IcoEncoder;
use image::codecs::jpeg::JpegEncoder;
use image::codecs::openexr::OpenExrEncoder;
use image::codecs::png::PngDecoder;
use image::codecs::png::PngEncoder;
use image::codecs::pnm::{PnmEncoder, PnmSubtype};
use image::codecs::tga::TgaEncoder;
use image::codecs::tiff::TiffEncoder;
use image::codecs::webp::WebPEncoder;
use image::error::{
	ImageError, ImageFormatHint, UnsupportedError, UnsupportedErrorKind,
};
use image::io::Reader;
use image::{
	AnimationDecoder, ColorType, ExtendedColorType, ImageEncoder, ImageFormat,
};
use std::fmt::Debug;
use std::fs::{self, File};
use std::io::{self, Cursor, Read, Seek, Write};
use std::path::Path;
use std::thread;
use thiserror::Error as ThisError;

/* Custom error implementation */
#[derive(Debug, ThisError)]
pub enum AppError {
	#[error("IO error: `{0}`")]
	Io(#[from] std::io::Error),
	#[error("Window system error: `{0}`")]
	WsError(String),
	#[error("Image error: `{0}`")]
	Image(#[from] image::error::ImageError),
	#[error("GIF encoding error: `{0}`")]
	GifEncoding(#[from] gif::EncodingError),
	#[error("PNG encoding error: `{0}`")]
	PngEncoding(#[from] png::EncodingError),
	#[cfg(feature = "ski")]
	#[error("gifski error: `{0}`")]
	Gifski(#[from] gifski::Error),
	#[error("viu error: `{0}`")]
	Viu(#[from] viuer::ViuError),
	#[error("Ctrlc error: `{0}`")]
	Ctrlc(#[from] ctrlc::Error),
	#[error("Frame error: `{0}`")]
	FrameError(String),
	#[error("Command error: `{0}`")]
	CommandError(String),
}

/* Application output and result types */
pub type AppOutput = (Option<Image>, Option<Frames>);
pub type AppResult<T> = std::result::Result<T, AppError>;

/* Application and main functionalities */
#[derive(Clone, Copy, Debug)]
pub struct App<'a, Window> {
	window: Option<Window>,
	settings: &'a AppSettings<'a>,
}

impl<'a, Window> App<'a, Window>
where
	Window: Capture + Send + Sync + Copy + Debug + 'static,
{
	/**
	 * Create a new App object.
	 *
	 * @param  window (Option)
	 * @param  settings
	 * @return App
	 */
	pub fn new(window: Option<Window>, settings: &'a AppSettings<'a>) -> Self {
		Self { window, settings }
	}

	/**
	 * Start the application.
	 *
	 * @return Result
	 */
	pub fn start(&self) -> AppResult<()> {
		trace!("Window: {:?}", self.window);
		debug!("{:?}", self.settings.save.file);
		debug!("Command: {:?}", self.settings.record.get_command());
		if let Some(misc_args) = self.settings.args.subcommand_matches("misc") {
			if let Some(shell) = misc_args.value_of("gen-completions") {
				Args::gen_completions(shell, &mut io::stdout());
			}
		} else if self.settings.args.is_present("split") {
			info!("Reading frames from {:?}...", self.settings.split.file);
			self.split_anim(File::open(&self.settings.split.file)?)?;
			info!(
				"Frames saved to {:?} in {} format.",
				self.settings.split.dir,
				self.settings.save.file.format.as_extension().to_uppercase(),
			);
		} else if self.settings.args.is_present("analyze") {
			debug!("Analyzing the image... ({:?})", self.settings.analyze.file);
			self.analyze_image()?;
		} else if self.settings.args.is_present("view") {
			debug!("Viewing the image... ({:?})", self.settings.view.file);
			self.view_image()?;
		} else if self.settings.save.file.path.to_str() == Some("-") {
			let mut buffer = Cursor::new(Vec::new());
			self.save_output(self.get_app_output()?, &mut buffer)?;
			io::stdout().write_all(&buffer.into_inner())?;
		} else {
			self.save_output(
				self.get_app_output()?,
				File::create(&self.settings.save.file.path)?,
			)?;
			info!(
				"{} saved to: {:?} ({})",
				self.settings.save.file.format.as_extension().to_uppercase(),
				self.settings.save.file.path,
				ByteSize(fs::metadata(&self.settings.save.file.path)?.len())
			);
		}
		Ok(())
	}

	/**
	 * Get the application output.
	 *
	 * @return AppOutput (Result)
	 */
	fn get_app_output(self) -> AppResult<AppOutput> {
		let output = if self.settings.save.file.format.is_animation() {
			(None, Some(self.get_frames()?))
		} else {
			(Some(self.get_image()?), None)
		};
		if let Some(window) = self.window {
			window.release();
		}
		Ok(output)
	}

	/**
	 * Get the image to save.
	 *
	 * @return Image (Result)
	 */
	fn get_image(self) -> AppResult<Image> {
		if self.settings.args.is_present("edit") {
			debug!("{:?}", self.settings.edit);
			info!("Opening {:?}...", self.settings.edit.path);
			self.edit_image(&self.settings.edit.path)
		} else {
			self.capture()
		}
	}

	/**
	 * Get the frames to save.
	 *
	 * @return Frames (Result)
	 */
	fn get_frames(self) -> AppResult<Frames> {
		if self.settings.args.is_present("edit") {
			info!("Reading frames from {:?}...", self.settings.edit.path);
			self.edit_anim(
				File::open(&self.settings.edit.path)?,
				&self.settings.edit.path,
			)
		} else if self.settings.args.is_present("make") {
			info!(
				"Making an animation from {} frames...",
				self.settings.anim.frames.len()
			);
			let mut images = Vec::new();
			for path in &self.settings.anim.frames {
				debug!("Reading a frame from {:?}   \r", path);
				io::stdout().flush()?;
				images.push(self.edit_image(path)?);
			}
			debug!("\n");
			Ok((images, self.settings.anim.fps))
		} else {
			Ok((self.record()?, self.settings.anim.fps))
		}
	}

	/**
	 * Capture the image of window.
	 *
	 * @return Image (Result)
	 */
	fn capture(self) -> AppResult<Image> {
		let window = self.window.ok_or_else(|| {
			AppError::WsError(String::from("Failed to get the window"))
		})?;
		if self.settings.record.command.is_some() {
			let image_thread = thread::spawn(move || {
				window.show_countdown();
				info!("Capturing an image...");
				window.get_image()
			});
			self.settings
				.record
				.get_command()
				.ok_or_else(|| {
					AppError::CommandError(String::from(
						"No command specified to run",
					))
				})?
				.execute()?;
			image_thread
				.join()
				.expect("Failed to join the image thread.")
		} else {
			window.show_countdown();
			info!("Capturing an image...");
			window.get_image()
		}
		.ok_or_else(|| AppError::WsError(String::from("Failed to get image")))
	}

	/**
	 * Start recording the frames.
	 *
	 * @return Vector of Image (Result)
	 */
	fn record(self) -> AppResult<Vec<Image>> {
		let mut recorder = Recorder::new(
			self.window.ok_or_else(|| {
				AppError::WsError(String::from("Failed to get the window"))
			})?,
			self.settings.anim.fps,
			self.settings.anim.gifski.0,
			self.settings.record,
		);
		if self.settings.record.command.is_some() {
			let record = recorder.record_async();
			self.settings
				.record
				.get_command()
				.ok_or_else(|| {
					AppError::CommandError(String::from(
						"No command specified to run",
					))
				})?
				.execute()?;
			Ok(match record.get() {
				Some(frames) => frames.expect("Failed to retrieve the frames."),
				None => Vec::new(),
			})
		} else {
			Ok(recorder.record_sync(
				if self.settings.record.flag.action_keys.is_some() {
					self.settings.input_state
				} else {
					None
				},
			)?)
		}
	}

	/**
	 * Edit and return the image.
	 *
	 * @param  path
	 * @return Image (Result)
	 */
	fn edit_image(self, path: &Path) -> AppResult<Image> {
		let image = Reader::open(path)?
			.with_guessed_format()?
			.decode()?
			.to_rgba8();
		Ok(self
			.settings
			.edit
			.get_imageops()
			.init(image.dimensions())
			.process(image)
			.get_image())
	}

	/**
	 * Analyze the image and return/save the report.
	 *
	 * @return Result
	 */
	fn analyze_image(self) -> AppResult<()> {
		let analyzer = self.settings.analyze.get_analyzer()?;
		if self.settings.save.file.format == FileFormat::Txt {
			fs::write(&self.settings.save.file.path, analyzer.get_report() + "\n")?;
			info!(
				"Report saved to: {:?} ({})",
				self.settings.save.file.path,
				ByteSize(fs::metadata(&self.settings.save.file.path)?.len())
			);
		} else {
			info!("{}#", analyzer.get_colored_report());
		}
		Ok(())
	}

	/**
	 * View the image.
	 *
	 * @return Result
	 */
	fn view_image(self) -> AppResult<()> {
		let image = Reader::open(&self.settings.view.file)?
			.with_guessed_format()?
			.decode()?;
		let viewer = ImageViewer::new(image, &self.settings.view);
		viewer
			.view()
			.map(|(w, h)| debug!("Image dimensions: {}x{}", w, h))
	}

	/**
	 * Return the updated frames after decoding the animation.
	 *
	 * @param  input
	 * @param  path
	 * @return Frames (Result)
	 */
	fn edit_anim<Input: Read>(self, input: Input, path: &Path) -> AppResult<Frames> {
		let format = Reader::open(path)?.with_guessed_format()?.format();
		let frames =
			AnimDecoder::new(self.settings.edit.get_imageops(), &self.settings.anim)
				.update_frames(match format {
					Some(ImageFormat::Gif) => {
						GifDecoder::new(input)?.into_frames().collect_frames()
					}
					Some(ImageFormat::Png) => PngDecoder::new(input)?
						.apng()
						.into_frames()
						.collect_frames(),
					_ => Err(ImageError::Unsupported(
						UnsupportedError::from_format_and_kind(
							ImageFormatHint::Unknown,
							UnsupportedErrorKind::Format(ImageFormatHint::Unknown),
						),
					)),
				}?)?;
		Ok(frames)
	}

	/**
	 * Split animation into frames.
	 *
	 * @param  input
	 * @return Frames (Result)
	 */
	fn split_anim<Input: Read>(self, input: Input) -> AppResult<()> {
		let (frames, fps) = self.edit_anim(input, &self.settings.split.file)?;
		debug!("FPS: {}", fps);
		fs::create_dir_all(&self.settings.split.dir)?;
		for i in 0..frames.len() {
			let path = FileUtil::get_path_with_extension(
				self.settings.split.dir.join(format!("frame_{i}",)),
				&self.settings.save.file.format,
			);
			debug!("Saving to {:?}\r", path);
			io::stdout().flush()?;
			self.save_output((frames.get(i).cloned(), None), File::create(path)?)?;
		}
		debug!("\n");
		Ok(())
	}

	/**
	 * Save the application output.
	 *
	 * @param   app_output
	 * @param   output
	 * @return  Result
	 */
	fn save_output<Output: Write + Seek>(
		&self,
		app_output: AppOutput,
		mut output: Output,
	) -> AppResult<()> {
		let (image, frames) = app_output;
		match self.settings.save.file.format {
			FileFormat::Gif => {
				debug!("{:?}", self.settings.anim);
				self.save_gif(frames, output)
			}
			FileFormat::Apng => {
				debug!("{:?}", self.settings.anim);
				self.save_apng(frames, output)
			}
			FileFormat::Png => self.save_image(
				image,
				PngEncoder::new_with_quality(
					output,
					self.settings.png.compression,
					self.settings.png.filter,
				),
				ExtendedColorType::Rgba8,
			),
			FileFormat::Jpg => self.save_image(
				image,
				JpegEncoder::new_with_quality(
					&mut output,
					self.settings.jpg.quality,
				),
				ExtendedColorType::Rgb8,
			),
			FileFormat::WebP => self.save_image(
				image,
				WebPEncoder::new_with_quality(
					&mut output,
					self.settings.webp.get_quality(),
				),
				ExtendedColorType::Rgb8,
			),
			FileFormat::Bmp => self.save_image(
				image,
				BmpEncoder::new(&mut output),
				ExtendedColorType::Rgba8,
			),
			FileFormat::Ico => self.save_image(
				image,
				IcoEncoder::new(output),
				ExtendedColorType::Rgba8,
			),
			FileFormat::Tiff => self.save_image(
				image,
				TiffEncoder::new(File::create(&self.settings.save.file.path)?),
				ExtendedColorType::Rgba8,
			),
			FileFormat::Tga => self.save_image(
				image,
				TgaEncoder::new(output),
				ExtendedColorType::Rgba8,
			),
			FileFormat::Pnm(_) => self.save_image(
				image,
				PnmEncoder::new(output).with_subtype(self.settings.pnm.subtype),
				match self.settings.pnm.subtype {
					PnmSubtype::Bitmap(_) => ExtendedColorType::L1,
					PnmSubtype::Graymap(_) => ExtendedColorType::L8,
					PnmSubtype::Pixmap(_) => ExtendedColorType::Rgb8,
					PnmSubtype::ArbitraryMap => ExtendedColorType::Rgba8,
				},
			),
			FileFormat::Ff => self.save_image(
				image,
				FarbfeldEncoder::new(output),
				ExtendedColorType::Rgba16,
			),
			FileFormat::Exr => self.save_image(
				image,
				OpenExrEncoder::new(output),
				ExtendedColorType::Rgba32F,
			),
			_ => Ok(()),
		}
	}

	/**
	 * Save the image to a file.
	 *
	 * @param  image (Option)
	 * @param  encoder
	 * @param  color_type
	 * @return Result
	 */
	fn save_image<Encoder: ImageEncoder>(
		self,
		image: Option<Image>,
		encoder: Encoder,
		color_type: ExtendedColorType,
	) -> AppResult<()> {
		let image = image.ok_or_else(|| {
			AppError::WsError(String::from("Failed to get the image"))
		})?;
		if !self.settings.args.is_present("split") {
			info!(
				"Saving the image as {}...",
				self.settings.save.file.format.as_extension().to_uppercase()
			);
			debug!("{:?}", image);
			debug!("{:?}", self.settings.png);
			debug!("{:?}", self.settings.jpg);
			debug!("{:?}", self.settings.pnm);
			debug!("Color type: {:?}", color_type);
		}
		encoder.write_image(
			&image.get_data(color_type),
			image.geometry.width,
			image.geometry.height,
			match color_type {
				ExtendedColorType::L1 | ExtendedColorType::L8 => ColorType::L8,
				ExtendedColorType::Rgb8 => ColorType::Rgb8,
				ExtendedColorType::Rgba16 => ColorType::Rgba16,
				ExtendedColorType::Rgba32F => ColorType::Rgba32F,
				_ => ColorType::Rgba8,
			},
		)?;
		Ok(())
	}

	/**
	 * Save frames to a GIF file.
	 *
	 * @param   frames (Option)
	 * @param   output
	 * @return  Result
	 */
	#[cfg(feature = "ski")]
	fn save_gif<Output: Write>(
		self,
		frames: Option<Frames>,
		output: Output,
	) -> AppResult<()> {
		let (images, fps) = frames.ok_or_else(|| {
			AppError::FrameError(String::from("Failed to get the frames"))
		})?;
		let geometry = images
			.first()
			.ok_or_else(|| {
				AppError::FrameError(String::from("No frames found to save"))
			})?
			.geometry;
		let config = EncoderConfig::new(fps, geometry, output, &self.settings.anim);
		if self.settings.anim.gifski.0 {
			GifskiEncoder::new(config)?.save(images, self.settings.input_state)?;
		} else {
			GifEncoder::new(config)?.save(images, self.settings.input_state)?;
		}
		Ok(())
	}

	/**
	 * Save frames to a GIF file.
	 *
	 * @param   frames (Option)
	 * @param   output
	 * @return  Result
	 */
	#[cfg(not(feature = "ski"))]
	fn save_gif<Output: Write>(
		self,
		frames: Option<Frames>,
		output: Output,
	) -> AppResult<()> {
		let (images, fps) = frames.ok_or_else(|| {
			AppError::FrameError(String::from("Failed to get the frames"))
		})?;
		let geometry = images
			.first()
			.ok_or_else(|| {
				AppError::FrameError(String::from("No frames found to save"))
			})?
			.geometry;
		GifEncoder::new(EncoderConfig::new(
			fps,
			geometry,
			output,
			&self.settings.anim,
		))?
		.save(images, self.settings.input_state)?;
		Ok(())
	}

	/**
	 * Save frames to a APNG file.
	 *
	 * @param   frames (Option)
	 * @param   output
	 * @return  Result
	 */
	fn save_apng<Output: Write>(
		self,
		frames: Option<Frames>,
		output: Output,
	) -> AppResult<()> {
		let images = frames
			.ok_or_else(|| {
				AppError::FrameError(String::from("Failed to get the frames"))
			})?
			.0;
		let geometry = images
			.first()
			.ok_or_else(|| {
				AppError::FrameError(String::from("No frames found to save"))
			})?
			.geometry;
		ApngEncoder::new(
			images.len().try_into().unwrap_or_default(),
			geometry,
			output,
			&self.settings.anim,
		)?
		.save(images, self.settings.input_state)?;
		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::args::matches::ArgMatches;
	use crate::window::test::TestWindow;
	use clap::ArgMatches as Args;
	use std::env;
	use std::path::PathBuf;
	#[test]
	fn test_app_image() -> AppResult<()> {
		let args = Args::default();
		let matches = ArgMatches::new(&args);
		let mut settings = AppSettings::new(&matches);
		let window = TestWindow::default();
		for format in vec![
			FileFormat::Png,
			FileFormat::Jpg,
			FileFormat::Bmp,
			FileFormat::Ico,
			FileFormat::Tiff,
			FileFormat::Tga,
			FileFormat::Pnm(String::from("ppm")),
			FileFormat::Ff,
			FileFormat::Exr,
		] {
			let path =
				FileUtil::get_path_with_extension(PathBuf::from("test.*"), &format);
			settings.save.file.format = format.clone();
			settings.save.file.path = path.clone();
			settings.analyze.file = path.clone();
			let app = App::new(Some(window), &settings);
			app.save_output((app.get_image().ok(), None), File::create(&path)?)?;
			app.edit_image(&path)?;
			app.analyze_image()?;
			fs::remove_file(path)?;
		}
		settings.save.file.path = PathBuf::from("test");
		App::new(Some(window), &settings).start()?;
		fs::remove_file(settings.save.file.path)?;
		Ok(())
	}
	#[test]
	fn test_app_anim() -> AppResult<()> {
		let args = Args::default();
		let matches = ArgMatches::new(&args);
		let mut settings = AppSettings::new(&matches);
		settings.save.file.format = FileFormat::Gif;
		settings.record.command = Some("sleep 0.3");
		settings.anim.cut = (0.1, 0.1);
		let window = TestWindow::default();
		let app = App::new(Some(window), &settings);
		let images = app.get_frames()?.0;
		app.save_gif(Some((images.clone(), 10)), File::create("test.gif")?)?;
		app.edit_anim(File::open("test.gif")?, Path::new("test.gif"))?;
		let dir = env::current_dir()?;
		settings.split.dir = PathBuf::from(dir.to_str().unwrap_or_default());
		settings.split.file = PathBuf::from("test.gif");
		settings.save.file.format = FileFormat::Png;
		let app = App::new(Some(window), &settings);
		app.split_anim(File::open("test.gif")?)?;
		fs::remove_file("test.gif")?;
		app.save_apng(Some((images.clone(), 20)), File::create("test.apng")?)?;
		fs::remove_file("test.apng")?;
		for i in 0..images.len() {
			let path = PathBuf::from(format!("frame_{i}.png"));
			if path.exists() {
				fs::remove_file(path)?;
			}
		}
		Ok(())
	}
}
