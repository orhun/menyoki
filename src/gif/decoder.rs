use crate::edit::ImageOps;
use crate::gif::encoder::Frames;
use crate::gif::settings::GifSettings;
use image::error::ImageError;
use image::gif::GifDecoder as BaseDecoder;
use image::AnimationDecoder;
use image::Frame;
use std::convert::TryInto;
use std::io::{self, Read, Write};

/* GIF decoder and settings */
pub struct GifDecoder<'a, Input: Read> {
	decoder: BaseDecoder<Input>,
	imageops: ImageOps<'a>,
	settings: &'a GifSettings,
}

impl<'a, Input: Read> GifDecoder<'a, Input> {
	/**
	 * Create a new GifDecoder object.
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
			decoder: BaseDecoder::new(input)?,
			imageops,
			settings,
		})
	}

	/**
	 * Update frames to cut the duration.
	 *
	 * @param  frames
	 * @param  cut
	 * @param  fps
	 * @return Vector of Frame
	 */
	fn cut_duration(
		frames: &mut Vec<Frame>,
		cut: (f32, f32),
		fps: u32,
	) -> Vec<Frame> {
		if cut != (0., 0.) {
			let (start, end) = cut;
			let frame_delay = 1000_u32.checked_div(fps).unwrap_or_default() as f32;
			frames
				.drain(
					((start / frame_delay) as u32)
						.try_into()
						.unwrap_or_default()
						..frames.len().saturating_sub((end / frame_delay) as usize),
				)
				.collect()
		} else {
			frames.to_vec()
		}
	}

	/**
	 * Update and return the frames.
	 *
	 * @return Result
	 */
	pub fn update_frames(mut self) -> Result<Frames, ImageError> {
		let mut frames = self.decoder.into_frames().collect_frames()?;
		let first_frame = frames.first().expect("No frames found to process");
		self.imageops
			.init(first_frame.clone().into_buffer().dimensions());
		let fps = ((1e3 / first_frame.delay().numer_denom_ms().0 as f32)
			* self.settings.speed) as u32;
		let frames = Self::cut_duration(&mut frames, self.settings.cut, fps);
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
