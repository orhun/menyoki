use crate::args::matches::ArgMatches;
use crate::args::parser::ArgParser;
use crate::edit::settings::EditSettings;
use crate::file::format::FileFormat;
use crate::file::info::FileInfo;
use crate::file::File;
use crate::image::settings::PnmSettings;
use std::path::PathBuf;
use std::str::FromStr;

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
	 * Create a new SaveSettings object from arguments.
	 *
	 * @param  matches
	 * @param  edit
	 * @return SaveSettings
	 */
	pub fn from_args(
		matches: &ArgMatches<'_>,
		edit: &EditSettings,
		pnm: &PnmSettings,
	) -> Self {
		let format = FileFormat::from_args(matches, Some(pnm.subtype));
		Self::from_parser(
			ArgParser::from_subcommand(matches, "save"),
			if edit.convert {
				format
			} else {
				FileFormat::from_str(
					edit.path
						.extension()
						.unwrap_or_default()
						.to_str()
						.unwrap_or_default(),
				)
				.unwrap_or(format)
			},
		)
	}

	/**
	 * Create a SaveSettings object from from an argument parser.
	 *
	 * @param  parser
	 * @param  file_format
	 * @return SaveSettings
	 */
	fn from_parser(parser: ArgParser<'_>, file_format: FileFormat) -> Self {
		match parser.args {
			Some(matches) => {
				let mut path =
					PathBuf::from(matches.value_of("file").unwrap_or_default());
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
			None => Self::new(file_format.into_file()),
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
								Arg::with_name("file")
									.long("file")
									.takes_value(true),
							)
							.arg(Arg::with_name("date").long("date")),
					),
				),
			)
			.get_matches_from(vec![
				"test", "capture", "jpg", "save", "--file", "test.jpg", "--date",
			]);
		let matches = ArgMatches::new(&args);
		let save_settings = SaveSettings::from_parser(
			ArgParser::from_subcommand(&matches, "save"),
			FileFormat::from_args(&matches, None),
		);
		assert!(save_settings.file.path.to_str().unwrap().contains("test_"));
		assert_eq!(FileFormat::Jpg, save_settings.file.format);
	}
}
