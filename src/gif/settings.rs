use crate::args::parser::ArgParser;
use crate::util::file::File;
use clap::ArgMatches;
use std::fs;
use std::path::{Path, PathBuf};

/* GIF and frame settings */
#[derive(Clone, Debug)]
pub struct GifSettings {
	pub fps: u32,
	pub repeat: i32,
	pub quality: u8,
	pub speed: f32,
	pub fast: bool,
	pub frames: Vec<PathBuf>,
}

/* Default initialization values for GifSettings */
impl Default for GifSettings {
	fn default() -> Self {
		Self {
			fps: 10,
			repeat: -1,
			quality: 75,
			speed: 1.,
			fast: false,
			frames: Vec::new(),
		}
	}
}

impl GifSettings {
	/**
	 * Create a new GifSettings object.
	 *
	 * @param  fps
	 * @param  repeat
	 * @param  quality
	 * @param  speed
	 * @param  fast
	 * @param  frames
	 * @return GifSettings
	 */
	pub fn new(
		fps: u32,
		repeat: i32,
		quality: u8,
		speed: f32,
		fast: bool,
		frames: Vec<PathBuf>,
	) -> Self {
		Self {
			fps,
			repeat,
			quality,
			speed,
			fast,
			frames,
		}
	}

	/**
	 * Create a GifSettings object from parsed arguments.
	 *
	 * @param  parser
	 * @return GifSettings
	 */
	pub fn from_args(parser: ArgParser<'_>) -> Self {
		match parser.args {
			Some(matches) => Self::new(
				match parser.parse("fps", Self::default().fps) {
					fps if fps > 0 => fps,
					_ => Self::default().fps,
				},
				parser.parse("repeat", Self::default().repeat) - 1,
				parser.parse("quality", Self::default().quality),
				parser.parse("speed", Self::default().speed),
				matches.is_present("fast"),
				Self::get_frames(matches),
			),
			None => Self::default(),
		}
	}

	/**
	 * Get the frame files from parsed arguments.
	 *
	 * @param  args
	 * @return Vector of PathBuf
	 */
	fn get_frames(args: &ArgMatches<'_>) -> Vec<PathBuf> {
		let mut values = if let Some(dir) = args.value_of("dir") {
			fs::read_dir(dir)
				.expect("Could not read files from directory")
				.map(|entry| {
					entry
						.expect("Failed to get directory entry")
						.path()
						.into_os_string()
						.into_string()
						.unwrap_or_default()
				})
				.collect()
		} else if let Some(values) = args.values_of("frames") {
			values.map(String::from).collect()
		} else {
			Vec::new()
		};
		if !args.is_present("no-sort") {
			values.sort_by(|a, b| natord::compare(a, b));
		}
		values.into_iter().map(PathBuf::from).collect()
	}
}

#[derive(Clone, Copy, Debug)]
pub struct SplitSettings<'a> {
	pub file: &'a Path,
	pub dir: &'a Path,
}

/* Default initialization values for GifSettings */
impl Default for SplitSettings<'_> {
	fn default() -> Self {
		Self {
			file: Path::new(""),
			dir: Path::new(""),
		}
	}
}

impl<'a> SplitSettings<'a> {
	/**
	 * Create a new SplitSettings object.
	 *
	 * @param  file
	 * @param  dir
	 * @return SplitSettings
	 */
	pub fn new(file: &'a Path, dir: &'a Path) -> Self {
		Self { file, dir }
	}

	/**
	 * Create a SplitSettings object from parsed arguments.
	 *
	 * @param  parser
	 * @return SplitSettings
	 */
	pub fn from_args(parser: ArgParser<'a>) -> Self {
		match parser.args {
			Some(matches) => {
				let file = Path::new(matches.value_of("file").unwrap_or_default());
				let dir = match matches.value_of("dir") {
					Some(dir) => Path::new(dir),
					None => Box::leak(
						File::get_default_path(&format!(
							"{}_frames",
							file.file_stem()
								.unwrap_or_default()
								.to_str()
								.unwrap_or_default(),
						))
						.into_boxed_path(),
					),
				};
				Self::new(file, dir)
			}
			None => Self::default(),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use clap::{App, Arg};
	#[test]
	fn test_gif_settings() {
		let args = App::new("test")
			.arg(Arg::with_name("fps").long("fps").takes_value(true))
			.arg(Arg::with_name("repeat").long("repeat").takes_value(true))
			.arg(Arg::with_name("quality").long("quality").takes_value(true))
			.arg(Arg::with_name("speed").long("speed").takes_value(true))
			.arg(Arg::with_name("fast").long("fast"))
			.get_matches_from(vec![
				"test",
				"--fps",
				"15",
				"--repeat",
				"5",
				"--quality",
				"10",
				"--speed",
				"1.1",
				"--fast",
			]);
		let gif_settings = GifSettings::from_args(ArgParser::new(Some(&args)));
		assert_eq!(15, gif_settings.fps);
		assert_eq!(4, gif_settings.repeat);
		assert_eq!(10, gif_settings.quality);
		assert_eq!(1.1, gif_settings.speed);
		assert_eq!(true, gif_settings.fast);
		let gif_settings = GifSettings::from_args(ArgParser::new(None));
		assert_eq!(-1, gif_settings.repeat);
		assert_eq!(75, gif_settings.quality);
		assert_eq!(1.0, gif_settings.speed);
		assert_eq!(false, gif_settings.fast);
	}
}
