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
	use crate::image::bgr::Bgr;
	#[test]
	fn test_image_mod() {
		let geometry = Geometry::new(0, 0, 200, 200, None);
		let bgr_data: [Bgr; 2] = [Bgr::new(128, 128, 128), Bgr::new(255, 255, 255)];
		let data = Bgr::get_rgb_pixels(&bgr_data);
		let image = Image::new(data, geometry);
		assert!(image.data[0] == 128 && image.data[5] == 255)
	}
}
