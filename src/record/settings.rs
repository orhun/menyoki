use crate::args::matches::ArgMatches;
use crate::args::parser::ArgParser;
use crate::image::geometry::Geometry;
use crate::image::padding::Padding;
use crate::util::cmd::Command;

/* Time related recording settings */
#[derive(Clone, Copy, Debug)]
pub struct RecordTime {
	pub duration: Option<f64>,
	pub countdown: u64,
	pub timeout: u64,
	pub interval: u64,
}

/* Default initialization values for RecordTime */
impl Default for RecordTime {
	fn default() -> Self {
		Self {
			duration: None,
			countdown: 3,
			timeout: 60,
			interval: 10,
		}
	}
}

impl RecordTime {
	/**
	 * Create a new RecordTime object.
	 *
	 * @param  duration (Option)
	 * @param  countdown
	 * @param  timeout
	 * @param  interval
	 * @return RecordTime
	 */
	pub fn new(
		duration: Option<f64>,
		countdown: u64,
		timeout: u64,
		interval: u64,
	) -> Self {
		Self {
			duration,
			countdown,
			timeout,
			interval,
		}
	}

	/**
	 * Create a RecordTime object from an argument parser.
	 *
	 * @param  parser
	 * @return RecordTime
	 */
	fn from_parser(parser: &ArgParser<'_>) -> Self {
		RecordTime::new(
			match parser.parse("duration", 0.0) {
				duration if duration > 0.0 => Some(duration),
				_ => Self::default().duration,
			},
			parser.parse("countdown", Self::default().countdown),
			parser.parse("timeout", Self::default().timeout),
			parser.parse("interval", Self::default().interval),
		)
	}
}

/* Flag values of recording */
#[derive(Clone, Copy, Debug)]
pub struct RecordFlag {
	pub alpha: bool,
	pub keys: Option<&'static str>,
}

/* Default initialization values for RecordFlag */
impl Default for RecordFlag {
	fn default() -> Self {
		Self {
			alpha: false,
			keys: Some(""),
		}
	}
}

impl RecordFlag {
	/**
	 * Create a new RecordFlag object.
	 *
	 * @param  alpha
	 * @param  keys (Option)
	 * @return RecordFlag
	 */
	pub fn new(alpha: bool, keys: Option<&'static str>) -> Self {
		Self { alpha, keys }
	}
}

/* Window to record, with geometric properties  */
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RecordWindow {
	Focus(Option<Geometry>),
	Root(Option<Geometry>),
}

impl RecordWindow {
	/**
	 * Create a RecordWindow enum from parsed arguments.
	 *
	 * @param  matches
	 * @return RecordWindow
	 */
	fn from_args(matches: &ArgMatches<'_>) -> Self {
		let select =
			if matches.occurrences_of("size") != 0 || matches.is_present("select") {
				Some(Geometry::parse(
					matches.value_of("size").unwrap_or_default(),
				))
			} else {
				None
			};
		if matches.is_present("focus") {
			Self::Focus(select)
		} else if matches.is_present("root") {
			Self::Root(select)
		} else {
			Self::Focus(Some(select.unwrap_or_default()))
		}
	}
}

/* Recording and window settings */
#[derive(Clone, Copy, Debug)]
pub struct RecordSettings {
	pub command: Option<&'static str>,
	pub color: u64,
	pub border: Option<u32>,
	pub padding: Padding,
	pub time: RecordTime,
	pub flag: RecordFlag,
	pub window: RecordWindow,
}

/* Default initialization values for RecordSettings */
impl Default for RecordSettings {
	fn default() -> Self {
		Self {
			command: None,
			color: 0x00ff_00ff,
			border: Some(1),
			padding: Padding::default(),
			time: RecordTime::default(),
			flag: RecordFlag::default(),
			window: RecordWindow::Focus(Some(Geometry::default())),
		}
	}
}

impl RecordSettings {
	/**
	 * Create a new RecordSettings object.
	 *
	 * @param  command (Option)
	 * @param  color
	 * @param  border (Option)
	 * @param  padding
	 * @param  time
	 * @param  flag
	 * @param  window
	 * @return RecordSettings
	 */
	pub fn new(
		command: Option<&'static str>,
		color: u64,
		border: Option<u32>,
		padding: Padding,
		time: RecordTime,
		flag: RecordFlag,
		window: RecordWindow,
	) -> Self {
		Self {
			command,
			color,
			border,
			padding,
			time,
			flag,
			window,
		}
	}

