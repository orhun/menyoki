use crate::args::parser::ArgParser;
use crate::file::{File, FileFormat, FileInfo};
use crate::util;
use std::path::PathBuf;

/* Output file settings */
#[derive(Debug)]
pub struct SaveSettings {
	pub file: File,
}

impl SaveSettings {
	/**
	 * Create a new SaveSettings object.
	 *
	 * @param  file
	 * @return SaveSettings
	 */
	pub fn new(file: File) -> Self {
		Self { file }
	}

	/**
	 * Create a SaveSettings object from parsed arguments.
	 *
	 * @param  parser
	 * @param  file_format
	 * @return SaveSettings
	 */
	pub fn from_args(parser: ArgParser<'_>, file_format: FileFormat) -> Self {
		match parser.args {
			Some(matches) => {
				let mut path =
					PathBuf::from(matches.value_of("output").unwrap_or_default());
				if matches.is_present("prompt") {
					if let Some(file_name) = util::read_input("Enter file name: ") {
						if path.is_dir() {
							path.push(file_name);
						} else {
							path.set_file_name(file_name);
						}
					}
				}
				if let Some(info) = FileInfo::from_args(&matches) {
					path.set_file_name(format!(
						"{}_{}",
						path.file_stem()
							.unwrap_or_default()
							.to_str()
							.unwrap_or_default(),
						info
					));
				}
				Self::new(File::new(
					path,
					file_format,
					matches.is_present("with-extension"),
				))
			}
			None => Self::new(File::from_format(file_format)),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::args::matches::ArgMatches;
	use clap::{App, Arg, SubCommand};
	use pretty_assertions::assert_eq;
	#[test]
	fn test_save_settings() {
		let args = App::new("test")
			.subcommand(
				SubCommand::with_name("capture").subcommand(
					SubCommand::with_name("jpg").subcommand(
						SubCommand::with_name("save")
							.arg(
								Arg::with_name("output")
									.long("output")
									.takes_value(true),
							)
							.arg(Arg::with_name("date").long("date")),
					),
				),
			)
			.get_matches_from(vec![
				"test", "capture", "jpg", "save", "--output", "test.jpg", "--date",
			]);
		let matches = ArgMatches::new(&args);
		let save_settings = SaveSettings::from_args(
			ArgParser::from_subcommand(&matches, "save"),
			FileFormat::from_args(&matches, None),
		);
		assert!(save_settings.file.path.to_str().unwrap().contains("test_"));
		assert_eq!(FileFormat::Jpg, save_settings.file.format);
	}
}
