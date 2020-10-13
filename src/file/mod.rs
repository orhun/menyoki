pub mod settings;

use crate::args::matches::ArgMatches;
use chrono::Local;
use image::pnm::PnmSubtype;
use std::ffi::OsStr;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;

const DEFAULT_FILE_NAME: &str = "t";

/* Information to include in file name */
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FileInfo<'a> {
	Date(&'a str),
	Timestamp,
}

impl<'a> FileInfo<'a> {
	/**
	 * Create a FileInfo enum from parsed arguments.
	 *
	 * @param  args
	 * @return FileInfo (Option)
	 */
	pub fn from_args(args: &'a ArgMatches<'a>) -> Option<Self> {
		if args.is_present("timestamp") {
			Some(Self::Timestamp)
		} else if args.is_present("date") && args.occurrences_of("date") != 0 {
			Some(Self::Date(args.value_of("date").unwrap_or_default()))
		} else {
			None
		}
	}
}

/* Display implementation for user-facing output */
impl fmt::Display for FileInfo<'_> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"{}",
			match self {
				FileInfo::Date(format) => Local::now().format(format).to_string(),
				FileInfo::Timestamp => Local::now().timestamp().to_string(),
			}
		)
	}
}

/* Format of the output file */
#[derive(Clone, Debug, PartialEq)]
pub enum FileFormat {
	Any,
	Gif,
	Png,
	Jpg,
	Bmp,
	Ico,
	Tiff,
	Tga,
	Pnm(String),
	Ff,
	Txt,
}

/* Display implementation for user-facing output */
impl fmt::Display for FileFormat {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{:?}", self)
	}
}

/* Implementation for parsing FileFormat from a string */
impl FromStr for FileFormat {
	type Err = &'static str;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"gif" => Ok(Self::Gif),
			"png" => Ok(Self::Png),
			"jpg" => Ok(Self::Jpg),
			"bmp" => Ok(Self::Bmp),
			"ico" => Ok(Self::Ico),
			"tiff" => Ok(Self::Tiff),
			"tga" => Ok(Self::Tga),
			"pnm" => Ok(Self::Pnm(String::from("ppm"))),
			"ff" => Ok(Self::Ff),
			"txt" => Ok(Self::Txt),
			_ => Err("Unrecognized file format"),
		}
	}
}

impl FileFormat {
	/**
	 * Create a FileFormat enum fron parsed arguments.
	 *
	 * @param  args
	 * @param  pnm_subtype
	 * @return FileFormat
	 */
	pub fn from_args<'a>(
		args: &'a ArgMatches<'a>,
		pnm_subtype: Option<PnmSubtype>,
	) -> Self {
		match args.subcommand_matches(if args.is_present("edit") {
			"edit"
		} else if args.is_present("split") {
			"split"
		} else if args.is_present("analyze") {
			"analyze"
		} else {
			"capture"
		}) {
			Some(matches) => {
				if args.is_present("analyze") {
					if matches.is_present("save") {
						Self::Txt
					} else {
						Self::Any
					}
				} else if matches.is_present("gif") {
					Self::Gif
				} else if matches.is_present("ff") {
					Self::Ff
				} else if matches.is_present("tiff") {
					Self::Tiff
				} else if matches.is_present("tga") {
					Self::Tga
				} else if matches.is_present("pnm") {
					Self::Pnm(String::from(match pnm_subtype {
						Some(PnmSubtype::Bitmap(_)) => "pbm",
						Some(PnmSubtype::Graymap(_)) => "pgm",
						Some(PnmSubtype::Pixmap(_)) => "ppm",
						Some(PnmSubtype::ArbitraryMap) => "pam",
						None => "pnm",
					}))
				} else if matches.is_present("bmp") {
					Self::Bmp
				} else if matches.is_present("ico") {
					Self::Ico
				} else if matches.is_present("jpg") {
					Self::Jpg
				} else {
					Self::Png
				}
			}
			None => Self::Gif,
		}
	}

	/**
	 * Get extension from format.
	 *
	 * @return String
	 */
	pub fn to_extension(&self) -> String {
		match self {
			Self::Any => String::from("*"),
			Self::Pnm(v) => v.to_string(),
			_ => self.to_string(),
		}
		.to_lowercase()
	}
}

