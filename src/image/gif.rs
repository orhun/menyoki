use crate::image::{Geometry, Image};
use gif::{Encoder, Frame as GifFrame, Repeat, SetParameter};
use std::fs::File;
use std::io::Error;

pub struct FrameSettings {
	repeat: i32,
	speed: u32,
}

impl FrameSettings {
	pub fn new(repeat: i32, speed: u32) -> Self {
		Self { repeat, speed }
	}
}

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

/* GIF encoder and processing speed */
pub struct Gif {
	encoder: Encoder<File>,
	settings: FrameSettings,
}

impl Gif {
	/**
	 * Create a new Gif object.
	 *
	 * @param  file
	 * @param  geometry
	 * @param  speed
	 * @param  repeat
	 * @return Result (Gif)
	 */
	pub fn new(
		file: File,
		geometry: Geometry,
		settings: FrameSettings,
	) -> Result<Self, Error> {
		let mut encoder =
			Encoder::new(file, geometry.width as u16, geometry.height as u16, &[])?;
		encoder.set(match settings.repeat {
			n if n >= 0 => Repeat::Finite(n as u16),
			_ => Repeat::Infinite,
		})?;
		Ok(Self { encoder, settings })
	}

	/**
	 * Write frames to the GIF file.
	 *
	 * @param  frames
	 * @return Result
	 */
	pub fn save(&mut self, frames: Vec<Frame>) -> Result<(), Error> {
		for frame in frames {
			self.encoder
				.write_frame(&frame.get(self.settings.speed as i32))?;
		}
		Ok(())
	}
}
