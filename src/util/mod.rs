pub mod args;
pub mod cmd;
pub mod device;
pub mod parser;
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
 * Append the given information to the file name.
 *
 * @param  file_name
 * @param  info
 * @return String
 */
pub fn update_file_name(file_name: String, info: String) -> String {
	file_name
		.split('.')
		.enumerate()
		.map(|(i, s)| {
			if i == 0 {
				format!("{}_{}", s, info)
			} else {
				format!(".{}", s)
			}
		})
		.collect::<String>()
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn test_parse_args() {
		let matches = parse_args();
		assert!(matches.args.len() > 0);
		assert!(matches.usage.unwrap().lines().count() > 1);
	}
	#[test]
	fn test_update_file_name() {
		assert_eq!(
			"t_1588101718.gif",
			update_file_name(String::from("t.gif"), String::from("1588101718"))
		)
	}
	#[test]
	fn test_init_logger() -> Result<(), SetLoggerError> {
		init_logger()?;
		Ok(())
	}
}
