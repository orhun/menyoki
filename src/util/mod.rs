pub mod cmd;
pub mod file;
pub mod modifier;
pub mod settings;
pub mod state;
use chrono::Local;
use fern::colors::{Color, ColoredLevelConfig};
use fern::Dispatch;
use log::{LevelFilter, SetLoggerError};

/**
 * Initialize the logger library.
 *
 * @return Result
 */
pub fn init_logger() -> Result<(), SetLoggerError> {
	let colors = ColoredLevelConfig::new()
		.info(Color::Magenta)
		.error(Color::Red)
		.warn(Color::Yellow)
		.debug(Color::Blue)
		.trace(Color::BrightBlack);
	Dispatch::new()
		.format(move |out, message, record| {
			out.finish(format_args!(
				"[{} {} {}] {}",
				Local::now().format("%Y-%m-%dT%H:%M:%S"),
				colors.color(record.level()),
				record.target(),
				message
			))
		})
		.level(LevelFilter::Debug)
		.chain(std::io::stdout())
		.apply()?;
	Ok(())
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
