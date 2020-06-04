pub mod bgr;

/* Padding properties */
#[derive(Clone, Copy, Debug, Default)]
pub struct Padding {
	top: u32,
	right: u32,
	bottom: u32,
	left: u32,
}

/* Position and size in 2D */
#[derive(Clone, Copy, Debug, Default)]
pub struct Geometry {
	pub x: i32,
	pub y: i32,
	pub width: u32,
	pub height: u32,
}

impl Geometry {
	/**
	 * Create a new Geometry object.
	 *
	 * @param  x
	 * @param  y
	 * @param  width
	 * @param  height
	 * @return Geometry
	 */
	pub fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
		Self {
			x,
			y,
			width,
			height,
		}
	}
}

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
		let geometry = Geometry::new(0, 0, 200, 200);
		let bgr_data: [Bgr; 2] = [Bgr::new(128, 128, 128), Bgr::new(255, 255, 255)];
		let data = Bgr::get_rgb_pixels(&bgr_data);
		let image = Image::new(data, geometry);
		assert!(image.data[0] == 128 && image.data[5] == 255)
	}
}
