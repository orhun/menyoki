pub mod command;
pub mod keys;
pub mod state;
use crate::file::format::FileFormat;
use chrono::{Datelike, Local, Utc, Weekday};
use colored::Color;
use fern::colors::ColoredLevelConfig;
use fern::{Dispatch, Output};
use log::{LevelFilter, SetLoggerError};

/**
 * Initialize the logger.
 *
 * @param  verbosity
 * @param  format
 * @param  color (Option)
 * @return Result
 */
pub fn init_logger(
	verbosity: u64,
	format: &FileFormat,
	color: Option<Color>,
) -> Result<(), SetLoggerError> {
	let colors = ColoredLevelConfig::new()
		.info(color.unwrap_or(Color::Magenta))
		.error(Color::Red)
		.warn(Color::Yellow)
		.debug(Color::Blue)
		.trace(Color::BrightBlack);
	let logger = Dispatch::new()
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
				out.finish(format_args!(
					"[{} {} {}] {}\n",
					time, color, target, message
				))
			}
		})
		.chain(Output::stdout(""))
		.level(match verbosity {
			0 => LevelFilter::Info,
			1 => LevelFilter::Debug,
			_ => LevelFilter::Trace,
		});
	if cfg!(test) {
		Ok(())
	} else if format == &FileFormat::Gif {
		logger
			.level_for(
				format!("{}::edit", env!("CARGO_PKG_NAME")),
				LevelFilter::Warn,
			)
			.apply()
	} else {
		logger.apply()
	}
}

/**
 * Map the given number from a range to another range.
 *
 * @param  value
 * @param  from_range
 * @param  to_range
 * @return f64
 */
pub fn map_range(value: f64, from_range: (f64, f64), to_range: (f64, f64)) -> f64 {
	to_range.0
		+ (value - from_range.0) * (to_range.1 - to_range.0)
			/ (from_range.1 - from_range.0)
}

/* Check if today is friday. */
pub fn check_friday() {
	if Utc::now().weekday() == Weekday::Fri {
		println!("(ﾉ◕ヮ◕)ﾉ*:･ﾟ✧ Thank God It's Friday! ┬──┬ ノ( ゜-゜ノ)");
	}
}
