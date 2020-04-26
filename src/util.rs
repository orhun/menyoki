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
				),
		)
		.get_matches()
}

pub fn exec_cmd(cmd: &str, cmd_args: &[&str]) -> Result<(), Error> {
	match Command::new(cmd).args(cmd_args).spawn() {
		Ok(mut child) => {
			child.wait()?;
			Ok(())
		}
		Err(e) => Err(e),
	}
}
