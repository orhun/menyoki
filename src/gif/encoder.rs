use crate::anim::settings::AnimSettings;
use crate::app::AppResult;
use crate::image::geometry::Geometry;
use crate::image::Image;
use crate::util::state::InputState;
use std::io::Write;

/* GIF encoder configuration */
#[derive(Clone, Copy, Debug)]
pub struct EncoderConfig<'a, Output: Write> {
	pub fps: u32,
	pub geometry: Geometry,
	pub output: Output,
	pub settings: &'a AnimSettings,
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
		settings: &'a AnimSettings,
	) -> Self {
		debug!("FPS: {}", fps);
		Self {
			fps,
			geometry,
			output,
			settings,
		}
	}
}

/* Required GIF encoding methods */
pub trait Encoder<'a, Output: Write> {
	fn new(config: EncoderConfig<'a, Output>) -> AppResult<Self>
	where
		Self: Sized;
	fn save(
		self,
		images: Vec<Image>,
		input_state: Option<&'static InputState>,
	) -> AppResult<()>;
}

#[cfg(test)]
mod tests {
	use super::*;
	#[cfg(feature = "ski")]
	use crate::gif::ski::GifskiEncoder;
	use crate::gif::GifEncoder;
	use image::Rgba;
	const GIF_HEADER: &[u8] = &[0x47, 0x49, 0x46, 0x38, 0x39, 0x61];
	fn get_config<Output: Write>(
		output: Output,
		settings: &AnimSettings,
	) -> (EncoderConfig<'_, Output>, Vec<Image>) {
		let geometry = Geometry::new(0, 0, 1, 2);
		let data = vec![Rgba::from([0, 0, 0, 0]), Rgba::from([255, 255, 255, 0])];
		let images = vec![
			Image::new(data.clone(), false, geometry),
			Image::new(data.into_iter().rev().collect(), false, geometry),
		];
		(EncoderConfig::new(10, geometry, output, settings), images)
	}
	#[test]
	fn test_gif_encoder() {
		let mut output = Vec::new();
		let settings = AnimSettings::default();
		let (config, images) = get_config(&mut output, &settings);
		GifEncoder::new(config).unwrap().save(images, None).unwrap();
		output.truncate(6);
		assert_eq!(GIF_HEADER, output);
		output.clear();
	}
	#[cfg(feature = "ski")]
	#[test]
	fn test_gifski_encoder() {
		let mut output = Vec::new();
		let settings = AnimSettings::default();
		let (config, images) = get_config(&mut output, &settings);
		GifskiEncoder::new(config)
			.unwrap()
			.save(images, None)
			.unwrap();
		output.truncate(6);
		assert_eq!(GIF_HEADER, output);
	}
}
