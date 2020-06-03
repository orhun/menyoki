use crate::util::parser::ArgParser;

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
	 * @param  parser
	 * @return RecordSettings
	 */
	pub fn from_args(parser: ArgParser<'_>) -> Self {
		match parser.args {
			Some(_) => Self::new(
				parser.parse("repeat", Self::default().repeat) - 1,
				parser.parse("speed", Self::default().speed),
			),
			None => GifSettings::default(),
		}
	}
}
