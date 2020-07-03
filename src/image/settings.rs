use crate::args::parser::ArgParser;
use image::png::CompressionType;
use image::png::FilterType;

/* PNG compression and filter settings */
#[derive(Clone, Copy, Debug)]
pub struct PngSettings {
	pub compression: CompressionType,
	pub filter: FilterType,
}

/* Default initialization values for PngSettings */
impl Default for PngSettings {
	fn default() -> Self {
		Self {
			compression: CompressionType::Fast,
			filter: FilterType::Sub,
		}
	}
}

impl PngSettings {
	/**
	 * Create a new PngSettings object.
	 *
	 * @param  compression
	 * @param  filter
	 * @return PngSettings
	 */
	pub fn new(compression: CompressionType, filter: FilterType) -> Self {
		Self {
			compression,
			filter,
		}
	}

	/**
	 * Create a PngSettings object from parsed arguments.
	 *
	 * @param  parser
	 * @return PngSettings
	 */
	pub fn from_args(parser: ArgParser<'_>) -> Self {
		match parser.args {
			Some(matches) => Self::new(
				match matches.value_of("compression") {
					Some("default") => CompressionType::Default,
					Some("fast") => CompressionType::Fast,
					Some("best") => CompressionType::Best,
					Some("huffman") => CompressionType::Huffman,
					Some("rle") => CompressionType::Rle,
					_ => CompressionType::Fast,
				},
				match matches.value_of("filter") {
					Some("none") => FilterType::NoFilter,
					Some("sub") => FilterType::Sub,
					Some("up") => FilterType::Up,
					Some("avg") => FilterType::Avg,
					Some("paeth") => FilterType::Paeth,
					_ => FilterType::Sub,
				},
			),
			None => Self::default(),
		}
	}
}

/* JPG quality setting */
#[derive(Clone, Copy, Debug)]
pub struct JpgSettings {
	pub quality: u8,
}

/* Default initialization values for JpgSettings */
impl Default for JpgSettings {
	fn default() -> Self {
		Self { quality: 90 }
	}
}

impl JpgSettings {
	/**
	 * Create a new JpgSettings object.
	 *
	 * @param  quality
	 * @return JpgSettings
	 */
	pub fn new(quality: u8) -> Self {
		if quality <= 25 {
			warn!("Image will be encoded in low quality.");
		}
		Self { quality }
	}

	/**
	 * Create a JpgSettings object from parsed arguments.
	 *
	 * @param  parser
	 * @return JpgSettings
	 */
	pub fn from_args(parser: ArgParser<'_>) -> Self {
		match parser.args {
			Some(_) => Self::new(parser.parse("quality", Self::default().quality)),
			None => Self::default(),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use clap::{App, Arg};
	#[test]
	fn test_png_settings() {
		for value in vec![
			("default", "none"),
			("best", "up"),
			("huffman", "avg"),
			("rle", "paeth"),
			("", ""),
		] {
			let args = App::new("test")
				.arg(
					Arg::with_name("compression")
						.long("compression")
						.takes_value(true),
				)
				.arg(Arg::with_name("filter").long("filter").takes_value(true))
				.get_matches_from(vec![
					"test",
					"--compression",
					value.0,
					"--filter",
					value.1,
				]);
			let parser = ArgParser::new(Some(&args));
			let png_settings = PngSettings::from_args(parser);
			if value.0.is_empty() && value.1.is_empty() {
				assert_eq!(
					PngSettings::default().compression,
					png_settings.compression
				);
				assert_eq!(PngSettings::default().filter, png_settings.filter);
			} else {
				assert_ne!(
					PngSettings::default().compression,
					png_settings.compression
				);
				assert_ne!(PngSettings::default().filter, png_settings.filter);
			}
		}
	}
	#[test]
	fn test_jpg_settings() {
		let args = App::new("test")
			.arg(Arg::with_name("quality").long("quality").takes_value(true))
			.get_matches_from(vec!["test", "--quality", "50"]);
		assert_eq!(
			50,
			JpgSettings::from_args(ArgParser::new(Some(&args))).quality
		);
		assert_eq!(90, JpgSettings::from_args(ArgParser::new(None)).quality);
	}
}
