pub mod cmd;
pub mod device;
pub mod settings;
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

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn test_init_logger() -> Result<(), SetLoggerError> {
		init_logger()?;
		Ok(())
	}
}
