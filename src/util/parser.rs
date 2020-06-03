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
	 * @param  args
	 * @return ArgParser
	 */
	pub fn new(args: Option<&'a ArgMatches<'a>>) -> Self {
		Self { args }
	}

	/**
	 * Create a new ArgParser object from a number of subcommands.
	 *
	 * @param  args
	 * @param  subcommands
	 * @return ArgParser
	 */
	pub fn from_subcommand(
		args: &'a ArgMatches<'a>,
		subcommands: Vec<&str>,
	) -> Self {
		let mut matches = args.subcommand_matches(subcommands[0]);
		for subcommand in subcommands.iter().skip(1) {
			matches = matches.and_then(|args| {
				args.subcommand_matches(subcommand).map(|args| args)
			});
		}
		Self::new(matches)
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
			.unwrap()
			.value_of(arg)
			.unwrap_or_default()
			.parse()
			.unwrap_or(default_value)
	}
}
