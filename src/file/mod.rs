pub mod format;
pub mod info;
pub mod settings;

use crate::file::format::FileFormat;
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};

/* Extension of the configuration file */
const CONFIG_FILE_EXTENSION: &str = "conf";

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
	 * Create the path if it does not exist.
	 *
	 * @param path
	 */
	fn create_path(path: &Path) {
		if !path.exists() && !cfg!(test) {
			fs::create_dir_all(path.parent().expect("Failed to get the directory"))
				.unwrap_or_else(|_| {
					panic!("Failed to create directory: {:?}", path)
				});
		}
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
			Some("*") | None => path.with_extension(format.as_extension()),
			_ => path,
		}
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
	 * Get a possible configuration file path.
	 *
	 * @return PathBuf (Option)
	 */
	pub fn get_config_file() -> Option<PathBuf> {
		if let Ok(config_file) =
			env::var(format!("{}_CONFIG", env!("CARGO_PKG_NAME")).to_uppercase())
		{
			return Some(PathBuf::from(config_file));
		}
		if let Some(config_dir) = dirs::config_dir() {
			let file_name =
				format!("{}.{}", env!("CARGO_PKG_NAME"), CONFIG_FILE_EXTENSION);
			for config_file in vec![
				config_dir.join(&file_name),
				config_dir.join(env!("CARGO_PKG_NAME")).join(&file_name),
				config_dir.join(env!("CARGO_PKG_NAME")).join("config"),
			] {
				if config_file.exists() {
					return Some(config_file);
				}
			}
		}
		None
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::args::matches::ArgMatches;
	use crate::file::info::FileInfo;
	use clap::{App, Arg, SubCommand};
	use pretty_assertions::assert_eq;
	#[test]
	fn test_file() {
		for format in &["png", "jpg", "bmp", "ico", "tiff", "tga", "pnm", "ff"] {
			let args = App::new("test")
				.subcommand(
					SubCommand::with_name("capture")
						.subcommand(SubCommand::with_name(format)),
				)
				.get_matches_from(vec!["test", "capture", format]);
			assert_eq!(
				File::get_default_path(&format!("cap.{}", format))
					.to_str()
					.unwrap(),
				FileFormat::from_args(&ArgMatches::new(&args), None)
					.into_file()
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
		for info in ["", "date", "timestamp"] {
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
