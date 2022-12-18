use crate::settings::AppSettings;
use chrono::Local;
use colored::Color;
use fern_colored::colors::ColoredLevelConfig;
use fern_colored::{Dispatch, Output};
use log::{LevelFilter, SetLoggerError};

/* Logger with settings */
#[derive(Clone, Copy)]
pub struct Logger<'a> {
	settings: &'a AppSettings<'a>,
	colors: ColoredLevelConfig,
	level_filter: LevelFilter,
}

impl<'a> Logger<'a> {
	/**
	 * Create a new Logger object.
	 *
	 * @param  settings
	 * @return Logger
	 */
	pub fn new(settings: &'a AppSettings<'a>) -> Self {
		Self {
			settings,
			colors: ColoredLevelConfig::new()
				.info(settings.get_main_color().unwrap_or(Color::Green))
				.error(Color::Red)
				.warn(Color::Yellow)
				.debug(Color::Blue)
				.trace(Color::BrightBlack),
			level_filter: if settings.args.is_present("quiet")
				|| settings.save.file.path.to_str() == Some("-")
			{
				LevelFilter::Off
			} else {
				match settings.args.occurrences_of("verbose") {
					0 => LevelFilter::Info,
					1 => LevelFilter::Debug,
					_ => LevelFilter::Trace,
				}
			},
		}
	}

	/**
	 * Initialize the logger.
	 *
	 * @return Result
	 */
	pub fn init(&self) -> Result<(), SetLoggerError> {
		let colors = self.colors;
		let mut logger = Dispatch::new()
			.format(move |out, message, record| {
				let time = Local::now().format("%FT%T");
				let color = colors.color(record.level());
				let target = record.target();
				let message = message.to_string();
				if message == "\n" {
					out.finish(format_args!("\n"))
				} else if message == "\r" {
					out.finish(format_args!("\r"))
				} else if message.ends_with('#') {
					out.finish(format_args!("{}", &message[..message.len() - 1]))
				} else if message.ends_with('\r') {
					out.finish(format_args!(
						"\r[{} {} {}] {}",
						time,
						color,
						target,
						&message[..message.len() - 1]
					))
				} else {
					out.finish(format_args!("[{time} {color} {target}] {message}\n"))
				}
			})
			.chain(Output::stdout(""))
			.level(self.level_filter);
		if self.settings.save.file.format.is_animation() {
			logger = logger.level_for(
				format!("{}::edit", env!("CARGO_PKG_NAME")),
				LevelFilter::Warn,
			)
		}
		logger.apply()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::args::matches::ArgMatches;
	use clap::ArgMatches as Args;
	use std::env;
	#[test]
	fn test_logger() -> Result<(), SetLoggerError> {
		let quiet_var =
			format!("{}_GENERAL_QUIET", env!("CARGO_PKG_NAME").to_uppercase());
		env::set_var(quiet_var, "true");
		let args = Args::default();
		let matches = ArgMatches::new(&args);
		let settings = AppSettings::new(&matches);
		assert!(matches.is_present("quiet"));
		Logger::new(&settings).init()
	}
}
