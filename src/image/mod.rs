pub mod bgra;
pub mod geometry;
pub mod padding;
pub mod settings;

use crate::image::geometry::Geometry;

/* Image data and geometric properties */
#[derive(Clone, Debug)]
pub struct Image {
	pub data: Vec<u8>,
	pub geometry: Geometry,
}

impl Image {
	/**
	 * Create a new Image object.
	 *
	 * @param  data
	 * @param  geometry
	 * @return Image
	 */
	pub fn new(data: Vec<u8>, geometry: Geometry) -> Self {
		Self { data, geometry }
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
