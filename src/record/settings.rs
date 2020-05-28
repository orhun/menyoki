use crate::settings::SettingsParser;
use clap::ArgMatches;

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

	pub fn from_args(args: Option<&ArgMatches<'static>>, color: u64) -> Self {
		match args {
			Some(matches) => {
				let settings_parser = SettingsParser::new(matches.clone());
				Self::new(
					settings_parser.get_arg::<u32>("fps", 10),
					settings_parser.get_arg::<u128>("timeout", 30),
					settings_parser.get_arg::<u64>("interval", 10),
					settings_parser.get_arg::<u64>("countdown", 3),
					color,
				)
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
