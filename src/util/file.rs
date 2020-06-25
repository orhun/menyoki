use chrono::Local;
use clap::ArgMatches;
use std::fmt;

/* Information to include in file name */
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum FileInfo {
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
	pub fn from_args(args: &ArgMatches<'_>) -> Option<Self> {
		if args.is_present("date") {
			Some(Self::Date)
		} else if args.is_present("timestamp") {
			Some(Self::Timestamp)
		} else {
			None
		}
	}

	/**
	 * Append the file information to the file name.
	 *
	 * @param  file_name
	 */
	pub fn append(self, file_name: &mut String) {
		*file_name = file_name
			.split('.')
			.enumerate()
			.map(|(i, s)| {
				if i == 0 {
					format!("{}_{}", s, self)
				} else {
					format!(".{}", s)
				}
			})
			.collect::<String>();
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

/* Format of the output file */
#[derive(Debug)]
pub enum FileFormat {
	Gif,
	Png,
	Jpg,
	Bmp,
	Tiff,
	Ff,
}

/* Display implementation for user-facing output */
impl fmt::Display for FileFormat {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{:?}", self)
	}
}

impl FileFormat {
	/**
	 * Create a FileFormat enum fron parsed arguments.
	 *
	 * @param  args
	 * @return FileFormat
	 */
	pub fn from_args<'a>(args: &'a ArgMatches<'a>) -> Self {
		match args.subcommand_matches("capture") {
			Some(matches) => {
				if matches.is_present("ff") {
					Self::Ff
				} else if matches.is_present("tiff") {
					Self::Tiff
				} else if matches.is_present("bmp") {
					Self::Bmp
				} else if matches.is_present("jpg") {
					Self::Jpg
				} else {
					Self::Png
				}
			}
			None => Self::Gif,
		}
	}
}

/* Representation of the output file */
#[derive(Debug)]
pub struct File {
	pub name: String,
	pub format: FileFormat,
	pub info: Option<FileInfo>,
}

impl File {
	/**
	 * Create a new File object.
	 *
	 * @param  name
	 * @param  format
	 * @param  info
	 * @return File
	 */
	pub fn new(name: String, format: FileFormat, info: Option<FileInfo>) -> Self {
		Self { name, format, info }
	}

	/**
	 * Create a new File object from file format.
	 *
	 * @param  file_format
	 * @return File
	 */
	pub fn from_format(file_format: FileFormat) -> Self {
		Self::new(
			format!("t.{}", file_format.to_string().to_lowercase()),
			file_format,
			None,
		)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use clap::{App, Arg, SubCommand};
	#[test]
	fn test_file() {
		for format in vec!["png", "jpg", "bmp", "tiff", "ff"] {
			let args = App::new("test")
				.subcommand(
					SubCommand::with_name("capture")
						.subcommand(SubCommand::with_name(format)),
				)
				.get_matches_from(vec!["test", "capture", format]);
			assert_eq!(
				format!("t.{}", format),
				File::from_format(FileFormat::from_args(&args)).name
			);
		}
		assert_eq!(
			"Gif",
			FileFormat::from_args(&App::new("test").get_matches_from(vec!["test"]))
				.to_string()
		);
		for info in vec!["", "date", "timestamp"] {
			let args = App::new("test")
				.arg(Arg::with_name(info).long(&format!("--{}", info)))
				.get_matches_from(vec!["test", &format!("--{}", info)]);
			assert_eq!(
				match info {
					"date" => {
						let file_info = FileInfo::Date;
						let mut file_name = String::from("x.y");
						file_info.append(&mut file_name);
						assert!(file_name.len() > 3);
						Some(file_info)
					}
					"timestamp" => {
						let file_info = FileInfo::Timestamp;
						let mut file_name = String::from("x.y.z");
						file_info.append(&mut file_name);
						assert!(file_name.len() > 5);
						Some(file_info)
					}
					_ => None,
				},
				FileInfo::from_args(&args)
			);
		}
	}
}
