use crate::image::{Geometry, Image};
use gif::{Encoder, Frame, Repeat, SetParameter};
use std::fs::File;
use std::io::Error;

/* GIF frames, processing speed and geometric properties */
pub struct Gif<'a> {
	pub frames: Vec<Frame<'a>>,
	pub speed: i32,
	pub geometry: Geometry,
}

impl Gif<'_> {
	/**
	 * Create a new Gif object.
	 *
	 * @param  speed
	 * @param  geometry
	 * @return Gif
	 */
	pub fn new(speed: i32, geometry: Geometry) -> Self {
		Self {
			frames: Vec::new(),
			speed,
			geometry,
		}
	}

	/**
	 * Add a frame to the GIF.
	 *
	 * @param image
	 * @param delay
	 */
	pub fn add_frame(&mut self, image: Image, delay: u16) {
		let mut frame = Frame::from_rgb_speed(
			self.geometry.width as u16,
			self.geometry.height as u16,
			&image.data,
			self.speed,
		);
		frame.delay = delay;
		self.frames.push(frame)
	}

	/**
	 * Process the frames and save GIF as a file.
	 *
	 * @param  file
	 * @param  repeat
	 * @return Result
	 */
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
