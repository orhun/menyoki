use crate::analyze::ImageAnalyzer;
use crate::app::AppResult;
use crate::args::matches::ArgMatches;
use crate::args::parser::ArgParser;
use chrono::{DateTime, Local, Utc};
use colored::Color;
use std::path::PathBuf;
use std::time::SystemTime;

/* Time zone with the timestamp flag */
#[derive(Debug)]
pub enum TimeZone {
	Local(bool),
	Utc(bool),
}

impl TimeZone {
	/**
	 * Get the system time as String.
	 *
	 * @param  system_time
	 * @return time
	 */
	pub fn get(&self, system_time: SystemTime) -> String {
		match self {
			Self::Local(false) => DateTime::<Local>::from(system_time).to_string(),
			Self::Local(true) => {
				DateTime::<Local>::from(system_time).timestamp().to_string()
			}
			Self::Utc(false) => DateTime::<Utc>::from(system_time).to_string(),
			Self::Utc(true) => {
				DateTime::<Utc>::from(system_time).timestamp().to_string()
			}
		}
	}

	/**
	 * Get the current date.
	 *
	 * @return time
	 */
	pub fn now(&self) -> String {
		match self {
			Self::Local(false) => Local::now().to_string(),
			Self::Local(true) => Local::now().timestamp().to_string(),
			Self::Utc(false) => Utc::now().to_string(),
			Self::Utc(true) => Utc::now().timestamp().to_string(),
		}
	}
}

/* Image analysis settings */
#[derive(Debug)]
pub struct AnalyzeSettings {
	pub file: PathBuf,
	pub color: Color,
	pub time: TimeZone,
}

/* Default initialization values for AnalyzeSettings */
impl Default for AnalyzeSettings {
	fn default() -> Self {
		Self {
			file: PathBuf::new(),
			color: Color::White,
			time: TimeZone::Utc(false),
		}
	}
}

impl AnalyzeSettings {
	/**
	 * Create a new AnalyzeSettings object.
	 *
	 * @param  file
	 * @param  color
	 * @param  time
	 * @return AnalyzeSettings
	 */
	pub fn new(file: PathBuf, color: Color, time: TimeZone) -> Self {
		Self { file, color, time }
	}

	/**
	 * Create a new AnalyzeSettings object from arguments.
	 *
	 * @param  matches
	 * @param  color (Option)
	 * @return AnalyzeSettings
	 */
	pub fn from_args(matches: &ArgMatches<'_>, color: Option<Color>) -> Self {
		Self::from_parser(ArgParser::from_subcommand(matches, "analyze"), color)
	}

	/**
	 * Create an AnalyzeSettings object from an argument parser.
	 *
	 * @param  parser
	 * @param  color (Option)
	 * @return AnalyzeSettings
	 */
	fn from_parser(parser: ArgParser<'_>, color: Option<Color>) -> Self {
		match parser.args {
			Some(matches) => {
				let timestamp = matches.is_present("timestamp");
				Self::new(
					PathBuf::from(matches.value_of("file").unwrap_or_default()),
					color.unwrap_or(Self::default().color),
					match matches.value_of("time-zone") {
						Some("local") => TimeZone::Local(timestamp),
						_ => TimeZone::Utc(timestamp),
					},
				)
			}
			None => Self::default(),
		}
	}

	/**
	 * Get ImageAnalyzer object from AnalyzeSettings.
	 *
	 * @return ImageAnalyzer (Result)
	 */
	pub fn get_analyzer(&self) -> AppResult<ImageAnalyzer<'_>> {
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
			AnalyzeSettings::from_parser(ArgParser::from_args(&args), None);
		assert_eq!(Some("test.png"), analyze_settings.file.to_str());
		let analyze_settings = AnalyzeSettings::default();
		assert_eq!(Some(""), analyze_settings.file.to_str());
		assert_eq!(Color::White, analyze_settings.color);
	}
	#[test]
	fn test_time_zone() {
		let system_time = SystemTime::now();
		let utc_time = TimeZone::Utc(false).get(system_time);
		let local_time = TimeZone::Local(false).get(system_time);
		assert!(utc_time.contains("UTC"));
		assert_eq!(
			Utc::now().format("%F").to_string(),
			utc_time.split_whitespace().collect::<Vec<&str>>()[0]
		);
		assert_eq!(
			Local::now().format("%F").to_string(),
			local_time.split_whitespace().collect::<Vec<&str>>()[0]
		);
		assert!(
			Utc::now().timestamp() - 1
				< TimeZone::Utc(true)
					.get(system_time)
					.parse()
					.unwrap_or_default()
		);
		assert!(
			Local::now().timestamp() - 1
				< TimeZone::Local(true)
					.get(system_time)
					.parse()
					.unwrap_or_default()
		);
	}
}
