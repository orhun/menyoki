use crate::image::{Geometry, Image};
use gif::{Encoder, Frame, Repeat, SetParameter};

pub struct Gif<'a> {
	pub frames: Vec<Frame<'a>>,
	pub speed: i32,
	pub geometry: Geometry
}

impl Gif<'_> {
	pub fn new(speed: i32, geometry: Geometry) -> Self {
		Self {
			frames: Vec::new(),
			speed,
			geometry,
		}
	}
	pub fn add_frame(&mut self, image: Image) {
		self.frames.push(Frame::from_rgb_speed(
			self.geometry.width as u16,
			self.geometry.height as u16,
			&image.data,
			self.speed,
		))
	}
}
