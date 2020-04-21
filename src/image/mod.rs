pub mod gif;

/* Position and size (2-D) */
#[derive(Clone, Copy, Debug, Default)]
pub struct Geometry {
	pub x: i32,
	pub y: i32,
	pub width: u32,
	pub height: u32,
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
	pub data: Vec<u8>,
	pub geometry: Geometry,
}

pub trait Capture {
	fn get_image(&self) -> Option<Image>;
}
