pub mod gif;

#[derive(Debug)]
pub struct Rect {
	pub x: i32,
	pub y: i32,
	pub width: u32,
	pub height: u32,
}

impl Rect {
	pub fn origin(&self) -> Self {
		Rect {
			x: 0,
			y: 0,
			width: self.width,
			height: self.height,
		}
	}
}

#[derive(Debug)]
pub struct Bgr {
	b: u8,
	g: u8,
	r: u8,
	_p: u8,
}

impl Bgr {
	pub fn get_rgb_pixels(bgr_data: &[Bgr]) -> Vec<u8> {
		let mut pixels = Vec::new();
		for bgr in bgr_data {
			pixels.extend(&[bgr.r, bgr.g, bgr.b])
		}
		pixels
	}
}

pub struct Image {
	pub rect: Rect,
	pub data: Vec<u8>,
}
