use crate::anim::settings::AnimSettings;
use crate::app::AppResult;
use crate::image::geometry::Geometry;
use crate::image::Image;
use crate::util::state::InputState;
use image::ExtendedColorType;
use png::{BitDepth, ColorType, Encoder, FilterType};
use std::io::{self, Write};

/* APNG encoder and settings */
pub struct ApngEncoder<'a, Output: Write> {
	encoder: Encoder<'a, Output>,
	settings: &'a AnimSettings,
}

impl<'a, Output: Write> ApngEncoder<'a, Output> {
	/**
	 * Create a new ApngEncoder object.
	 *
	 * @param  frame_count
	 * @param  geometry
	 * @param  output
	 * @param  settings
	 * @return ApngEncoder (Result)
	 */
	pub fn new(
		frame_count: u32,
		geometry: Geometry,
		output: Output,
		settings: &'a AnimSettings,
	) -> AppResult<Self> {
		let mut encoder = Encoder::new(output, geometry.width, geometry.height);
		encoder.set_animated(
			frame_count,
			settings.repeat.try_into().unwrap_or_default(),
		)?;
		encoder.set_color(ColorType::Rgba);
		encoder.set_depth(BitDepth::Eight);
		encoder.set_filter(FilterType::NoFilter);
		Ok(Self { encoder, settings })
	}

	/**
	 * Encode images as frame and write to the APNG file.
	 *
	 * @param  images
	 * @param  input_state (Option)
	 * @return Result
	 */
	pub fn save(
		self,
		images: Vec<Image>,
		input_state: Option<&'static InputState>,
	) -> AppResult<()> {
		let mut writer = self.encoder.write_header()?;
		writer.set_frame_delay(1, self.settings.fps.try_into().unwrap_or(1))?;
		for (i, image) in images.iter().enumerate() {
			let percentage = ((i + 1) as f64 / images.len() as f64) * 100.;
			info!("Saving... ({:.1}%)\r", percentage);
			debug!(
				"Encoding... ({:.1}%) [{}/{}]\r",
				percentage,
				i + 1,
				images.len()
			);
			io::stdout().flush()?;
			if let Some(state) = input_state {
				if state.check_cancel_keys() {
					info!("\n");
					warn!("User interrupt detected.");
					panic!("Failed to write the frames")
				}
			}
			writer.write_image_data(&image.get_data(ExtendedColorType::Rgba8))?;
		}
		info!("\n");
		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use image::Bgra;
	#[test]
	fn test_apng_encoder() -> AppResult<()> {
		let geometry = Geometry::new(0, 0, 1, 2);
		let data = vec![Bgra::from([128, 128, 128, 0]), Bgra::from([16, 16, 16, 0])];
		let images = vec![
			Image::new(data.clone(), false, geometry),
			Image::new(data.into_iter().rev().collect(), false, geometry),
		];
		let mut output = Vec::new();
		ApngEncoder::new(
			images.len().try_into().unwrap(),
			geometry,
			&mut output,
			&AnimSettings::default(),
		)?
		.save(images, None)?;
		output.truncate(6);
		assert_eq!(vec![0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a], output);
		Ok(())
	}
}
