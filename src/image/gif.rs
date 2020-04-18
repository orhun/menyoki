use crate::image::{Geometry, Image};
use gif::{Encoder, Frame, Repeat, SetParameter};
use std::fs::File;
use std::io::Error;

pub struct Gif<'a> {
	pub frames: Vec<Frame<'a>>,
	pub speed: i32,
	pub geometry: Geometry,
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

	pub fn save(&self, file: File, repeat: Repeat) -> Result<(), Error> {
		let mut encoder = Encoder::new(
			file,
			self.geometry.width as u16,
			self.geometry.height as u16,
			&[],
		)?;
		encoder.set(repeat)?;
		for frame in &self.frames {
			encoder.write_frame(&frame)?;
		}
		Ok(())
	}
}
