pub mod geometry;
pub mod padding;
pub mod settings;
use std::fmt;

use crate::image::geometry::Geometry;
use image::{ExtendedColorType, Rgba};
#[cfg(feature = "ski")]
use {
	imgref::{Img, ImgVec},
	rgb::RGBA8,
};

/* Coefficients for transforming sRGB to CIE Y (luminance value) */
const SRGB_LUMA: [f32; 3] = [0.2126, 0.7152, 0.0722];

/* Image data and geometric properties */
#[derive(Clone)]
pub struct Image {
	data: Vec<Rgba<u8>>,
	alpha_channel: bool,
	pub geometry: Geometry,
}

/* Debug implementation for programmer-facing output */
impl fmt::Debug for Image {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("Image")
			.field("data_len", &self.data.len())
			.field("alpha_channel", &self.alpha_channel)
			.field("geometry", &self.geometry)
			.finish()
	}
}

impl Image {
	/**
	 * Create a new Image object.
	 *
	 * @param  data
	 * @param  alpha_channel
	 * @param  geometry
	 * @return Image
	 */
	pub fn new(
		data: Vec<Rgba<u8>>,
		alpha_channel: bool,
		geometry: Geometry,
	) -> Self {
		Self {
			data,
			alpha_channel,
			geometry,
		}
	}

	/**
	 * Get image data in the given color type.
	 *
	 * @param  color_type
	 * @return Vector of u8
	 */
	pub fn get_data(&self, color_type: ExtendedColorType) -> Vec<u8> {
		self.data.iter().fold(Vec::<u8>::new(), |mut data, rgba| {
			let alpha = if self.alpha_channel { rgba[3] } else { 255 };
			data.extend(&match color_type {
				ExtendedColorType::L1 | ExtendedColorType::L8 => vec![{
					let y = (SRGB_LUMA[0] * rgba[0] as f32
						+ SRGB_LUMA[1] * rgba[1] as f32
						+ SRGB_LUMA[2] * rgba[2] as f32) as u8;
					if color_type == ExtendedColorType::L1 {
						(y >> 7) * 0xFF
					} else {
						y
					}
				}],
				ExtendedColorType::Rgb8 => vec![rgba[0], rgba[1], rgba[2]],
				ExtendedColorType::Rgba16 => vec![
					rgba[0], rgba[0], rgba[1], rgba[1], rgba[2], rgba[2], alpha,
					alpha,
				],
				_ => vec![rgba[0], rgba[1], rgba[2], alpha],
			});
			data
		})
	}

	/**
	 * Get an Img Vector from the image data.
	 *
	 * @return ImgVec
	 */
	#[cfg(feature = "ski")]
	pub fn get_img_vec(&self) -> ImgVec<RGBA8> {
		Img::new(
			self.data
				.iter()
				.fold(Vec::<RGBA8>::new(), |mut rgba8, rgba| {
					let alpha = if self.alpha_channel { rgba[3] } else { 255 };
					rgba8.extend(vec![RGBA8 {
						r: rgba[0],
						g: rgba[1],
						b: rgba[2],
						a: alpha,
					}]);
					rgba8
				}),
			self.geometry.width.try_into().unwrap_or_default(),
			self.geometry.height.try_into().unwrap_or_default(),
		)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use pretty_assertions::assert_eq;
	#[test]
	fn test_image() {
		let geometry = Geometry::new(0, 0, 200, 200);
		let data: [Rgba<u8>; 2] = [
			Rgba::from([128, 128, 128, 0]),
			Rgba::from([255, 255, 255, 0]),
		];
		let image = Image::new(data.to_vec(), false, geometry);
		assert_eq!(
			format!("{:?}", image),
			"Image { data_len: 2, alpha_channel: false, \
			geometry: Geometry { x: 0, y: 0, width: 200, height: 200 } }"
		);
		assert_eq!(2, image.get_data(ExtendedColorType::L1).len());
		assert_eq!(2, image.get_data(ExtendedColorType::L8).len());
		assert_eq!(6, image.get_data(ExtendedColorType::Rgb8).len());
		assert_eq!(8, image.get_data(ExtendedColorType::Rgba8).len());
		assert_eq!(16, image.get_data(ExtendedColorType::Rgba16).len());
		assert_eq!(255, image.get_data(ExtendedColorType::L1)[0]);
		assert_eq!(255, image.get_data(ExtendedColorType::L8)[1]);
		assert_eq!(255, image.get_data(ExtendedColorType::Rgb8)[4]);
		assert_eq!(255, image.get_data(ExtendedColorType::Rgba8)[5]);
		assert_eq!(128, image.get_data(ExtendedColorType::Rgba16)[5]);
	}
}
