pub mod settings;

use crate::analyze::settings::AnalyzeSettings;
use bytesize::ByteSize;
use colored::{Color, Colorize};
use exif::{Exif, Reader as ExifReader};
use hex::ToHex;
use image::io::Reader as ImageReader;
use image::{DynamicImage, ImageFormat};
use std::fs::{self, File, Metadata};
use std::io::BufReader;

/* Time information of a file */
pub enum TimeInfo {
	Created,
	Modified,
	Accessed,
}

/* Analyzer for image files */
pub struct ImageAnalyzer<'a> {
	format: Option<ImageFormat>,
	image: DynamicImage,
	metadata: Metadata,
	exif: Option<Exif>,
	settings: &'a AnalyzeSettings,
}

impl<'a> ImageAnalyzer<'a> {
	/**
	 * Create a new ImageAnalyzer object.
	 *
	 * @param  settings
	 * @return ImageAnalyzer
	 */
	pub fn new(settings: &'a AnalyzeSettings) -> Self {
		debug!("{:?}", settings);
		let reader = ImageReader::open(&settings.file)
			.expect("File not found")
			.with_guessed_format()
			.expect("File format not supported");
		Self {
			format: reader.format(),
			image: reader.decode().expect("Failed to decode the image"),
			metadata: fs::metadata(&settings.file)
				.expect("Failed to get information about the file"),
			exif: ExifReader::new()
				.read_from_container(&mut BufReader::new(
					File::open(&settings.file).expect("File not found"),
				))
				.ok(),
			settings,
		}
	}

	/**
	 * Get the time information of the file.
	 *
	 * @param  info
	 * @return date
	 */
	fn get_time_info(&self, info: TimeInfo) -> String {
		if let Ok(d) = match info {
			TimeInfo::Created => self.metadata.created(),
			TimeInfo::Modified => self.metadata.modified(),
			TimeInfo::Accessed => self.metadata.accessed(),
		} {
			self.settings.time.get(d)
		} else {
			String::from("(?)")
		}
	}

	/**
	 * Get the size of the file.
	 *
	 * @return String
	 */
	fn get_file_size(&self) -> String {
		ByteSize(self.metadata.len()).to_string_as(false)
	}

	/**
	 * Get the formatted width and height of the image.
	 *
	 * @return dimensions
	 */
	fn get_image_dimensions(&self) -> String {
		let (width, height) = self.image.clone().into_rgba8().dimensions();
		format!("{}x{}", width, height)
	}

	/**
	 * Get dominant colors of the image.
	 *
	 * @return Vector of String
	 */
	fn get_dominant_colors(&self) -> Vec<String> {
		dominant_color::get_colors(&self.image.clone().into_rgba8().into_vec(), true)
			.chunks(4)
			.map(|rgba| format!("#{}", rgba.encode_hex::<String>()).to_uppercase())
			.collect()
	}

	/**
	 * Get EXIF data from the image.
	 *
	 * @return data
	 */
	fn get_exif_data(&self) -> String {
		let mut data = String::new();
		if let Some(exif) = &self.exif {
			data += "\nEXIF Data\n";
			for f in exif.fields() {
				let mut value = f.display_value().with_unit(exif).to_string();
				if value.len() > 64
					&& (f.tag.to_string() == "MakerNote"
						|| f.tag.to_string() == "UserComment")
				{
					value = format!("({} bytes binary data)", value.len());
				}
				data += &format!(
					"  {}: {}{}\n",
					f.tag,
					value,
					if f.ifd_num.index() == 1 { " (T)" } else { "" }
				);
			}
		}
		data
	}

