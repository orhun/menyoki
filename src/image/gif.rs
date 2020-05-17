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

/* GIF and frame settings */
pub struct GifSettings {
	repeat: i32,
	speed: u32,
}

impl GifSettings {
	/**
	 * Create a new GifSettings object.
	 *
	 * @param  repeat
	 * @param  speed
	 * @return GifSettings
	 */
	pub fn new(repeat: i32, speed: u32) -> Self {
		Self { repeat, speed }
	}
}

/* GIF encoder and processing speed */
pub struct Gif {
	encoder: Encoder<File>,
	settings: GifSettings,
}

impl Gif {
	/**
	 * Create a new Gif object.
	 *
	 * @param  file
	 * @param  geometry
	 * @param  settings
	 * @return Result (Gif)
	 */
	pub fn new(
		file: File,
		geometry: Geometry,
		settings: GifSettings,
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

#[cfg(test)]
mod tests {
	use super::*;
	use crate::util::cmd::Command;
	#[test]
	fn test_gif_mod() -> Result<(), Error> {
		let geometry = Geometry::new(0, 0, 1, 2);
		let settings = GifSettings::new(-1, 10);
		let frames = vec![
			Frame::new(Image::new(vec![0, 0, 0, 255, 255, 255], geometry), 10),
			Frame::new(Image::new(vec![255, 255, 255, 0, 0, 0], geometry), 10),
		];
		let mut gif = Gif::new(File::create("test.gif")?, geometry, settings)?;
		gif.save(frames)?;
		Command::new(String::from("rm"), vec![String::from("test.gif")])
			.execute()?;
		Ok(())
	}
}
