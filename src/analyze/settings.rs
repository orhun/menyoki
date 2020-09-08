use crate::analyze::ImageAnalyzer;
use crate::args::parser::ArgParser;
use std::path::PathBuf;

/* Image analysis settings */
#[derive(Debug)]
pub struct AnalyzeSettings {
	pub file: PathBuf,
}

/* Default initialization values for AnalyzeSettings */
impl Default for AnalyzeSettings {
	fn default() -> Self {
		Self {
			file: PathBuf::new(),
		}
	}
}

impl AnalyzeSettings {
	/**
	 * Create a new AnalyzeSettings object.
	 *
	 * @param  file
	 * @return AnalyzeSettings
	 */
	pub fn new(file: PathBuf) -> Self {
		Self { file }
	}

	/**
	 * Create a AnalyzeSettings object from parsed arguments.
	 *
	 * @param  parser
	 * @return AnalyzeSettings
	 */
	pub fn from_args(parser: ArgParser<'_>) -> Self {
		match parser.args {
			Some(matches) => Self::new(PathBuf::from(
				matches.value_of("file").unwrap_or_default(),
			)),
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
	#[test]
	fn test_analyze_settings() {
		let args = App::new("test")
			.arg(Arg::with_name("file").required(true))
			.get_matches_from(vec!["test", "test.png"]);
		assert_eq!(
			Some("test.png"),
			AnalyzeSettings::from_args(ArgParser::new(Some(&args)))
				.file
				.to_str()
		);
		assert_eq!(Some(""), AnalyzeSettings::default().file.to_str());
	}
}
