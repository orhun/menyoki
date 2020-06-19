use crate::args::parser::ArgParser;
use crate::util::file::{File, FileFormat, FileInfo};
use clap::ArgMatches;

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
	 * @param  args
	 * @param  parser
	 * @return SaveSettings
	 */
	pub fn from_args<'a>(args: &'a ArgMatches<'a>, parser: ArgParser<'_>) -> Self {
		let file_format = FileFormat::from_args(args);
		match parser.args {
			Some(matches) => {
				let mut file_name =
					String::from(matches.value_of("output").unwrap_or_default());
				let file_info = FileInfo::from_args(&matches);
				if matches.is_present("prompt") {
					file_name = Self::read_input().unwrap_or(file_name);
				}
				if let Some(info) = &file_info {
					info.append(&mut file_name);
				}
				Self::new(File::new(file_name, file_format, file_info))
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
