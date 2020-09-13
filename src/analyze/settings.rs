use crate::analyze::ImageAnalyzer;
use crate::args::parser::ArgParser;
use chrono::{DateTime, Local, Utc};
use colored::Color;
use std::path::PathBuf;
use std::time::SystemTime;

/* Time zone */
#[derive(Debug)]
pub enum TimeZone {
	Local,
	Utc,
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
			Self::Local => DateTime::<Local>::from(system_time).to_string(),
			Self::Utc => DateTime::<Utc>::from(system_time).to_string(),
		}
	}

	/**
	 * Get the current date.
	 *
	 * @return time
	 */
	pub fn now(&self) -> String {
		match self {
			Self::Local => Local::now().to_string(),
			Self::Utc => Utc::now().to_string(),
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
			time: TimeZone::Utc,
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
				match matches.value_of("time-zone") {
					Some("local") => TimeZone::Local,
					_ => TimeZone::Utc,
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
	#[test]
	fn test_time_zone() {
		let system_time = SystemTime::now();
		let utc_time = TimeZone::Utc.get(system_time);
		let local_time = TimeZone::Local.get(system_time);
		assert!(utc_time.contains("UTC"));
		assert_eq!(
			Utc::now().format("%F").to_string(),
			utc_time.split_whitespace().collect::<Vec<&str>>()[0]
		);
		assert_eq!(
			Local::now().format("%F").to_string(),
			local_time.split_whitespace().collect::<Vec<&str>>()[0]
		);
	}
}
