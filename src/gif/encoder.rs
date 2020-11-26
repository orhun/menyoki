use crate::gif::settings::GifSettings;
use crate::image::geometry::Geometry;
use crate::image::Image;
use crate::util::state::InputState;
use std::io::Write;

/* GIF encoder configuration */
pub struct EncoderConfig<'a, Output: Write> {
	pub fps: u32,
	pub geometry: Geometry,
	pub output: Output,
	pub settings: &'a GifSettings,
}

impl<'a, Output: Write> EncoderConfig<'a, Output> {
	/**
	 * Create a new EncoderConfig object.
	 *
	 * @param  fps
	 * @param  geometry
	 * @param  output
	 * @param  settings
	 * @return EncoderConfig
	 */
	pub fn new(
		fps: u32,
		geometry: Geometry,
		output: Output,
		settings: &'a GifSettings,
	) -> Self {
		Self {
			fps,
			geometry,
			output,
			settings,
		}
	}
}

/* Images to encode and FPS value */
pub type Frames = (Vec<Image>, u32);

/* Required GIF encoding methods */
pub trait Encoder<'a, Output: Write> {
	fn new(config: EncoderConfig<'a, Output>) -> Self
	where
		Self: Sized;
	fn save(self, images: Vec<Image>, input_state: Option<&'static InputState>);
}

#[cfg(test)]
mod tests {
	use super::*;
	#[cfg(feature = "ski")]
	use crate::gif::ski::GifskiEncoder as GifEncoder;
	#[cfg(not(feature = "ski"))]
	use crate::gif::GifEncoder;
	use image::Bgra;
	#[test]
	fn test_gif_encoder() {
		let geometry = Geometry::new(0, 0, 1, 2);
		let data = vec![Bgra::from([0, 0, 0, 0]), Bgra::from([255, 255, 255, 0])];
		let images = vec![
			Image::new(data.clone(), false, geometry),
			Image::new(data.into_iter().rev().collect(), false, geometry),
		];
		let settings = GifSettings::default();
		let mut output = Vec::new();
		let config = EncoderConfig::new(10, geometry, &mut output, &settings);
		GifEncoder::new(config).save(images, None);
		output.truncate(6);
		assert_eq!(vec![71, 73, 70, 56, 57, 97], output);
	}
}
