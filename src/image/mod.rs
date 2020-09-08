pub mod geometry;
pub mod padding;
pub mod settings;
use std::fmt;

use crate::image::geometry::Geometry;
use image::{Bgra, ExtendedColorType};
#[cfg(feature = "ski")]
use {
	imgref::{Img, ImgVec},
	rgb::RGBA8,
	std::convert::TryInto,
};

/* Coefficients for transforming sRGB to CIE Y (luminance value) */
const SRGB_LUMA: [f32; 3] = [0.2126, 0.7152, 0.0722];

/* Image data and geometric properties */
#[derive(Clone)]
pub struct Image {
	data: Vec<Bgra<u8>>,
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
		data: Vec<Bgra<u8>>,
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
		self.data.iter().fold(Vec::<u8>::new(), |mut data, bgra| {
			let alpha = if self.alpha_channel { bgra[3] } else { 255 };
			data.extend(&match color_type {
				ExtendedColorType::L1 | ExtendedColorType::L8 => vec![{
					let y = (SRGB_LUMA[0] * bgra[2] as f32
						+ SRGB_LUMA[1] * bgra[1] as f32
						+ SRGB_LUMA[2] * bgra[0] as f32) as u8;
					if color_type == ExtendedColorType::L1 {
						(y >> 7) * 0xFF
					} else {
						y
					}
				}],
				ExtendedColorType::Rgb8 => vec![bgra[2], bgra[1], bgra[0]],
				ExtendedColorType::Rgba16 => vec![
					bgra[2], bgra[2], bgra[1], bgra[1], bgra[0], bgra[0], alpha,
					alpha,
				],
				_ => vec![bgra[2], bgra[1], bgra[0], alpha],
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
				.fold(Vec::<RGBA8>::new(), |mut rgba8, bgra| {
					let alpha = if self.alpha_channel { bgra[3] } else { 255 };
					rgba8.extend(vec![RGBA8 {
						r: bgra[2],
						g: bgra[1],
						b: bgra[0],
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
	fn test_image_mod() {
		let geometry = Geometry::new(0, 0, 200, 200);
		let data: [Bgra<u8>; 2] = [
			Bgra::from([128, 128, 128, 0]),
			Bgra::from([255, 255, 255, 0]),
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