	/**
	 * Get the analysis report.
	 *
	 * @return report
	 */
	pub fn get_report(self) -> String {
		format!(
			"{} - image analysis report\n\n\
			File Information\
			\n  File:     {:?} ({}){}\
			\n  Created:  {}\
			\n  Modified: {}\
			\n  Accessed: {}\
			\n\nImage Information\
			\n  Format:     {}\
			\n  Dimensions: {}px\
			\n  Color Type: {}\
			\n  Main Colors:\
			\n   \u{2022} {}\
			\n{}\n\
			generated on {}\
			",
			env!("CARGO_PKG_NAME"),
			self.settings.file,
			self.get_file_size(),
			if self.metadata.permissions().readonly() {
				" [readonly]"
			} else {
				""
			},
			self.get_time_info(TimeInfo::Created),
			self.get_time_info(TimeInfo::Modified),
			self.get_time_info(TimeInfo::Accessed),
			self.format.map_or_else(
				|| String::from("(?)"),
				|f| format!("{:?}", f).to_uppercase()
			),
			self.get_image_dimensions(),
			format!("{:?}", self.image.color()).to_uppercase(),
			self.get_dominant_colors().join("\n   \u{2022} "),
			self.get_exif_data(),
			self.settings.time.now(),
		)
	}

	/**
	 * Colorize the report by using the predefined format.
	 *
	 * @param  color
	 * @param  report
	 * @return colored_report
	 */
	fn colorize_report(color: Color, report: String) -> String {
		let mut colored_report = String::new();
		for line in report.lines() {
			colored_report += &if !(line.starts_with("  ") || line.contains('-')) {
				line.white().bold().to_string()
			} else if line.starts_with("  ") && line.contains(':') {
				let mut values = line.split(':');
				format!(
					"{}:{}",
					values.next().unwrap_or_default().color(color),
					values.collect::<String>()
				)
			} else if line.starts_with("  ") && line.contains("\u{2022}") {
				match hex::decode(
					line.split('#')
						.collect::<Vec<&str>>()
						.get(1)
						.cloned()
						.unwrap_or_default(),
				) {
					Ok(rgb) => line.truecolor(rgb[0], rgb[1], rgb[2]).to_string(),
					Err(_) => line.to_string(),
				}
			} else {
				line.to_string()
			};
			colored_report += "\n";
		}
		colored_report
	}

	/**
	 * Get the colored analysis report.
	 *
	 * @return report
	 */
	pub fn get_colored_report(self) -> String {
		Self::colorize_report(self.settings.color, self.get_report())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::analyze::settings::TimeZone;
	use chrono::Utc;
	use colored::Color;
	use image::{ColorType, ImageBuffer, Rgba};
	use pretty_assertions::assert_eq;
	use std::path::PathBuf;
	#[test]
	fn test_analyze() {
		let file_name = "test2.png";
		ImageBuffer::<Rgba<u8>, Vec<u8>>::from_vec(
			1,
			2,
			vec![255, 255, 255, 255, 0, 0, 0, 255],
		)
		.unwrap()
		.save(file_name)
		.unwrap();
		let settings = AnalyzeSettings::new(
			PathBuf::from(file_name),
			Color::White,
			TimeZone::Utc(false),
		);
		let analyzer = ImageAnalyzer::new(&settings);
		assert_eq!("73 B", analyzer.get_file_size());
		assert_eq!(
			Utc::now().format("%F").to_string(),
			analyzer
				.get_time_info(TimeInfo::Created)
				.split_whitespace()
				.collect::<Vec<&str>>()[0]
		);
		assert_eq!(
			Utc::now().format("%F").to_string(),
			analyzer
				.get_time_info(TimeInfo::Modified)
				.split_whitespace()
				.collect::<Vec<&str>>()[0]
		);
		assert_eq!(
			Utc::now().format("%F").to_string(),
			analyzer
				.get_time_info(TimeInfo::Accessed)
				.split_whitespace()
				.collect::<Vec<&str>>()[0]
		);
		assert_eq!(false, analyzer.metadata.permissions().readonly());
		assert_eq!(Some(ImageFormat::Png), analyzer.format);
		assert_eq!(ColorType::Rgba8, analyzer.image.color());
		assert_eq!("1x2", analyzer.get_image_dimensions());
		assert_eq!(
			"#000000FF-#FFFFFFFF",
			analyzer.get_dominant_colors().join("-")
		);
		assert!(analyzer.exif.is_none());
		assert_eq!(
			17,
			analyzer
				.get_colored_report()
				.lines()
				.collect::<Vec<&str>>()
				.len()
		);
		fs::remove_file(file_name).unwrap();
	}
}
