use clap::{App, Arg, ArgMatches, SubCommand};
use std::io::Error;
use std::process::Command;

/**
 * Parse command line arguments.
 *
 * @return ArgMatches
 */
pub fn parse_args() -> ArgMatches<'static> {
	App::new(env!("CARGO_PKG_NAME"))
		.version(env!("CARGO_PKG_VERSION"))
		.author(env!("CARGO_PKG_AUTHORS"))
		.about(env!("CARGO_PKG_DESCRIPTION"))
		.arg(
			Arg::with_name("command")
				.value_name("COMMAND")
				.help("Sets the command to run")
				.required(true),
		)
		.arg(
			Arg::with_name("fps")
				.short("f")
				.long("fps")
				.value_name("FPS")
				.default_value("10")
				.help("Sets the FPS (frames per second) value")
				.takes_value(true),
		)
		.subcommand(
			SubCommand::with_name("gif")
				.about("Changes the GIF encoder settings")
				.arg(
					Arg::with_name("speed")
						.short("s")
						.long("speed")
						.value_name("SPEED")
						.default_value("10")
						.help("Sets the frame encoding speed (1-30)")
						.takes_value(true),
				)
				.arg(
					Arg::with_name("repeat")
						.short("r")
						.long("repeat")
						.value_name("REPEAT")
						.default_value("\u{221E}")
						.help("Sets the number of repetitions")
						.takes_value(true),
				),
		)
		.subcommand(
			SubCommand::with_name("save")
				.about("Changes the output file settings")
				.arg(
					Arg::with_name("output")
						.value_name("FILE")
						.default_value("t.gif")
						.help("Sets the output file"),
				)
				.arg(
					Arg::with_name("date")
						.short("d")
						.long("date")
						.help("Adds date and time to the file name"),
				)
				.arg(
					Arg::with_name("timestamp")
						.short("t")
						.long("timestamp")
						.help("Adds timestamp to the file name"),
				)
				.arg(
					Arg::with_name("prompt")
						.short("p")
						.long("prompt")
						.help("Shows prompt for the file name input"),
				),
		)
		.get_matches()
}

/**
 * Execute command and wait for it to exit.
 *
 * @param  cmd
 * @param  cmd_args
 * @return Result
 */
pub fn exec_cmd(cmd: &str, cmd_args: &[&str]) -> Result<(), Error> {
	match Command::new(cmd).args(cmd_args).spawn() {
		Ok(mut child) => {
			child.wait()?;
			Ok(())
		}
		Err(e) => Err(e),
	}
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
	use std::time::{Duration, Instant};
	#[test]
	fn test_parse_args() {
		let matches = parse_args();
		assert!(matches.args.len() > 0);
		assert!(matches.usage.unwrap().lines().count() > 1);
	}
	#[test]
	fn test_exec_cmd() -> Result<(), Error> {
		let sleep_time = Duration::from_millis(10);
		let now = Instant::now();
		exec_cmd("sleep", &["0.01"])?;
		assert!(now.elapsed() >= sleep_time);
		Ok(())
	}
}