use clap::ArgMatches;
use std::str::FromStr;

#[derive(Debug)]
pub struct RecordSettings {
	pub fps: u32,
	pub timeout: u128,
	pub interval: u64,
	pub countdown: u64,
	pub color: u64,
}

impl RecordSettings {
	pub fn new(
		fps: u32,
		timeout: u128,
		interval: u64,
		countdown: u64,
		color: u64,
	) -> Self {
		Self {
			fps,
			timeout,
			interval,
			countdown,
			color,
		}
	}

	pub fn from_args(args: Option<&ArgMatches<'static>>) -> Self {
		match args {
			Some(matches) => {
				let settings_parser = SettingsParser::new(matches.clone());
				Self {
					fps: settings_parser.get_arg::<u32>("fps", 10),
					timeout: settings_parser.get_arg::<u128>("timeout", 30),
					interval: settings_parser.get_arg::<u64>("interval", 10),
					countdown: settings_parser.get_arg::<u64>("countdown", 3),
					color: settings_parser.get_color(),
				}
			}
			None => RecordSettings::default(),
		}
	}
}

impl Default for RecordSettings {
	fn default() -> Self {
		Self {
			fps: 10,
			timeout: 30,
			interval: 10,
			countdown: 3,
			color: 0x00ff_00ff,
		}
	}
}

struct SettingsParser {
	args: ArgMatches<'static>,
}

impl SettingsParser {
	fn new(args: ArgMatches<'static>) -> Self {
		Self { args }
	}

	/**
	 * Get an argument value from parsed arguments.
	 *
	 * @return T
	 */
	pub fn get_arg<T: FromStr>(&self, arg: &str, default: T) -> T {
		self.args
			.value_of(arg)
			.unwrap_or_default()
			.parse()
			.unwrap_or(default)
	}

	/**
	 * Get the main color from parsed arguments.
	 *
	 * @return u64
	 */
	pub fn get_color(&self) -> u64 {
		u64::from_str_radix(self.args.value_of("color").unwrap_or("FF00FF"), 16)
			.expect("Failed to parse the color HEX")
	}
}
