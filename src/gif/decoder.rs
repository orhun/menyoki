use crate::edit::ImageOps;
use crate::gif::settings::GifSettings;
use crate::image::Image;
use image::error::ImageError;
use image::gif::GifDecoder;
use image::{AnimationDecoder, Bgra};
use std::io::Read;

/* GIF decoder and settings */
pub struct Decoder<'a, Input: Read> {
	decoder: GifDecoder<Input>,
	imageops: ImageOps<'a>,
	settings: GifSettings,
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
		settings: GifSettings,
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
	pub fn update_frames(mut self) -> Result<(Vec<Image>, u32), ImageError> {
		let frames = self.decoder.into_frames().collect_frames()?;
		let first_frame = frames.first().expect("No frames found to edit");
		let fps = ((1e3 / first_frame.delay().numer_denom_ms().0 as f32)
			* self.settings.speed) as u32;
		self.imageops
			.init(first_frame.clone().into_buffer().dimensions());
		let mut images = Vec::new();
		for frame in frames {
			images.push(Image::new(
				self.imageops
					.process(frame.into_buffer())
					.into_vec()
					.chunks(4)
					.map(|rgba| Bgra::from([rgba[2], rgba[1], rgba[0], rgba[3]]))
					.collect(),
				true,
				self.imageops.geometry,
			));
		}
		Ok((images, fps))
	}
}
