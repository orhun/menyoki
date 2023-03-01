use crate::args::matches::ArgMatches;
use crate::args::parser::ArgParser;
use image::codecs::png::{CompressionType, FilterType};
use image::codecs::pnm::{PnmSubtype, SampleEncoding};
use image::codecs::webp::WebPQuality;

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
	 * Create a new PngSettings object from arguments.
	 *
	 * @param  matches
	 * @return PngSettings
	 */
	pub fn from_args(matches: &ArgMatches<'_>) -> Self {
		Self::from_parser(ArgParser::from_subcommand(matches, "png"))
	}

	/**
	 * Create a PngSettings object from an argument parser.
	 *
	 * @param  parser
	 * @return PngSettings
	 */
	fn from_parser(parser: ArgParser<'_>) -> Self {
		match parser.args {
			Some(matches) => Self::new(
				match matches.value_of("compression") {
					Some("default") => CompressionType::Default,
					Some("fast") => CompressionType::Fast,
					Some("best") => CompressionType::Best,
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
		Self { quality }
	}

	/**
	 * Create a new JpgSettings object from arguments.
	 *
	 * @param  matches
	 * @return JpgSettings
	 */
	pub fn from_args(matches: &ArgMatches<'_>) -> Self {
		Self::from_parser(ArgParser::from_subcommand(matches, "jpg"))
	}

	/**
	 * Create a JpgSettings object from an argument parser.
	 *
	 * @param  parser
	 * @return JpgSettings
	 */
	fn from_parser(parser: ArgParser<'_>) -> Self {
		match parser.args {
			Some(_) => Self::new(parser.parse("quality", Self::default().quality)),
			None => Self::default(),
		}
	}
}

/* WebP quality setting */
#[derive(Clone, Copy, Debug)]
pub struct WebPSettings {
	quality: Option<u8>,
}

/* Default initialization values for WebPSettings */
impl Default for WebPSettings {
	fn default() -> Self {
		Self {
			quality: Some(WebPQuality::DEFAULT),
		}
	}
}

impl WebPSettings {
	/**
	 * Create a new WebPSettings object.
	 *
	 * @param  quality
	 * @return WebPSettings
	 */
	pub fn new(quality: Option<u8>) -> Self {
		Self { quality }
	}

	/**
	 * Create a WebPQuality object from WebPSettings.
	 *
	 * @return WebPQuality
	 */
	pub fn get_quality(&self) -> WebPQuality {
		match self.quality {
			Some(quality) => WebPQuality::lossy(quality),
			None => WebPQuality::lossless(),
		}
	}

	/**
	 * Create a new WebPSettings object from arguments.
	 *
	 * @param  matches
	 * @return WebPSettings
	 */
	pub fn from_args(matches: &ArgMatches<'_>) -> Self {
		Self::from_parser(ArgParser::from_subcommand(matches, "webp"))
	}

	/**
	 * Create a WebPSettings object from an argument parser.
	 *
	 * @param  parser
	 * @return WebPSettings
	 */
	fn from_parser(parser: ArgParser<'_>) -> Self {
		match &parser.args {
			Some(args) => {
				if args.is_present("lossless") {
					Self::new(None)
				} else {
					Self::new(Some(parser.parse("quality", WebPQuality::DEFAULT)))
				}
			}
			None => Self::default(),
		}
	}
}

/* PNM subtype settings */
#[derive(Clone, Copy, Debug)]
pub struct PnmSettings {
	pub subtype: PnmSubtype,
}

/* Default initialization values for PnmSettings */
impl Default for PnmSettings {
	fn default() -> Self {
		Self {
			subtype: PnmSubtype::Pixmap(SampleEncoding::Binary),
		}
	}
}

impl PnmSettings {
	/**
	 * Create a new PnmSettings object.
	 *
	 * @param  subtype
	 * @return PnmSettings
	 */
	pub fn new(subtype: PnmSubtype) -> Self {
		Self { subtype }
	}

	/**
	 * Create a new PnmSettings object from arguments.
	 *
	 * @param  matches
	 * @return PnmSettings
	 */
	pub fn from_args(matches: &ArgMatches<'_>) -> Self {
		Self::from_parser(ArgParser::from_subcommand(matches, "pnm"))
	}

	/**
	 * Create a PnmSettings object from an argument parser.
	 *
	 * @param  parser
	 * @return PnmSettings
	 */
	fn from_parser(parser: ArgParser<'_>) -> Self {
		match parser.args {
			Some(matches) => {
				let encoding = match matches.value_of("encoding") {
					Some("ascii") => SampleEncoding::Ascii,
					_ => SampleEncoding::Binary,
				};
				Self::new(match matches.value_of("format") {
					Some("bitmap") => PnmSubtype::Bitmap(encoding),
					Some("graymap") => PnmSubtype::Graymap(encoding),
					Some("arbitrary") => PnmSubtype::ArbitraryMap,
					_ => PnmSubtype::Pixmap(encoding),
				})
			}
			None => Self::default(),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use clap::{App, Arg};
	use pretty_assertions::{assert_eq, assert_ne};
	#[test]
	fn test_png_settings() {
		for value in &[
			("default", "none"),
			("best", "up"),
			("default", "avg"),
			("best", "paeth"),
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
			let png_settings = PngSettings::from_parser(ArgParser::from_args(&args));
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
			JpgSettings::from_parser(ArgParser::from_args(&args)).quality
		);
		assert_eq!(90, JpgSettings::from_parser(ArgParser::new(None)).quality);
	}
	#[test]
	fn test_webp_settings() {
		let args = App::new("test")
			.arg(Arg::with_name("quality").long("quality").takes_value(true))
			.get_matches_from(vec!["test", "--quality", "20"]);
		assert_eq!(
			Some(20),
			WebPSettings::from_parser(ArgParser::from_args(&args)).quality
		);
		assert_eq!(
			Some(80),
			WebPSettings::from_parser(ArgParser::new(None)).quality
		);
		let args = App::new("test")
			.arg(Arg::with_name("lossless").long("lossless"))
			.get_matches_from(vec!["test", "--lossless"]);
		assert_eq!(
			None,
			WebPSettings::from_parser(ArgParser::from_args(&args)).quality
		);
	}
	#[test]
	fn test_pnm_settings() {
		let args = App::new("test")
			.arg(Arg::with_name("format").long("format").takes_value(true))
			.arg(
				Arg::with_name("encoding")
					.long("encoding")
					.takes_value(true),
			)
			.get_matches_from(vec![
				"test",
				"--format",
				"graymap",
				"--encoding",
				"ascii",
			]);
		assert_eq!(
			PnmSubtype::Graymap(SampleEncoding::Ascii),
			PnmSettings::from_parser(ArgParser::from_args(&args)).subtype
		);
		assert_eq!(
			PnmSubtype::Pixmap(SampleEncoding::Binary),
			PnmSettings::from_parser(ArgParser::new(None)).subtype
		)
	}
}
