pub mod gif;

/* Position and size (2-D) */
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

/* BGR color fields (and padding for XImage casting) */
#[derive(Debug)]
pub struct Bgr {
	b: u8,
	g: u8,
	r: u8,
	_p: u8,
}

impl Bgr {
	/**
	 * Convert a BGR slice to RGB pixel vector.
	 *
	 * @param  bgr_data
	 * @return Vector of u8
	 */
	pub fn get_rgb_pixels(bgr_data: &[Bgr]) -> Vec<u8> {
		let mut pixels = Vec::new();
		for bgr in bgr_data {
			pixels.extend(&[bgr.r, bgr.g, bgr.b])
		}
		pixels
	}
}

/* Image data and geometric properties */
#[derive(Clone, Debug)]
pub struct Image {
	data: Vec<u8>,
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

/* Image capture methods */
pub trait Capture {
	fn get_image(&self) -> Option<Image>;
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn test_image_mod() {
		let geometry = Geometry::new(0, 0, 200, 200);
		let bgr_data: [Bgr; 2] = [
			Bgr {
				b: 128,
				g: 128,
				r: 128,
				_p: 0,
			},
			Bgr {
				b: 255,
				g: 255,
				r: 255,
				_p: 0,
			},
		];
		let data = Bgr::get_rgb_pixels(&bgr_data);
		let image = Image::new(data, geometry);
		assert!(image.data[0] == 128 && image.data[5] == 255)
	}
}