/* Representation of the output file */
#[derive(Debug)]
pub struct File {
	pub path: PathBuf,
	pub format: FileFormat,
}

impl File {
	/**
	 * Create a new File object.
	 *
	 * @param  path
	 * @param  format
	 * @param  with_extension
	 * @return File
	 */
	pub fn new(mut path: PathBuf, format: FileFormat, with_extension: bool) -> Self {
		Self::create_path(&path);
		if with_extension || path.extension().and_then(OsStr::to_str) == Some("*") {
			path = Self::get_path_with_extension(path, &format)
		}
		Self { path, format }
	}

	/**
	 * Create a new File object from file format.
	 *
	 * @param  format
	 * @return File
	 */
	pub fn from_format(format: FileFormat) -> Self {
		Self::new(
			Self::get_default_path(&format!(
				"{}.{}",
				DEFAULT_FILE_NAME,
				format.to_extension()
			)),
			format,
			true,
		)
	}

	/**
	 * Get the default path for a file.
	 *
	 * @param  file_name
	 * @param  PathBuf
	 */
	pub fn get_default_path(file_name: &str) -> PathBuf {
		dirs::picture_dir()
			.unwrap_or_else(|| {
				dirs::home_dir()
					.expect("Failed to access the home directory")
					.as_path()
					.join(env!("CARGO_PKG_NAME"))
			})
			.join(file_name)
	}

	/**
	 * Get the path with extension using the given file format.
	 *
	 * @param  path
	 * @param  format
	 * @return PathBuf
	 */
	pub fn get_path_with_extension(path: PathBuf, format: &FileFormat) -> PathBuf {
		match path.extension().and_then(OsStr::to_str) {
			Some("*") | None => path.with_extension(format.to_extension()),
			_ => path,
		}
	}

	/**
	 * Create the path if it does not exist.
	 *
	 * @param path
	 */
	fn create_path(path: &Path) {
		if !path.exists() {
			fs::create_dir_all(&path.parent().expect("Failed to get the directory"))
				.expect("Failed to create directory");
		}
	}

	/**
	 * Get a possible configuration file path.
	 *
	 * @return PathBuf (Option)
	 */
	pub fn get_config_file() -> Option<PathBuf> {
		if let Some(config_dir) = dirs::config_dir() {
			let file_name = format!("{}.conf", env!("CARGO_PKG_NAME"));
			for config_file in vec![
				config_dir.join(&file_name),
				config_dir.join(env!("CARGO_PKG_NAME")).join(&file_name),
			] {
				if config_file.exists() {
					return Some(config_file);
				}
			}
			None
		} else {
			None
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use clap::{App, Arg, SubCommand};
	use pretty_assertions::assert_eq;
	#[test]
	fn test_file() {
		for format in vec!["png", "jpg", "bmp", "ico", "tiff", "tga", "pnm", "ff"] {
			let args = App::new("test")
				.subcommand(
					SubCommand::with_name("capture")
						.subcommand(SubCommand::with_name(format)),
				)
				.get_matches_from(vec!["test", "capture", format]);
			assert_eq!(
				File::get_default_path(&format!("t.{}", format))
					.to_str()
					.unwrap(),
				File::from_format(FileFormat::from_args(
					&ArgMatches::new(&args),
					None
				))
				.path
				.to_str()
				.unwrap()
			);
		}
		assert_eq!(
			"Gif",
			FileFormat::from_args(
				&ArgMatches::new(&App::new("test").get_matches_from(vec!["test"])),
				None
			)
			.to_string()
		);
		for info in vec!["", "date", "timestamp"] {
			let args = App::new("test")
				.arg(Arg::with_name(info).long(&format!("--{}", info)))
				.get_matches_from(vec!["test", &format!("--{}", info)]);
			assert_eq!(
				match info {
					"date" => {
						Some(FileInfo::Date(""))
					}
					"timestamp" => {
						Some(FileInfo::Timestamp)
					}
					_ => None,
				},
				FileInfo::from_args(&ArgMatches::new(&args))
			);
		}
	}
}
