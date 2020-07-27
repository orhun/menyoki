use crate::gif::settings::GifSettings;
use crate::image::geometry::Geometry;
use crate::image::Image;
use crate::util::state::InputState;
use std::io::{Error, Write};

/* Images to encode and FPS value */
pub type Frames = (Vec<Image>, u32);

/* Required encoding methods for Gif */
pub trait Encoder<Output: Write> {
	fn new(
		fps: u32,
		geometry: Geometry,
		output: Output,
		settings: GifSettings,
	) -> Result<Self, Error>
	where
		Self: Sized;
	fn save(
		self,
		images: Vec<Image>,
		input_state: &'static InputState,
	) -> Result<(), Error>;
}

#[cfg(test)]
mod tests {
	use super::*;
	#[cfg(feature = "ski")]
	use crate::gif::ski::Gif as GifEncoder;
	#[cfg(not(feature = "ski"))]
	use crate::gif::Gif as GifEncoder;
	use image::Bgra;
	#[test]
	fn test_gif_encoder() -> Result<(), Error> {
		let geometry = Geometry::new(0, 0, 1, 2);
		let settings = GifSettings::new(-1, 10, 1., false);
		let data = vec![Bgra::from([0, 0, 0, 0]), Bgra::from([255, 255, 255, 0])];
		let images = vec![
			Image::new(data.clone(), false, geometry),
			Image::new(data.into_iter().rev().collect(), false, geometry),
		];
		let gif = GifEncoder::new(10, geometry, Vec::new(), settings)?;
		gif.save(images, Box::leak(Box::new(InputState::new())))?;
		Ok(())
	}
}
