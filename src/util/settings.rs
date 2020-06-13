use crate::args::parser::ArgParser;
use chrono::Local;
use clap::ArgMatches;
use std::fmt;

/* Information to include in file name */
#[derive(Debug)]
enum FileInfo {
	Date,
	Timestamp,
}

impl FileInfo {
	/**
	 * Create a FileInfo enum from parsed arguments.
	 *
	 * @param  args
	 * @return FileInfo (Option)
	 */
	fn from_args(args: &ArgMatches<'_>) -> Option<Self> {
		if args.is_present("date") {
			Some(Self::Date)
		} else if args.is_present("timestamp") {
			Some(Self::Timestamp)
		} else {
			None
		}
	}
}

/* Display implementation for user-facing output */
impl fmt::Display for FileInfo {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"{}",
			match self {
				FileInfo::Date => Local::now().format("%Y%m%dT%H%M%S").to_string(),
				FileInfo::Timestamp => Local::now().timestamp().to_string(),
			}
		)
	}
}

/* Output file settings */
#[derive(Debug)]
pub struct SaveSettings {
	pub file: String,
}

impl SaveSettings {
	/**
	 * Create a new SaveSettings object.
	 *
	 * @param  file
	 * @return SaveSettings
	 */
	pub fn new(file: String) -> Self {
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
		match parser.args {
			Some(matches) => {
				let mut file_name =
					String::from(matches.value_of("output").unwrap_or_default());
				if matches.is_present("prompt") {
					file_name = Self::read_input().unwrap_or(file_name);
				}
				if let Some(info) = FileInfo::from_args(&matches) {
					file_name = Self::add_file_info(&file_name, info);
				}
				Self::new(file_name)
			}
			None => Self::new(if args.is_present("record") {
				String::from("t.gif")
			} else {
				String::from("t.png")
			}),
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
