use crate::image::{Geometry, Image};
use gif::{Encoder, Frame as GifFrame, Repeat, SetParameter};
use std::fs::File;
use std::io::Error;

/* Frame image and delay (in units of 10 ms) */
#[derive(Clone, Debug)]
pub struct Frame {
	image: Image,
	delay: u16,
}

impl Frame {
	/**
	 * Create a new Frame object.
	 *
	 * @param  image
	 * @param  delay
	 * @return Frame
	 */
	pub fn new(image: Image, delay: u16) -> Self {
		Self { image, delay }
	}

	/**
	 * Get a GIF frame from the Frame object.
	 *
	 * @param  speed
	 * @return GifFrame
	 */
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

/* GIF encoder, processing speed and FPS values */
pub struct Gif {
	encoder: Encoder<File>,
	speed: i32,
}

impl Gif {
	/**
	 * Create a new Gif object.
	 *
	 * @param  file
	 * @param  geometry
	 * @param  fps
	 * @param  speed
	 * @param  repeat
	 * @return Result (Gif)
	 */
	pub fn new(
		file: File,
		geometry: Geometry,
		fps: u32,
		speed: i32,
		repeat: Repeat,
	) -> Result<Self, Error> {
		let mut encoder =
			Encoder::new(file, geometry.width as u16, geometry.height as u16, &[])?;
		encoder.set(repeat)?;
		Ok(Self {
			encoder,
			speed,
		})
	}

	/**
	 * Write a frame to the GIF file.
	 *
	 * @param  frame
	 * @return Result
	 */
	pub fn write_frame(&mut self, frame: Frame) -> Result<(), Error> {
		self.encoder.write_frame(&frame.get(self.speed))?;
		Ok(())
	}
}
