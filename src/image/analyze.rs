use bytesize::ByteSize;
use chrono::{DateTime, Utc};
use exif::{Exif, Reader as ExifReader};
use hex::ToHex;
use image::io::Reader as ImageReader;
use image::ColorType;
use image::DynamicImage;
use std::fs::File;
use std::fs::{self, Metadata};
use std::io::BufReader;
use std::path::Path;

/* Analyzer for image files */
pub struct ImageAnalyzer {
	pub image: DynamicImage,
	pub metadata: Metadata,
	pub exif: Option<Exif>,
}

impl ImageAnalyzer {
	/**
	 * Create a new ImageAnalyzer object.
	 *
	 * @param  path
	 * @return ImageAnalyzer
	 */
	pub fn new(path: &Path) -> Self {
		Self {
			image: ImageReader::open(path)
				.expect("File not found")
				.with_guessed_format()
				.expect("File format not supported")
				.decode()
				.expect("Failed to decode the image"),
			metadata: fs::metadata(path)
				.expect("Failed to get information about the file"),
			exif: ExifReader::new()
				.read_from_container(&mut BufReader::new(
					File::open(path).expect("File not found"),
				))
				.ok(),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn test_analyze_mod() {
		let analyzer = ImageAnalyzer::new(Path::new("t.png"));
		assert_eq!(
			"2020-07-05 12:50:22.935074397 UTC",
			match analyzer.metadata.created() {
				Ok(d) => DateTime::<Utc>::from(d).to_string(),
				Err(_) => String::from("(?)"),
			}
		);
		assert_eq!(
			"2020-07-24 13:08:45.999968059 UTC",
			match analyzer.metadata.modified() {
				Ok(d) => DateTime::<Utc>::from(d).to_string(),
				Err(_) => String::from("(?)"),
			}
		);
		assert_eq!(
			"2020-09-05 09:23:46.631859511 UTC",
			match analyzer.metadata.accessed() {
				Ok(d) => DateTime::<Utc>::from(d).to_string(),
				Err(_) => String::from("(?)"),
			}
		);
		assert_eq!(false, analyzer.metadata.permissions().readonly());
		assert_eq!(
			"672.9 KB",
			ByteSize(analyzer.metadata.len()).to_string_as(false)
		);
		assert_eq!(ColorType::Rgba8, analyzer.image.color());
		let (width, height) = analyzer.image.clone().into_rgba().dimensions();
		assert_eq!("920x485", format!("{}x{}", width, height));
		let colors =
			dominant_color::get_colors(&analyzer.image.into_rgba().into_vec(), true)
				.chunks(4)
				.map(|rgba| {
					format!("#{}", rgba.encode_hex::<String>()).to_uppercase()
				})
				.collect::<Vec<String>>();
		assert_eq!(
			"#2A3D41FF-#C247B2FF-#E3CBE5FF-#60509BFF-#9D356FFF",
			colors.join("-")
		);
	}
}
