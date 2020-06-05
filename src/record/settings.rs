use crate::args::parser::ArgParser;
use crate::image::Padding;

/* Window to record */
#[derive(Clone, Copy, Debug)]
pub enum RecordWindow {
	Focus,
	Root,
	None,
}

/* Time related recording settings */
#[derive(Clone, Copy, Debug)]
pub struct RecordTime {
	pub timeout: u64,
	pub interval: u64,
	pub countdown: u64,
}

impl RecordTime {
	/**
	 * Create a new RecordTime object.
	 *
	 * @param  timeout
	 * @param  interval
	 * @param  countdown
	 * @return RecordTime
	 */
	pub fn new(timeout: u64, interval: u64, countdown: u64) -> Self {
		Self {
			timeout,
			interval,
			countdown,
		}
	}
}

/* Default initialization values for RecordTime */
impl Default for RecordTime {
	fn default() -> Self {
		Self {
			timeout: 30,
			interval: 10,
			countdown: 3,
		}
	}
}

/* Recording and window settings */
#[derive(Clone, Copy, Debug)]
pub struct RecordSettings {
	pub fps: u32,
	pub color: u64,
	pub border: u32,
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
			border: 5,
			padding: Padding::default(),
			time: RecordTime::default(),
			window: RecordWindow::None,
		}
	}
}

impl RecordSettings {
	/**
	 * Create a new RecordSettings object.
	 *
	 * @param  fps
	 * @param  color
	 * @param  border
	 * @param  padding
	 * @param  time
	 * @param  window
	 * @return RecordSettings
	 */
	pub fn new(
		fps: u32,
		color: u64,
		border: u32,
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
			Some(matches) => Self::new(
				parser.parse("fps", Self::default().fps),
				u64::from_str_radix(
					matches.value_of("color").unwrap_or_default(),
					16,
				)
				.unwrap_or(Self::default().color),
				parser.parse("border", Self::default().border),
				Padding::parse(matches.value_of("padding").unwrap_or_default()),
				RecordTime::new(
					parser.parse("timeout", Self::default().time.timeout),
					parser.parse("interval", Self::default().time.interval),
					parser.parse("countdown", Self::default().time.countdown),
				),
				if matches.is_present("focus") {
					RecordWindow::Focus
				} else if matches.is_present("root") {
					RecordWindow::Root
				} else {
					RecordWindow::None
				},
			),
			None => RecordSettings::default(),
		}
	}
}
