pub mod settings;

use crate::app::AppResult;
use crate::view::settings::ViewSettings;
use image::DynamicImage;
use viuer::Config;

/* Viewer for image files */
pub struct ImageViewer {
	image: DynamicImage,
	config: Config,
}

impl ImageViewer {
	/**
	 * Create a new ImageViewer object.
	 *
	 * @param  image
	 * @param  settings
	 * @return ImageViewer
	 */
	pub fn new(image: DynamicImage, settings: &ViewSettings) -> Self {
		debug!("{:?}", settings);
		Self {
			image,
			config: Config {
				transparent: settings.transparent,
				absolute_offset: false,
				..Config::default()
			},
		}
	}

	/**
	 * View the image by printing to the terminal.
	 *
	 * @return Result
	 */
	pub fn view(&self) -> AppResult<(u32, u32)> {
		Ok(viuer::print(&self.image, &self.config)?)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use image::{ImageBuffer, Rgba};
	use pretty_assertions::assert_eq;
	#[test]
	fn test_view() {
		let buffer = ImageBuffer::<Rgba<u8>, Vec<u8>>::from_vec(
			1,
			2,
			vec![58, 164, 49, 255, 0, 0, 0, 255],
		)
		.unwrap();
		let image = DynamicImage::ImageRgba8(buffer);
		let mut viewer = ImageViewer::new(image, &ViewSettings::default());
		viewer.config.restore_cursor = true;
		assert_eq!((1, 1), viewer.view().unwrap());
	}
}
