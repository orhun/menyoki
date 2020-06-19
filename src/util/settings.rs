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
	 * @param  parser
	 * @param  args
	 * @return SaveSettings
	 */
	pub fn from_args<'a>(parser: ArgParser<'_>, args: &'a ArgMatches<'a>) -> Self {
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
					file_name = Self::add_file_info(&file_name, *info);
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

	/**
	 * Append the given information to the file name.
	 *
	 * @param  file_name
	 * @param  info
	 * @return String
	 */
	fn add_file_info(file_name: &str, info: FileInfo) -> String {
		file_name
			.split('.')
			.enumerate()
			.map(|(i, s)| {
				if i == 0 {
					format!("{}_{}", s, info)
				} else {
					format!(".{}", s)
				}
			})
			.collect::<String>()
	}
}
