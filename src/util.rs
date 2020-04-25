use clap::{App, Arg, ArgMatches};
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
			Arg::with_name("output")
				.short("o")
				.long("output")
				.value_name("FILE")
				.default_value("t.gif")
				.help("Set the output file")
				.takes_value(true),
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
