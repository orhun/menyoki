use clap::ArgMatches;
use std::str::FromStr;

/* Clap single argument parser */
#[derive(Debug)]
pub struct ArgParser<'a> {
	pub args: Option<&'a ArgMatches<'a>>,
}

impl<'a> ArgParser<'a> {
	/**
	 * Create a new ArgParser object.
	 *
	 * @param  args (Option)
	 * @return ArgParser
	 */
	pub fn new(args: Option<&'a ArgMatches<'a>>) -> Self {
		Self { args }
	}

	/**
	 * Create a new ArgParser object from a name of subcommand.
	 *
	 * @param  name
	 * @return ArgParser
	 */
	pub fn from_subcommand(args: &'a ArgMatches<'a>, name: &str) -> Self {
		let mut args = args.subcommand();
		while args.0 != name {
			args = match args.1 {
				Some(subcommand) => subcommand.subcommand(),
				None => break,
			}
		}
		Self::new(args.1)
	}

	/**
	 * Parse an argument and return the parsed value.
	 *
	 * @param  arg
	 * @param  default_value
	 * @return T
	 */
	pub fn parse<T: FromStr>(&self, arg: &str, default_value: T) -> T {
		self.args
			.expect("Invalid arguments")
			.value_of(arg)
			.unwrap_or_default()
			.parse()
			.unwrap_or(default_value)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::args::Args;
	#[test]
	fn test_parser_mod() {
		let args = Args::parse();
		let parser = ArgParser::new(Some(&args));
		assert_eq!(1, parser.parse("test", 1));
		assert!(ArgParser::from_subcommand(&args, "test").args.is_none())
	}
}
