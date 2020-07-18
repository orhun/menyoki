use crate::gif::settings::GifSettings;
use crate::image::geometry::Geometry;
use crate::image::Image;
use image::error::ImageError;
use image::gif::GifDecoder;
use image::AnimationDecoder;
use image::Bgra;
use std::io::Read;

/* GIF decoder and settings */
pub struct Decoder<'a, Input: Read> {
	decoder: GifDecoder<Input>,
	settings: GifSettings<'a>,
}

impl<'a, Input: Read> Decoder<'a, Input> {
	/**
	 * Create a new Decoder object.
	 *
	 * @param  input
	 * @param  settings
	 * @return Result
	 */
	pub fn new(input: Input, settings: GifSettings<'a>) -> Result<Self, ImageError> {
		Ok(Self {
			decoder: GifDecoder::new(input)?,
			settings,
		})
	}

	/**
	 * Update and return the frames.
	 *
	 * @return Result
	 */
	pub fn edit(self) -> Result<(Vec<Image>, u32), ImageError> {
		let frames = self.decoder.into_frames().collect_frames()?;
		let first_frame = frames.first().expect("No frames found to edit");
		let fps = ((1e3 / first_frame.delay().numer_denom_ms().0 as f32)
			* (self.settings.speed / 1e2)) as u32;
		let (width, height) = first_frame.clone().into_buffer().dimensions();
		let geometry = Geometry::new(0, 0, width, height);
		let mut images = Vec::new();
		for frame in frames {
			images.push(Image::new(
				frame
					.into_buffer()
					.into_vec()
					.chunks(4)
					.map(|rgba| Bgra::from([rgba[2], rgba[1], rgba[0], rgba[3]]))
					.collect(),
				true,
				geometry,
			));
		}
		Ok((images, fps))
	}
}
