use crate::image::Image;
use image::error::ImageError;
use image::png::PNGEncoder;
use image::ColorType;
use std::io::Write;

/* PNG image and encoder */
pub struct Png<W: Write> {
	image: Image,
	encoder: PNGEncoder<W>,
}

impl<W: Write> Png<W> {
	/**
	 * Create a new Png object.
	 *
	 * @param  image
	 * @param  file
	 * @return Png
	 */
	pub fn new(image: Image, file: W) -> Self {
		Self {
			image,
			encoder: PNGEncoder::new(file),
		}
	}

	/**
	 * Encode the image and write to PNG file.
	 *
	 * @return Result
	 */
	pub fn encode(self) -> Result<(), ImageError> {
		self.encoder.encode(
			&self.image.data,
			self.image.geometry.width,
			self.image.geometry.height,
			ColorType::Rgb8,
		)?;
		Ok(())
	}
}
