pub mod decoder;
pub mod encoder;
pub mod settings;
#[cfg(feature = "ski")]
pub mod ski;

use crate::gif::encoder::Encoder;
use crate::gif::settings::GifSettings;
use crate::image::geometry::Geometry;
use crate::image::Image;
use crate::util::state::InputState;
use gif::{Encoder as GifEncoder, Frame, Repeat};
use image::ExtendedColorType;
use std::convert::TryInto;
use std::io::{self, Write};

/* GIF encoder and settings */
pub struct Gif<'a, Output: Write> {
	fps: u32,
	encoder: GifEncoder<Output>,
	settings: &'a GifSettings,
}

impl<'a, Output: Write> Encoder<'a, Output> for Gif<'a, Output> {
	/**
	 * Create a new Gif object.
	 *
	 * @param  fps
	 * @param  geometry
	 * @param  output
	 * @param  settings
	 * @return Gif
	 */
	fn new(
		fps: u32,
		geometry: Geometry,
		output: Output,
		settings: &'a GifSettings,
	) -> Self {
		let mut encoder = GifEncoder::new(
			output,
			geometry.width.try_into().unwrap_or_default(),
			geometry.height.try_into().unwrap_or_default(),
			&[],
		)
		.expect("Failed to create a GIF encoder");
		encoder
			.set_repeat(match settings.repeat {
				n if n >= 0 => Repeat::Finite(n.try_into().unwrap_or_default()),
				_ => Repeat::Infinite,
			})
			.expect("Failed to set repeat count");
		Self {
			fps,
			encoder,
			settings,
		}
	}

	/**
	 * Encode images as frame and write to the GIF file.
	 *
	 * @param images
	 * @param input_state (Option)
	 */
	fn save(mut self, images: Vec<Image>, input_state: Option<&'static InputState>) {
		let speed = 30
			- self.settings.map_range(
				self.settings.quality.into(),
				(1., 100.),
				(0., 29.),
			) as i32;
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
			let mut frame = Frame::from_rgba_speed(
				image.geometry.width.try_into().unwrap_or_default(),
				image.geometry.height.try_into().unwrap_or_default(),
				&mut image.get_data(ExtendedColorType::Rgba8),
				speed,
			);
			frame.delay = (1e2 / self.fps as f32) as u16;
			self.encoder.write_frame(&frame).unwrap_or_else(|_| {
				panic!("Failed to write frame: {}/{}", i + 1, images.len())
			});
		}
		info!("\n");
	}
}
