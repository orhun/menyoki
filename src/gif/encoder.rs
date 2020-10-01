use crate::gif::settings::GifSettings;
use crate::image::geometry::Geometry;
use crate::image::Image;
use crate::util::state::InputState;
use std::io::Write;

/* Images to encode and FPS value */
pub type Frames = (Vec<Image>, u32);

/* Required encoding methods for Gif */
pub trait Encoder<'a, Output: Write> {
	fn new(
		fps: u32,
		geometry: Geometry,
		output: Output,
		settings: &'a GifSettings,
	) -> Self
	where
		Self: Sized;
	fn save(self, images: Vec<Image>, input_state: Option<&'static InputState>);
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
	fn test_gif_encoder() {
		let geometry = Geometry::new(0, 0, 1, 2);
		let data = vec![Bgra::from([0, 0, 0, 0]), Bgra::from([255, 255, 255, 0])];
		let images = vec![
			Image::new(data.clone(), false, geometry),
			Image::new(data.into_iter().rev().collect(), false, geometry),
		];
		let settings = GifSettings::default();
		let mut output = Vec::new();
		GifEncoder::new(10, geometry, &mut output, &settings).save(images, None);
		output.truncate(6);
		assert_eq!(vec![71, 73, 70, 56, 57, 97], output);
	}
}
