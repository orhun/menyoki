use crate::anim::settings::AnimSettings;
use crate::image::geometry::Geometry;
use crate::image::Image;
use crate::util::state::InputState;
use apng::{Config, Encoder, Frame, PNGImage};
use image::ExtendedColorType;
use png::{BitDepth, ColorType, FilterType};
use std::convert::TryInto;
use std::io::{self, Write};

/* APNG encoder and settings */
#[derive(Debug)]
pub struct ApngEncoder<'a> {
	geometry: Geometry,
	config: Config,
	settings: &'a AnimSettings,
}

impl<'a> ApngEncoder<'a> {
	/**
	 * Create a new ApngEncoder object.
	 *
	 * @param  frame_count
	 * @param  geometry
	 * @param  settings
	 * @return ApngEncoder
	 */
	pub fn new(
		frame_count: u32,
		geometry: Geometry,
		settings: &'a AnimSettings,
	) -> Self {
		Self {
			geometry,
			config: Config {
				width: geometry.width,
				height: geometry.height,
				num_frames: frame_count,
				num_plays: settings.repeat.try_into().unwrap_or_default(),
				color: ColorType::RGBA,
				depth: BitDepth::Eight,
				filter: FilterType::NoFilter,
			},
			settings,
		}
	}

	/**
	 * Encode images as frame and write to the APNG file.
	 *
	 * @param images
	 * @param input_state (Option)
	 */
	pub fn save<Output: Write>(
		&self,
		images: Vec<Image>,
		input_state: Option<&'static InputState>,
		mut output: Output,
	) {
		let mut encoder = Encoder::new(&mut output, self.config.clone())
			.expect("Failed to create APNG encoder");
		for (i, image) in images.iter().enumerate() {
			let percentage = ((i + 1) as f64 / images.len() as f64) * 100.;
			info!("Saving... ({:.1}%)\r", percentage);
			debug!(
				"Encoding... ({:.1}%) [{}/{}]\r",
				percentage,
				i + 1,
				images.len()
			);
			io::stdout().flush().expect("Failed to flush stdout");
			if let Some(state) = input_state {
				if state.check_cancel_keys() {
					info!("\n");
					warn!("User interrupt detected.");
					panic!("Failed to write the frames")
				}
			}
			encoder
				.write_frame(
					&PNGImage {
						width: self.geometry.width,
						height: self.geometry.height,
						data: image.get_data(ExtendedColorType::Rgba8),
						color_type: self.config.color,
						bit_depth: self.config.depth,
					},
					Frame {
						delay_num: Some(1),
						delay_den: Some(self.settings.fps.try_into().unwrap_or(1)),
						..Default::default()
					},
				)
				.unwrap_or_else(|_| {
					panic!("Failed to write frame: {}/{}", i + 1, images.len())
				});
		}
		info!("\n");
		encoder.finish_encode().expect("Failed to finish encoding");
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use image::Bgra;
	#[test]
	fn test_apng_encoder() {
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
			&AnimSettings::default(),
		)
		.save(images, None, &mut output);
		output.truncate(6);
		assert_eq!(vec![0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a], output);
	}
}
