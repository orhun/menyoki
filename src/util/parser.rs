use clap::ArgMatches;
use std::str::FromStr;

#[derive(Debug)]
pub struct ArgParser<'a> {
	pub args: &'a ArgMatches<'a>,
}

impl<'a> ArgParser<'a> {
	pub fn new(args: &'a ArgMatches<'a>) -> Self {
		Self { args }
	}

	pub fn parse<T: FromStr>(&self, arg: &str, default_value: T) -> T {
		self.args
			.value_of(arg)
			.unwrap_or_default()
			.parse()
			.unwrap_or(default_value)
	}
}
