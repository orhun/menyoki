use crate::args::parser::ArgParser;
use crate::util::file::{File, FileFormat, FileInfo};
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
				let mut file_path =
					PathBuf::from(matches.value_of("output").unwrap_or_default());
				let mut file_name = file_path
					.file_name()
					.expect("Invalid file name")
					.to_string_lossy()
					.into_owned();
				if matches.is_present("prompt") {
					file_name = Self::read_input().unwrap_or(file_name);
				}
				if let Some(info) = FileInfo::from_args(&matches) {
					info.append(&mut file_name);
				}
				file_path.set_file_name(file_name);
				Self::new(File::new(file_path, file_format))
			}
			None => Self::new(File::from_format(file_format)),
		}
	}

	/**
	 * Read input from stdin with prompt.
	 *
	 * @return String (Option)
	 */
	fn read_input() -> Option<String> {
		match rprompt::prompt_reply_stdout("Enter file name: ") {
			Ok(v) if !v.is_empty() => Some(v),
			_ => None,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::util::file::{FileFormat, FileInfo};
	use clap::{App, Arg, SubCommand};
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
		let save_settings = SaveSettings::from_args(
			&args,
			ArgParser::from_subcommand(&args, "save"),
		);
		assert!(save_settings.file.name.contains("test_"));
		assert_eq!(FileFormat::Jpg, save_settings.file.format);
		assert_eq!(FileInfo::Date, save_settings.file.info.unwrap());
	}
}
