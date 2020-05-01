use clap::{App, Arg, ArgMatches, SubCommand};

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
}
