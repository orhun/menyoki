use crate::util::parser::ArgParser;
use clap::ArgMatches;

/* GIF and frame settings */
#[derive(Clone, Copy, Debug)]
pub struct GifSettings {
	pub repeat: i32,
	pub speed: u32,
}

/* Default initialization values for GifSettings */
impl Default for GifSettings {
	fn default() -> Self {
		Self {
			repeat: -1,
			speed: 10,
		}
	}
}

impl GifSettings {
	/**
	 * Create a new GifSettings object.
	 *
	 * @param  repeat
	 * @param  speed
	 * @return GifSettings
	 */
	pub fn new(repeat: i32, speed: u32) -> Self {
		Self { repeat, speed }
	}

	/**
	 * Create a GifSettings object from parsed arguments.
	 *
	 * @param  args
	 * @return RecordSettings
	 */
	pub fn from_args(args: Option<&ArgMatches<'static>>) -> Self {
		match args {
			Some(matches) => {
				let parser = ArgParser::new(matches.clone());
				Self::new(
					parser.parse("repeat", Self::default().repeat),
					parser.parse("speed", Self::default().speed),
				)
			}
			None => GifSettings::default(),
		}
	}
}
