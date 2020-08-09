use crate::args::parser::ArgParser;
use crate::util::file::File;
use clap::ArgMatches;
use std::fs;
use std::path::PathBuf;

/* GIF and frame settings */
#[derive(Debug)]
pub struct GifSettings {
	pub fps: u32,
	pub repeat: i32,
	pub quality: u8,
	pub fast: bool,
	pub speed: f32,
	pub frames: Vec<PathBuf>,
}

/* Default initialization values for GifSettings */
impl Default for GifSettings {
	fn default() -> Self {
		Self {
			fps: 10,
			repeat: -1,
			quality: 75,
			fast: false,
			speed: 1.,
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
	 * @param  fast
	 * @param  speed
	 * @param  frames
	 * @return GifSettings
	 */
	pub fn new(
		fps: u32,
		repeat: i32,
		quality: u8,
		fast: bool,
		speed: f32,
		frames: Vec<PathBuf>,
	) -> Self {
		Self {
			fps,
			repeat,
			quality,
			fast,
			speed,
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
				matches.is_present("fast"),
				parser.parse("speed", Self::default().speed),
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

#[derive(Debug)]
pub struct SplitSettings {
	pub file: PathBuf,
	pub dir: PathBuf,
}

/* Default initialization values for GifSettings */
impl Default for SplitSettings {
	fn default() -> Self {
		Self {
			file: PathBuf::new(),
			dir: PathBuf::new(),
		}
	}
}

impl SplitSettings {
	/**
	 * Create a new SplitSettings object.
	 *
	 * @param  file
	 * @param  dir
	 * @return SplitSettings
	 */
	pub fn new(file: PathBuf, dir: PathBuf) -> Self {
		Self { file, dir }
	}

	/**
	 * Create a SplitSettings object from parsed arguments.
	 *
	 * @param  parser
	 * @return SplitSettings
	 */
	pub fn from_args(parser: ArgParser<'_>) -> Self {
		match parser.args {
			Some(matches) => {
				let file =
					PathBuf::from(matches.value_of("file").unwrap_or_default());
				let dir = match matches.value_of("dir") {
					Some(dir) => PathBuf::from(dir),
					None => File::get_default_path(&format!(
						"{}_frames",
						file.file_stem()
							.unwrap_or_default()
							.to_str()
							.unwrap_or_default(),
					)),
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
			.arg(Arg::with_name("fast").long("fast"))
			.arg(Arg::with_name("speed").long("speed").takes_value(true))
			.get_matches_from(vec![
				"test",
				"--fps",
				"15",
				"--repeat",
				"5",
				"--quality",
				"10",
				"--fast",
				"--speed",
				"1.1",
			]);
		let gif_settings = GifSettings::from_args(ArgParser::new(Some(&args)));
		assert_eq!(15, gif_settings.fps);
		assert_eq!(4, gif_settings.repeat);
		assert_eq!(10, gif_settings.quality);
		assert_eq!(true, gif_settings.fast);
		assert_eq!(1.1, gif_settings.speed);
		let gif_settings = GifSettings::from_args(ArgParser::new(None));
		assert_eq!(-1, gif_settings.repeat);
		assert_eq!(75, gif_settings.quality);
		assert_eq!(false, gif_settings.fast);
		assert_eq!(1.0, gif_settings.speed);
	}
}
