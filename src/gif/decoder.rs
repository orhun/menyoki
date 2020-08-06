use crate::edit::ImageOps;
use crate::gif::encoder::Frames;
use crate::gif::settings::GifSettings;
use image::error::ImageError;
use image::gif::GifDecoder;
use image::AnimationDecoder;
use std::io::{self, Read, Write};

/* GIF decoder and settings */
pub struct Decoder<'a, Input: Read> {
	decoder: GifDecoder<Input>,
	imageops: ImageOps<'a>,
	settings: &'a GifSettings,
}

impl<'a, Input: Read> Decoder<'a, Input> {
	/**
	 * Create a new Decoder object.
	 *
	 * @param  input
	 * @param  imageops
	 * @param  settings
	 * @return Result
	 */
	pub fn new(
		input: Input,
		imageops: ImageOps<'a>,
		settings: &'a GifSettings,
	) -> Result<Self, ImageError> {
		Ok(Self {
			decoder: GifDecoder::new(input)?,
			imageops,
			settings,
		})
	}

	/**
	 * Update and return the frames.
	 *
	 * @return Result
	 */
	pub fn update_frames(mut self) -> Result<Frames, ImageError> {
		let frames = self.decoder.into_frames().collect_frames()?;
		let first_frame = frames.first().expect("No frames found to process");
		let fps = ((1e3 / first_frame.delay().numer_denom_ms().0 as f32)
			* self.settings.speed) as u32;
		self.imageops
			.init(first_frame.clone().into_buffer().dimensions());
		let mut images = Vec::new();
		for (i, frame) in frames.iter().enumerate() {
			let percentage = ((i + 1) as f64 / frames.len() as f64) * 100.;
			info!("Processing the GIF frames... ({:.1}%)\r", percentage);
			debug!(
				"Processing the GIF frames... ({:.1}%) [{}/{}]\r",
				percentage,
				i + 1,
				frames.len()
			);
			io::stdout().flush().expect("Failed to flush stdout");
			images.push(
				self.imageops
					.process(frame.clone().into_buffer())
					.get_image(),
			);
		}
		info!("\n");
		Ok((images, fps))
	}
}
