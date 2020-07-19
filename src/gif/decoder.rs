use crate::gif::settings::GifSettings;
use crate::image::geometry::Geometry;
use crate::image::Image;
use image::error::ImageError;
use image::gif::GifDecoder;
use image::imageops::{self, FilterType};
use image::{AnimationDecoder, Bgra};
use std::convert::TryInto;
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
		let (mut width, mut height) = if !self.settings.resize.is_zero() {
			(self.settings.resize.width, self.settings.resize.height)
		} else {
			first_frame.clone().into_buffer().dimensions()
		};
		if self.settings.ratio > 0. && self.settings.ratio != 1. {
			let (w, h) = (width, height);
			width = (w as f32 * self.settings.ratio) as u32;
			height = (h as f32 * self.settings.ratio) as u32;
		}
		let geometry =
			Geometry::new(0, 0, width, height).with_padding(self.settings.padding);
		let mut images = Vec::new();
		for frame in frames {
			let mut image = frame.into_buffer();
			if !self.settings.resize.is_zero()
				|| (self.settings.ratio > 0. && self.settings.ratio != 1.)
			{
				image = imageops::resize(
					&image,
					geometry.width,
					geometry.height,
					FilterType::Lanczos3,
				);
			}
			let sub_image = if !self.settings.padding.is_zero() {
				imageops::crop(
					&mut image,
					geometry.x.try_into().unwrap_or_default(),
					geometry.y.try_into().unwrap_or_default(),
					geometry.width,
					geometry.height,
				)
				.to_image()
			} else {
				image
			};
			images.push(Image::new(
				sub_image
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
