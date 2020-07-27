use crate::args::parser::ArgParser;

/* GIF and frame settings */
#[derive(Clone, Copy, Debug)]
pub struct GifSettings {
	pub repeat: i32,
	pub quality: u8,
	pub speed: f32,
	pub fast: bool,
}

/* Default initialization values for GifSettings */
impl Default for GifSettings {
	fn default() -> Self {
		Self {
			repeat: -1,
			quality: 75,
			speed: 1.,
			fast: false,
		}
	}
}

impl GifSettings {
	/**
	 * Create a new GifSettings object.
	 *
	 * @param  repeat
	 * @param  quality
	 * @param  speed
	 * @param  fast
	 * @return GifSettings
	 */
	pub fn new(repeat: i32, quality: u8, speed: f32, fast: bool) -> Self {
		if quality <= 20 {
			warn!("GIF will be encoded in low quality.");
		}
		Self {
			repeat,
			quality,
			speed,
			fast,
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
				parser.parse("repeat", Self::default().repeat) - 1,
				parser.parse("quality", Self::default().quality),
				parser.parse("speed", Self::default().speed),
				matches.is_present("fast"),
			),
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
			.arg(Arg::with_name("repeat").long("repeat").takes_value(true))
			.arg(Arg::with_name("quality").long("quality").takes_value(true))
			.arg(Arg::with_name("speed").long("speed").takes_value(true))
			.arg(Arg::with_name("fast").long("fast"))
			.get_matches_from(vec![
				"test",
				"--repeat",
				"5",
				"--quality",
				"10",
				"--speed",
				"1.1",
				"--fast",
			]);
		let gif_settings = GifSettings::from_args(ArgParser::new(Some(&args)));
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
