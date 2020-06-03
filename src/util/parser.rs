use clap::ArgMatches;
use std::str::FromStr;

/* Clap single argument parser */
#[derive(Debug)]
pub struct ArgParser<'a> {
	pub args: &'a ArgMatches<'a>,
}

impl<'a> ArgParser<'a> {
	/**
	 * Create a new ArgParser object.
	 *
	 * @param  args
	 * @return ArgParser
	 */
	pub fn new(args: &'a ArgMatches<'a>) -> Self {
		Self { args }
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
			.value_of(arg)
			.unwrap_or_default()
			.parse()
			.unwrap_or(default_value)
	}
}
