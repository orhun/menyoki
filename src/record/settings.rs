use crate::args::parser::ArgParser;
use crate::image::padding::Padding;
use clap::ArgMatches;

/* Window to record */
#[derive(Clone, Copy, Debug)]
pub enum RecordWindow {
	Focus,
	Root,
	Select,
}

impl RecordWindow {
	/**
	 * Create a RecordWindow enum from parsed arguments.
	 *
	 * @param  args
	 * @return RecordWindow
	 */
	fn from_args(args: &ArgMatches<'_>) -> Self {
		if args.is_present("focus") {
			Self::Focus
		} else if args.is_present("root") {
			Self::Root
		} else {
			Self::Select
		}
	}
}

/* Time related recording settings */
#[derive(Clone, Copy, Debug)]
pub struct RecordTime {
	pub countdown: u64,
	pub timeout: u64,
	pub interval: u64,
}

/* Default initialization values for RecordTime */
impl Default for RecordTime {
	fn default() -> Self {
		Self {
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
	 * @param  countdown
	 * @param  timeout
	 * @param  interval
	 * @return RecordTime
	 */
	pub fn new(countdown: u64, timeout: u64, interval: u64) -> Self {
		Self {
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
			border: Some(5),
			padding: Padding::default(),
			time: RecordTime::default(),
			window: RecordWindow::Select,
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
	 * @param  padding
	 * @param  time
	 * @param  window
	 * @return RecordSettings
	 */
	pub fn new(
		fps: u32,
		color: u64,
		border: Option<u32>,
		padding: Padding,
		time: RecordTime,
		window: RecordWindow,
	) -> Self {
		Self {
			fps,
			color,
			border,
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
					} else {
						Some(parser.parse(
							"border",
							Self::default().border.unwrap_or_default(),
						))
					},
					Padding::parse(matches.value_of("padding").unwrap_or_default()),
					RecordTime::from_args(parser),
					RecordWindow::from_args(matches),
				)
			}
			None => RecordSettings::default(),
		}
	}
}
