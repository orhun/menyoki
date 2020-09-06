use bytesize::ByteSize;
use chrono::{DateTime, Utc};
use exif::{Exif, Reader as ExifReader};
use hex::ToHex;
use image::io::Reader as ImageReader;
use image::{ColorType, DynamicImage, ImageBuffer, Rgba};
use std::fs::{self, File, Metadata};
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
		let file_name = "test.png";
		ImageBuffer::<Rgba<u8>, Vec<u8>>::from_vec(
			1,
			2,
			vec![255, 255, 255, 255, 0, 0, 0, 255],
		)
		.unwrap()
		.save(file_name)
		.unwrap();
		let analyzer = ImageAnalyzer::new(Path::new(file_name));
		assert_eq!(
			Utc::now().format("%F").to_string(),
			match analyzer.metadata.created() {
				Ok(d) => DateTime::<Utc>::from(d).to_string(),
				Err(_) => String::from("(?)"),
			}
			.split_whitespace()
			.collect::<Vec<&str>>()[0]
		);
		assert_eq!(
			Utc::now().format("%F").to_string(),
			match analyzer.metadata.modified() {
				Ok(d) => DateTime::<Utc>::from(d).to_string(),
				Err(_) => String::from("(?)"),
			}
			.split_whitespace()
			.collect::<Vec<&str>>()[0]
		);
		assert_eq!(
			Utc::now().format("%F").to_string(),
			match analyzer.metadata.accessed() {
				Ok(d) => DateTime::<Utc>::from(d).to_string(),
				Err(_) => String::from("(?)"),
			}
			.split_whitespace()
			.collect::<Vec<&str>>()[0]
		);
		assert_eq!(false, analyzer.metadata.permissions().readonly());
		assert_eq!(
			"73 B",
			ByteSize(analyzer.metadata.len()).to_string_as(false)
		);
		assert_eq!(ColorType::Rgba8, analyzer.image.color());
		let (width, height) = analyzer.image.clone().into_rgba().dimensions();
		assert_eq!("1x2", format!("{}x{}", width, height));
		let colors =
			dominant_color::get_colors(&analyzer.image.into_rgba().into_vec(), true)
				.chunks(4)
				.map(|rgba| {
					format!("#{}", rgba.encode_hex::<String>()).to_uppercase()
				})
				.collect::<Vec<String>>();
		assert_eq!("#000000FF-#FFFFFFFF", colors.join("-"));
		assert!(analyzer.exif.is_none());
		fs::remove_file(file_name).unwrap();
	}
}
