use crate::image::{Geometry, Image};
use gif::{Encoder, Frame as GifFrame, Repeat, SetParameter};
use std::fs::File;
use std::io::Error;

pub struct Frame {
	image: Image,
	delay: u16,
}

impl Frame {
	pub fn new(image: Image, delay: u16) -> Self {
		Self { image, delay }
	}
	pub fn get(&self, speed: i32) -> GifFrame {
		let mut frame = GifFrame::from_rgb_speed(
			self.image.geometry.width as u16,
			self.image.geometry.height as u16,
			&self.image.data,
			speed,
		);
		frame.delay = self.delay;
		frame
	}
}

/* GIF frames, processing speed and geometric properties */
pub struct Gif {
	encoder: Encoder<File>,
	speed: i32,
}

impl Gif {
	/**
	 * Create a new Gif object.
	 *
	 * @param  speed
	 * @param  geometry
	 * @return Gif
	 */
	pub fn new(
		file: File,
		geometry: Geometry,
		speed: i32,
		repeat: Repeat,
	) -> Result<Self, Error> {
		let mut encoder =
			Encoder::new(file, geometry.width as u16, geometry.height as u16, &[])?;
		encoder.set(repeat)?;
		Ok(Self { encoder, speed })
	}

	/**
	 * Process the frames and save GIF as a file.
	 *
	 * @param  file
	 * @param  repeat
	 * @return Result
	 */
	pub fn write_frame(&mut self, frame: Frame) -> Result<(), Error> {
		self.encoder.write_frame(&frame.get(self.speed))?;
		Ok(())
	}
}
