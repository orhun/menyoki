use crate::args::matches::ArgMatches;
use crate::args::parser::ArgParser;
use shellexpand;
use std::path::PathBuf;

/* Image view settings */
#[derive(Debug)]
pub struct ViewSettings {
	pub file: PathBuf,
	pub transparent: bool,
}

/* Default initialization values for ViewSettings */
impl Default for ViewSettings {
	fn default() -> Self {
		Self {
			file: PathBuf::new(),
			transparent: false,
		}
	}
}

impl ViewSettings {
	/**
	 * Create a new ViewSettings object.
	 *
	 * @param  file
	 * @param  transparent
	 * @return ViewSettings
	 */
	pub fn new(file: PathBuf, transparent: bool) -> Self {
		Self { file, transparent }
	}

	/**
	 * Create a new ViewSettings object from arguments.
	 *
	 * @param  matches
	 * @return ViewSettings
	 */
	pub fn from_args(matches: &ArgMatches<'_>) -> Self {
		Self::from_parser(ArgParser::from_subcommand(matches, "view"))
	}

	/**
	 * Create an ViewSettings object from an argument parser.
	 *
	 * @param  parser
	 * @return ViewSettings
	 */
	fn from_parser(parser: ArgParser<'_>) -> Self {
		match parser.args {
			Some(matches) => {
				let file = matches.value_of("file").unwrap_or_default();
				let file = shellexpand::full(file)
					.map(|s| s.to_string())
					.unwrap_or(file.to_string());
				Self::new(PathBuf::from(file), matches.is_present("transparent"))
			}
			None => Self::default(),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use clap::{App, Arg};
	use pretty_assertions::assert_eq;
	#[test]
	fn test_view_settings() {
		let args = App::new("test")
			.arg(Arg::with_name("file").required(true))
			.arg(Arg::with_name("transparent").long("transparent"))
			.get_matches_from(vec!["test", "test.png", "--transparent"]);
		let view_settings = ViewSettings::from_parser(ArgParser::from_args(&args));
		assert_eq!(Some("test.png"), view_settings.file.to_str());
		assert!(view_settings.transparent);
		let view_settings = ViewSettings::default();
		assert_eq!(Some(""), view_settings.file.to_str());
		assert!(!view_settings.transparent);
	}
}
