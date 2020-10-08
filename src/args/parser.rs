use crate::args::matches::ArgMatches;
use clap::ArgMatches as Args;
use std::str::FromStr;

/* Clap single argument parser */
#[derive(Debug)]
pub struct ArgParser<'a> {
	pub args: Option<ArgMatches<'a>>,
}

impl<'a> ArgParser<'a> {
	/**
	 * Create a new ArgParser object.
	 *
	 * @param  args (Option)
	 * @return ArgParser
	 */
	pub fn new(args: Option<ArgMatches<'a>>) -> Self {
		Self { args }
	}

	/**
	 * Create a new ArgParser object from clap ArgMatches.
	 *
	 * @param  args
	 * @return ArgParser
	 */
	#[allow(dead_code)]
	pub fn from_args(args: &'a Args<'a>) -> Self {
		Self::new(Some(ArgMatches::new(args)))
	}

	/**
	 * Create a new ArgParser object from a name of subcommand.
	 *
	 * @param  matches
	 * @param  name
	 * @return ArgParser
	 */
	pub fn from_subcommand(matches: &'a ArgMatches<'a>, name: &'a str) -> Self {
		let mut matches = matches.clone();
		matches.section = name;
		let mut args = matches.subcommand();
		while args.0 != name {
			args = match args.1 {
				Some(subcommand) => subcommand.subcommand(),
				None => break,
			}
		}
		match args.1 {
			Some(v) => {
				matches.args = v;
				Self::new(Some(matches))
			}
			None => Self::new(None),
		}
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
			.as_ref()
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
	use pretty_assertions::assert_eq;
	#[test]
	fn test_parser() {
		let args = Args::default();
		let matches = ArgMatches::new(&args);
		let parser = ArgParser::from_args(&args);
		assert_eq!(1, parser.parse("test", 1));
		assert!(ArgParser::from_subcommand(&matches, "test").args.is_none())
	}
}
