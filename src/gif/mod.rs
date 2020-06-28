pub mod settings;

use crate::gif::settings::GifSettings;
use crate::image::geometry::Geometry;
use crate::image::Image;
use crate::util::state::InputState;
use gif::{Encoder, Frame as GifFrame, Repeat, SetParameter};
use image::ColorType;
use std::convert::TryInto;
use std::io::Error;
use std::io::Write;

/* Frame image and delay (in units of 10 ms) */
#[derive(Clone, Debug)]
pub struct Frame {
	pub image: Image,
}

impl Frame {
	/**
	 * Create a new Frame object.
	 *
	 * @param  image
	 * @return Frame
	 */
	pub fn new(image: Image) -> Self {
		Self { image }
	}
}

/* GIF encoder and settings */
pub struct Gif<Output: Write> {
	fps: u32,
	encoder: Encoder<Output>,
	settings: GifSettings,
}

impl<Output: Write> Gif<Output> {
	/**
	 * Create a new Gif object.
	 *
	 * @param  geometry
	 * @param  output
	 * @param  fps
	 * @param  settings
	 * @return Result (Gif)
	 */
	pub fn new(
		geometry: Geometry,
		output: Output,
		fps: u32,
		settings: GifSettings,
	) -> Result<Self, Error> {
		let mut encoder = Encoder::new(
			output,
			geometry.width.try_into().unwrap_or_default(),
			geometry.height.try_into().unwrap_or_default(),
			&[],
		)?;
		encoder.set(match settings.repeat {
			n if n >= 0 => Repeat::Finite(n.try_into().unwrap_or_default()),
			_ => Repeat::Infinite,
		})?;
		Ok(Self {
			fps,
			encoder,
			settings,
		})
	}

	/**
	 * Write frames to the GIF file.
	 *
	 * @param  frames
	 * @param  input_state
	 * @return Result
	 */
	pub fn save(
		mut self,
		frames: Vec<Frame>,
		input_state: &InputState,
	) -> Result<(), Error> {
		for frame in frames {
			if input_state.check_cancel_keys() {
				warn!("User interrupt detected.");
				break;
			}
			let mut frame = GifFrame::from_rgba_speed(
				frame.image.geometry.width.try_into().unwrap_or_default(),
				frame.image.geometry.height.try_into().unwrap_or_default(),
				&mut frame.image.get_data(ColorType::Rgba8),
				(31 - self.settings.quality).try_into().unwrap_or_default(),
			);
			frame.delay = ((1. / self.fps as f32) * 1e2) as u16;
			self.encoder.write_frame(&frame)?;
		}
		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use image::Bgra;
	#[test]
	fn test_gif_mod() -> Result<(), Error> {
		let geometry = Geometry::new(0, 0, 1, 2, None);
		let settings = GifSettings::new(-1, 10);
		let data = vec![Bgra::from([0, 0, 0, 0]), Bgra::from([255, 255, 255, 0])];
		let frames = vec![
			Frame::new(Image::new(data.clone(), false, geometry), 10),
			Frame::new(
				Image::new(data.into_iter().rev().collect(), false, geometry),
				10,
			),
		];
		let mut gif = Gif::new(geometry, Vec::new(), settings)?;
		gif.save(frames, &InputState::new())?;
		Ok(())
	}
}
