use crate::util::parser::ArgParser;
use clap::ArgMatches;

/* Recording and window settings */
#[derive(Clone, Copy, Debug)]
pub struct RecordSettings {
	pub fps: u32,
	pub padding: u32,
	pub timeout: u64,
	pub interval: u64,
	pub countdown: u64,
	pub color: u64,
	pub record_root: bool,
}

/* Default initialization values for RecordSettings */
impl Default for RecordSettings {
	fn default() -> Self {
		Self {
			fps: 10,
			padding: 5,
			timeout: 30,
			interval: 10,
			countdown: 3,
			color: 0x00ff_00ff,
			record_root: false,
		}
	}
}

impl RecordSettings {
	/**
	 * Create a new RecordSettings object.
	 *
	 * @param  fps
	 * @param  padding
	 * @param  timeout
	 * @param  interval
	 * @param  countdown
	 * @param  color
	 * @param  record_root
	 * @return RecordSettings
	 */
	pub fn new(
		fps: u32,
		padding: u32,
		timeout: u64,
		interval: u64,
		countdown: u64,
		color: u64,
		record_root: bool,
	) -> Self {
		Self {
			fps,
			padding,
			timeout,
			interval,
			countdown,
			color,
			record_root,
		}
	}

	/**
	 * Create a RecordSettings object from parsed arguments.
	 *
	 * @param  args
	 * @param  color
	 * @return RecordSettings
	 */
	pub fn from_args(args: Option<&ArgMatches<'static>>, color: u64) -> Self {
		match args {
			Some(matches) => {
				let parser = ArgParser::new(&matches);
				Self::new(
					parser.parse("fps", Self::default().fps),
					parser.parse("padding", Self::default().padding),
					parser.parse("timeout", Self::default().timeout),
					parser.parse("interval", Self::default().interval),
					parser.parse("countdown", Self::default().countdown),
					color,
					matches.is_present("root"),
				)
			}
			None => {
				let mut settings = RecordSettings::default();
				settings.color = color;
				settings
			}
		}
	}
}
