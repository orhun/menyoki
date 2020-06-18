pub mod settings;

use crate::image::Image;
use crate::png::settings::PngSettings;
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
	 * @param  output
	 * @param  settings
	 * @return Png
	 */
	pub fn new(image: Image, output: W, settings: PngSettings) -> Self {
		Self {
			image,
			encoder: PNGEncoder::new_with_quality(
				output,
				settings.compression,
				settings.filter,
			),
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

#[cfg(test)]
mod tests {
	use super::*;
	use crate::image::geometry::Geometry;
	use crate::util::cmd::Command;
	use std::fs::File;
	#[test]
	fn test_png_mod() -> Result<(), ImageError> {
		let geometry = Geometry::new(0, 0, 1, 2, None);
		let image = Image::new(vec![0, 0, 0, 255, 255, 255], geometry);
		let settings = PngSettings::default();
		let png = Png::new(image, File::create("test.png")?, settings);
		png.encode()?;
		Command::new(String::from("rm"), vec![String::from("test.png")])
			.execute()?;
		Ok(())
	}
}
