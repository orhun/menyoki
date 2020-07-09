use crate::args::parser::ArgParser;
use crate::image::padding::Padding;
use clap::ArgMatches;

/* Window to record, with selection flag  */
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RecordWindow {
	Focus(bool),
	Root(bool),
}

impl RecordWindow {
	/**
	 * Create a RecordWindow enum from parsed arguments.
	 *
	 * @param  args
	 * @return RecordWindow
	 */
	fn from_args(args: &ArgMatches<'_>) -> Self {
		let select = args.occurrences_of("select") != 0;
		if args.is_present("focus") {
			Self::Focus(select)
		} else if args.is_present("root") {
			Self::Root(select)
		} else {
			Self::Focus(true)
		}
	}
}

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
			timeout: 30,
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
	 * Create a RecordTime object from parsed arguments.
	 *
	 * @param  parser
	 * @return RecordTime
	 */
	pub fn from_args(parser: ArgParser<'_>) -> Self {
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

/* Recording and window settings */
#[derive(Clone, Copy, Debug)]
pub struct RecordSettings {
	pub fps: u32,
	pub color: u64,
	pub border: Option<u32>,
	pub alpha: bool,
	pub padding: Padding,
	pub time: RecordTime,
	pub window: RecordWindow,
}

/* Default initialization values for RecordSettings */
impl Default for RecordSettings {
	fn default() -> Self {
		Self {
			fps: 10,
			color: 0x00ff_00ff,
			border: Some(1),
			alpha: false,
			padding: Padding::default(),
			time: RecordTime::default(),
			window: RecordWindow::Focus(true),
		}
	}
}

impl RecordSettings {
	/**
	 * Create a new RecordSettings object.
	 *
	 * @param  fps
	 * @param  color
	 * @param  border (Option)
	 * @param  alpha
	 * @param  padding
	 * @param  time
	 * @param  window
	 * @return RecordSettings
	 */
	pub fn new(
		fps: u32,
		color: u64,
		border: Option<u32>,
		alpha: bool,
		padding: Padding,
		time: RecordTime,
		window: RecordWindow,
	) -> Self {
		Self {
			fps,
			color,
			border,
			alpha,
			padding,
			time,
			window,
		}
	}

	/**
	 * Create a RecordSettings object from parsed arguments.
	 *
	 * @param  parser
	 * @return RecordSettings
	 */
	pub fn from_args(parser: ArgParser<'_>) -> Self {
		match parser.args {
			Some(matches) => {
				let padding =
					Padding::parse(matches.value_of("padding").unwrap_or_default());
				Self::new(
					match parser.parse("fps", Self::default().fps) {
						fps if fps > 0 => fps,
						_ => Self::default().fps,
					},
					u64::from_str_radix(
						matches.value_of("color").unwrap_or_default(),
						16,
					)
					.unwrap_or(Self::default().color),
					if matches.is_present("no-border") {
						None
					} else if !padding.is_zero() {
						Some(1)
					} else {
						Some(parser.parse(
							"border",
							Self::default().border.unwrap_or_default(),
						))
					},
					matches.is_present("with-alpha"),
					padding,
					RecordTime::from_args(parser),
					RecordWindow::from_args(matches),
				)
			}
			None => RecordSettings::default(),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use clap::{App, Arg};
	#[test]
	fn test_record_settings() {
		let args = App::new("test")
			.arg(Arg::with_name("fps").long("fps").takes_value(true))
			.arg(Arg::with_name("color").long("color").takes_value(true))
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
			.arg(Arg::with_name("no-border").long("no-border"))
			.get_matches_from(vec![
				"test",
				"--fps",
				"15",
				"--color",
				"000000",
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
		let record_settings = RecordSettings::from_args(ArgParser::new(Some(&args)));
		assert_eq!(15, record_settings.fps);
		assert_eq!(0x0000_0000, record_settings.color);
		assert_eq!(10, record_settings.border.unwrap());
		assert!(record_settings.padding.is_zero());
		assert_eq!(2, record_settings.time.countdown);
		assert_eq!(60, record_settings.time.timeout);
		assert_eq!(12, record_settings.time.interval);
		assert_eq!(RecordWindow::Root(false), record_settings.window);
		assert!(record_settings.alpha);
	}
}
