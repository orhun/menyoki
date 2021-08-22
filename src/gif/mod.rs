pub mod encoder;
#[cfg(feature = "ski")]
pub mod ski;

use crate::anim::settings::AnimSettings;
use crate::app::AppResult;
use crate::gif::encoder::{Encoder, EncoderConfig};
use crate::image::Image;
use crate::util::state::InputState;
use gif::{Encoder as BaseEncoder, Frame, Repeat};
use image::ExtendedColorType;
use std::convert::TryInto;
use std::io::{self, Write};

/* GIF encoder and settings */
pub struct GifEncoder<'a, Output: Write> {
	fps: u32,
	encoder: BaseEncoder<Output>,
	settings: &'a AnimSettings,
}

impl<'a, Output: Write> Encoder<'a, Output> for GifEncoder<'a, Output> {
	/**
	 * Create a new GifEncoder object.
	 *
	 * @param  config
	 * @return GifEncoder (Result)
	 */
	fn new(config: EncoderConfig<'a, Output>) -> AppResult<Self> {
		let mut encoder = BaseEncoder::new(
			config.output,
			config.geometry.width.try_into().unwrap_or_default(),
			config.geometry.height.try_into().unwrap_or_default(),
			&[],
		)?;
		encoder.set_repeat(match config.settings.repeat {
			n if n >= 0 => Repeat::Finite(n.try_into().unwrap_or_default()),
			_ => Repeat::Infinite,
		})?;
		Ok(Self {
			fps: config.fps,
			encoder,
			settings: config.settings,
		})
	}

	/**
	 * Encode images as frame and write to the GIF file.
	 *
	 * @param  images
	 * @param  input_state (Option)
	 * @param  Result
	 */
	fn save(
		mut self,
		images: Vec<Image>,
		input_state: Option<&'static InputState>,
	) -> AppResult<()> {
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
			io::stdout().flush()?;
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
			self.encoder.write_frame(&frame)?;
		}
		info!("\n");
		Ok(())
	}
}
