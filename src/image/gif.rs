use crate::image::{Geometry, Image};
use gif::{Encoder, Frame, Repeat, SetParameter};

pub struct Gif<'a> {
	pub frames: Vec<Frame<'a>>,
	pub geometry: Geometry,
	pub speed: i32,
}

impl Gif<'_> {
	pub fn new(geometry: Geometry, speed: i32) -> Self {
		Self {
			geometry,
			frames: Vec::new(),
			speed,
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
