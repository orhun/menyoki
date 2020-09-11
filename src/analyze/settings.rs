use crate::analyze::ImageAnalyzer;
use crate::args::parser::ArgParser;
use colored::Color;
use std::path::PathBuf;

/* Image analysis settings */
#[derive(Debug)]
pub struct AnalyzeSettings {
	pub file: PathBuf,
	pub color: Color,
}

/* Default initialization values for AnalyzeSettings */
impl Default for AnalyzeSettings {
	fn default() -> Self {
		Self {
			file: PathBuf::new(),
			color: Color::White,
		}
	}
}

impl AnalyzeSettings {
	/**
	 * Create a new AnalyzeSettings object.
	 *
	 * @param  file
	 * @param  color
	 * @return AnalyzeSettings
	 */
	pub fn new(file: PathBuf, color: Color) -> Self {
		Self { file, color }
	}

	/**
	 * Create a AnalyzeSettings object from parsed arguments.
	 *
	 * @param  parser
	 * @return AnalyzeSettings
	 */
	pub fn from_args(parser: ArgParser<'_>, color: &str) -> Self {
		match parser.args {
			Some(matches) => Self::new(
				PathBuf::from(matches.value_of("file").unwrap_or_default()),
				match hex::decode(color) {
					Ok(rgb) => Color::TrueColor {
						r: rgb[0],
						g: rgb[1],
						b: rgb[2],
					},
					Err(_) => Self::default().color,
				},
			),
			None => Self::default(),
		}
	}

	/**
	 * Get ImageAnalyzer object from AnalyzeSettings.
	 *
	 * @return ImageAnalyzer
	 */
	pub fn get_analyzer(&self) -> ImageAnalyzer<'_> {
		ImageAnalyzer::new(self)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use clap::{App, Arg};
	use pretty_assertions::assert_eq;
	#[test]
	fn test_analyze_settings() {
		let args = App::new("test")
			.arg(Arg::with_name("file").required(true))
			.get_matches_from(vec!["test", "test.png"]);
		let analyze_settings =
			AnalyzeSettings::from_args(ArgParser::new(Some(&args)), "F16823");
		assert_eq!(Some("test.png"), analyze_settings.file.to_str());
		assert_eq!(
			Color::TrueColor {
				r: 241,
				g: 104,
				b: 35
			},
			analyze_settings.color
		);
		let analyze_settings = AnalyzeSettings::default();
		assert_eq!(Some(""), analyze_settings.file.to_str());
		assert_eq!(Color::White, analyze_settings.color);
	}
}
