use crate::file::File;
use clap::{ArgMatches as Args, Values};
use ini::Ini as Config;
use std::env::{self, VarError};
use std::fmt;
use std::path::PathBuf;

/* clap::ArgMatches wrapper with config file */
#[derive(Clone)]
pub struct ArgMatches<'a> {
	pub args: &'a Args<'a>,
	pub config: Option<Config>,
	pub section: &'a str,
}

/* Debug implementation for programmer-facing output */
impl fmt::Debug for ArgMatches<'_> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("ArgMatches")
			.field("args", &self.args)
			.field("config", &self.config.is_some())
			.field("section", &self.section)
			.finish()
	}
}

impl<'a> ArgMatches<'a> {
	/**
	 * Create a new ArgMatches object.
	 *
	 * @param  args
	 * @return ArgMatches
	 */
	pub fn new(args: &'a Args<'a>) -> Self {
		let config = if let Some(config_file) = args
			.value_of("config")
			.map_or(File::get_config_file().take(), |v| Some(PathBuf::from(v)))
		{
			Config::load_from_file(config_file).ok()
		} else {
			None
		};
		Self {
			args,
			config,
			section: "general",
		}
	}

	/**
	 * Get argument value from an environment variable.
	 *
	 * @param  name
	 * @return String (Result)
	 */
	fn get_env(&self, name: &'a str) -> Result<String, VarError> {
		env::var(
			format!(
				"{}_{}_{}",
				env!("CARGO_PKG_NAME"),
				self.section,
				name.replace('-', "_")
			)
			.to_uppercase(),
		)
	}

	/**
	 * Get the value of a specific option or positional argument.
	 *
	 * @param  name
	 * @return str (Option)
	 */
	pub fn value_of(&self, name: &'a str) -> Option<&str> {
		self.get_env(name).map_or(
			match self.config {
				Some(ref config) => config
					.get_from(Some(self.section), name)
					.or_else(|| self.args.value_of(name)),
				None => self.args.value_of(name),
			},
			|v| Some(Box::leak(v.into_boxed_str())),
		)
	}

	/**
	 * Check the argument is present in runtime.
	 *
	 * @param  name
	 * @return bool
	 */
	pub fn is_present(&self, name: &'a str) -> bool {
		self.args.is_present(name)
			|| if let Some(config) = &self.config {
				config
					.get_from(Some(self.section), name)
					.map_or(false, |s| s.to_lowercase() == "true")
			} else {
				false
			} || self
			.get_env(name)
			.map_or(false, |s| s.to_lowercase() == "true")
	}

	/**
	 * Get the number of times an argument was used at runtime.
	 *
	 * @param  name
	 * @return u64
	 */
	pub fn occurrences_of(&self, name: &'a str) -> u64 {
		self.get_env(name).map_or(
			match self.config {
				Some(ref config) => config
					.get_from(Some(self.section), name)
					.map_or(0, |s| s.parse().unwrap_or(1)),
				None => self.args.occurrences_of(name),
			},
			|v| v.parse().unwrap_or(1),
		)
	}

	/**
	 * Get the subcommand matches with names.
	 *
	 * @return tuple (str, Args)
	 */
	pub fn subcommand(&self) -> (&'a str, Option<&'a Args<'a>>) {
		self.args.subcommand()
	}

	/**
	 * Get the subcommand matches.
	 *
	 * @param  name
	 * @return Args (Option)
	 */
	pub fn subcommand_matches(&'a self, name: &'a str) -> Option<&'a Args<'a>> {
		self.args.subcommand_matches(name)
	}

	/**
	 * Get the values of an argument.
	 *
	 * @param  name
	 * @return Values (Option)
	 */
	pub fn values_of(&self, name: &'a str) -> Option<Values<'a>> {
		self.args.values_of(name)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use pretty_assertions::assert_eq;
	#[test]
	fn test_matches() {
		let args = Args::default();
		let matches = ArgMatches::new(&args);
		let key = concat!(env!("CARGO_PKG_NAME"), "_general_test").to_uppercase();
		env::set_var(&key, "...");
		assert_eq!("...", matches.get_env("test").unwrap_or_default());
		assert_eq!(Some("..."), matches.value_of("test"));
		assert_eq!(None, matches.value_of("test_"));
		env::set_var(&key, "true");
		assert_eq!(true, matches.is_present("test"));
		assert_eq!(false, matches.is_present("test_"));
		env::set_var(&key, "5");
		assert_eq!(5, matches.occurrences_of("test"));
		assert_eq!(0, matches.occurrences_of("test_"));
		assert_eq!(
			format!("{matches:?}"),
			"ArgMatches { args: ArgMatches { args: {}, \
			subcommand: None, usage: None }, config: false, section: \"general\" }"
		);
	}
}
