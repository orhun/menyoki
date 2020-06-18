pub mod settings;

use crate::encode::jpg::settings::JpgSettings;
use crate::image::Image;
use image::error::ImageError;
use image::jpeg::JPEGEncoder;
use image::ColorType;
use std::io::Write;

/* JPG image and encoder */
pub struct Jpg<'a, W: Write> {
	image: Image,
	encoder: JPEGEncoder<'a, W>,
}

impl<'a, W: Write> Jpg<'a, W> {
	/**
	 * Create a new Jpg object.
	 *
	 * @param  image
	 * @param  output
	 * @param  settings
	 * @return Jpg
	 */
	pub fn new(image: Image, output: &'a mut W, settings: JpgSettings) -> Self {
		Self {
			image,
			encoder: JPEGEncoder::new_with_quality(output, settings.quality),
		}
	}

	/**
	 * Encode the image and write to JPG file.
	 *
	 * @return Result
	 */
	pub fn encode(mut self) -> Result<(), ImageError> {
		self.encoder.encode(
			&self.image.data,
			self.image.geometry.width,
			self.image.geometry.height,
			ColorType::Rgb8,
		)?;
		Ok(())
	}
}
