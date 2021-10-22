use crate::anim::settings::AnimSettings;
use crate::anim::Frames;
use crate::app::{AppError, AppResult};
use crate::edit::ImageOps;
use image::Frame;
use std::io::{self, Write};

/* Animation decoder and settings */
pub struct AnimDecoder<'a> {
	imageops: ImageOps<'a>,
	settings: &'a AnimSettings,
}

impl<'a> AnimDecoder<'a> {
	/**
	 * Create a new AnimDecoder object.
	 *
	 * @param  imageops
	 * @param  settings
	 * @return AnimDecoder
	 */
	pub fn new(imageops: ImageOps<'a>, settings: &'a AnimSettings) -> Self {
		Self { imageops, settings }
	}

	/**
	 * Update frames to cut the duration.
	 *
	 * @param  frames
	 * @param  cut
	 * @param  fps
	 * @return Vector of Frame
	 */
	fn cut_duration(
		frames: &mut Vec<Frame>,
		cut: (f32, f32),
		fps: u32,
	) -> Vec<Frame> {
		if cut != (0., 0.) {
			let (start, end) = cut;
			let frame_delay = 1000_u32.checked_div(fps).unwrap_or_default() as f32;
			frames
				.drain(
					((start / frame_delay) as u32)
						.try_into()
						.unwrap_or_default()
						..frames.len().saturating_sub((end / frame_delay) as usize),
				)
				.collect()
		} else {
			frames.to_vec()
		}
	}

	/**
	 * Update and return the frames.
	 *
	 * @param  frames
	 * @return Frames (Result)
	 */
	pub fn update_frames(mut self, mut frames: Vec<Frame>) -> AppResult<Frames> {
		let first_frame = frames.first().ok_or_else(|| {
			AppError::FrameError(String::from("No frames found to process"))
		})?;
		self.imageops
			.init(first_frame.clone().into_buffer().dimensions());
		let fps = ((1e3 / first_frame.delay().numer_denom_ms().0 as f32)
			* self.settings.speed) as u32;
		debug!("FPS: {:?}", fps);
		let frames = Self::cut_duration(&mut frames, self.settings.cut, fps);
		let mut images = Vec::new();
		for (i, frame) in frames.iter().enumerate() {
			let percentage = ((i + 1) as f64 / frames.len() as f64) * 100.;
			info!("Processing the frames... ({:.1}%)\r", percentage);
			debug!(
				"Processing the frames... ({:.1}%) [{}/{}]\r",
				percentage,
				i + 1,
				frames.len()
			);
			io::stdout().flush()?;
			images.push(
				self.imageops
					.process(frame.clone().into_buffer())
					.get_image(),
			);
		}
		info!("\n");
		Ok((images, fps))
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::edit::settings::EditSettings;
	use crate::image::geometry::Geometry;
	use image::{Delay, Frame, RgbaImage};
	use pretty_assertions::assert_eq;
	#[test]
	fn test_anim_decoder() {
		let mut anim_settings = AnimSettings::default();
		anim_settings.cut = (500., 0.);
		anim_settings.speed = 2.0;
		let mut edit_settings = EditSettings::default();
		edit_settings.image.ratio = 2.0;
		let frames = AnimDecoder::new(edit_settings.get_imageops(), &anim_settings)
			.update_frames(vec![
				Frame::from_parts(
					RgbaImage::new(1, 1),
					0,
					0,
					Delay::from_numer_denom_ms(1000, 1),
				),
				Frame::from_parts(
					RgbaImage::new(1, 1),
					0,
					0,
					Delay::from_numer_denom_ms(10, 1),
				),
			])
			.unwrap();
		assert_eq!(2, frames.1);
		assert_eq!(1, frames.0.len());
		assert_eq!(Geometry::new(0, 0, 2, 2), frames.0[0].geometry);
	}
}
