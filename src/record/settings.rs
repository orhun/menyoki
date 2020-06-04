use crate::args::parser::ArgParser;

/* Window to record */
#[derive(Clone, Copy, Debug)]
pub enum RecordWindow {
	Focus,
	Root,
	None,
}

/* Recording and window settings */
#[derive(Clone, Copy, Debug)]
pub struct RecordSettings {
	pub fps: u32,
	pub border: u32,
	pub timeout: u64,
	pub interval: u64,
	pub countdown: u64,
	pub color: u64,
	pub window: RecordWindow,
}

/* Default initialization values for RecordSettings */
impl Default for RecordSettings {
	fn default() -> Self {
		Self {
			fps: 10,
			border: 5,
			timeout: 30,
			interval: 10,
			countdown: 3,
			color: 0x00ff_00ff,
			window: RecordWindow::None,
		}
	}
}

impl RecordSettings {
	/**
	 * Create a new RecordSettings object.
	 *
	 * @param  fps
	 * @param  border
	 * @param  timeout
	 * @param  interval
	 * @param  countdown
	 * @param  color
	 * @param  window
	 * @return RecordSettings
	 */
	pub fn new(
		fps: u32,
		border: u32,
		timeout: u64,
		interval: u64,
		countdown: u64,
		color: u64,
		window: RecordWindow,
	) -> Self {
		Self {
			fps,
			border,
			timeout,
			interval,
			countdown,
			color,
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
				parser.parse("border", Self::default().border),
				parser.parse("timeout", Self::default().timeout),
				parser.parse("interval", Self::default().interval),
				parser.parse("countdown", Self::default().countdown),
				u64::from_str_radix(
					matches.value_of("color").unwrap_or_default(),
					16,
				)
				.unwrap_or(Self::default().color),
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