	/**
	 * Create a new RecordSettings object from arguments.
	 *
	 * @param  matches
	 * @return RecordSettings
	 */
	pub fn from_args(matches: &ArgMatches<'_>) -> Self {
		Self::from_parser(
			ArgParser::from_subcommand(
				matches,
				if matches.is_present("capture") {
					"capture"
				} else {
					"record"
				},
			),
			matches.value_of("color").unwrap_or_default(),
		)
	}

	/**
	 * Create a RecordSettings object from an argument parser.
	 *
	 * @param  parser
	 * @param  color
	 * @return RecordSettings
	 */
	fn from_parser(parser: ArgParser<'_>, color: &str) -> Self {
		match parser.args {
			Some(ref matches) => {
				let padding =
					Padding::parse(matches.value_of("padding").unwrap_or_default());
				Self::new(
					match matches.value_of("command") {
						Some(cmd) => {
							Some(Box::leak(cmd.to_string().into_boxed_str()))
						}
						_ => None,
					},
					u64::from_str_radix(color, 16).unwrap_or(Self::default().color),
					if matches.is_present("no-borders") {
						None
					} else if !padding.is_zero() {
						Some(1)
					} else {
						Some(parser.parse(
							"border",
							Self::default().border.unwrap_or_default(),
						))
					},
					padding,
					RecordTime::from_parser(&parser),
					RecordFlag::new(
						matches.is_present("with-alpha"),
						if matches.is_present("no-keys") {
							None
						} else {
							Some(Box::leak(
								matches
									.value_of("keys")
									.unwrap_or_default()
									.to_string()
									.into_boxed_str(),
							))
						},
					),
					RecordWindow::from_args(&matches),
				)
			}
			None => RecordSettings::default(),
		}
	}

	/**
	 * Get Command from parsed settings.
	 *
	 * @return Command (Option)
	 */
	pub fn get_command<'a>(&self) -> Option<Command<'a>> {
		match self.command {
			Some(v) => Some(Command::from(v)),
			_ => None,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use clap::{App, Arg};
	use pretty_assertions::assert_eq;
	#[test]
	fn test_record_settings() {
		let args = App::new("test")
			.arg(Arg::with_name("keys").long("keys").takes_value(true))
			.arg(Arg::with_name("border").long("border").takes_value(true))
			.arg(Arg::with_name("padding").long("padding").takes_value(true))
			.arg(
				Arg::with_name("countdown")
					.long("countdown")
					.takes_value(true),
			)
			.arg(Arg::with_name("timeout").long("timeout").takes_value(true))
			.arg(
				Arg::with_name("interval")
					.long("interval")
					.takes_value(true),
			)
			.arg(Arg::with_name("root").long("root"))
			.arg(Arg::with_name("focus").long("focus"))
			.arg(Arg::with_name("with-alpha").long("with-alpha"))
			.arg(Arg::with_name("no-borders").long("no-borders"))
			.arg(Arg::with_name("no-keys").long("no-keys"))
			.get_matches_from(vec![
				"test",
				"--keys",
				"LControl-Q/S",
				"--border",
				"10",
				"--padding",
				"0:0:0:0",
				"--countdown",
				"2",
				"--timeout",
				"60",
				"--interval",
				"12",
				"--root",
				"--with-alpha",
			]);
		let record_settings =
			RecordSettings::from_parser(ArgParser::from_args(&args), "000000");
		assert_eq!(0x0000_0000, record_settings.color);
		assert_eq!(10, record_settings.border.unwrap());
		assert!(record_settings.padding.is_zero());
		assert_eq!(2, record_settings.time.countdown);
		assert_eq!(60, record_settings.time.timeout);
		assert_eq!(12, record_settings.time.interval);
		assert_eq!(RecordWindow::Root(None), record_settings.window);
		assert!(record_settings.flag.alpha);
		assert_eq!("LControl-Q/S", record_settings.flag.keys.unwrap());
	}
}
