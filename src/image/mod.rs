pub mod geometry;
pub mod padding;
pub mod settings;

use crate::image::geometry::Geometry;
use image::{Bgra, ColorType};

/* Image data and geometric properties */
#[derive(Clone, Debug)]
pub struct Image {
	data: Vec<Bgra<u8>>,
	alpha_channel: bool,
	pub geometry: Geometry,
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
	pub fn get_data(&self, color_type: ColorType) -> Vec<u8> {
		self.data.iter().fold(Vec::<u8>::new(), |mut rgba8, bgra| {
			let alpha = if self.alpha_channel { bgra[3] } else { 255 };
			rgba8.extend(&match color_type {
				ColorType::Rgb8 => vec![bgra[2], bgra[1], bgra[0]],
				ColorType::Rgba16 => vec![
					bgra[2], bgra[2], bgra[1], bgra[1], bgra[0], bgra[0], alpha,
					alpha,
				],
				_ => vec![bgra[2], bgra[1], bgra[0], alpha],
			});
			rgba8
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn test_image_mod() {
		let geometry = Geometry::new(0, 0, 200, 200, None);
		let data: [Bgra<u8>; 2] = [
			Bgra::from([128, 128, 128, 0]),
			Bgra::from([255, 255, 255, 0]),
		];
		let image = Image::new(data.to_vec(), false, geometry);
		assert_eq!(6, image.get_data(ColorType::Rgb8).len());
		assert_eq!(8, image.get_data(ColorType::Rgba8).len());
		assert_eq!(16, image.get_data(ColorType::Rgba16).len());
		assert_eq!(255, image.get_data(ColorType::Rgb8)[4]);
		assert_eq!(255, image.get_data(ColorType::Rgba8)[5]);
		assert_eq!(128, image.get_data(ColorType::Rgba16)[5]);
	}
}
