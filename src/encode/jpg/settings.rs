use crate::args::parser::ArgParser;

/* JPG quality setting */
#[derive(Clone, Copy, Debug)]
pub struct JpgSettings {
	pub quality: u8,
}

/* Default initialization values for JpgSettings */
impl Default for JpgSettings {
	fn default() -> Self {
		Self { quality: 90 }
	}
}

impl JpgSettings {
	/**
	 * Create a new JpgSettings object.
	 *
	 * @param  quality
	 * @return JpgSettings
	 */
	pub fn new(quality: u8) -> Self {
		Self { quality }
	}

	/**
	 * Create a JpgSettings object from parsed arguments.
	 *
	 * @param  parser
	 * @return JpgSettings
	 */
	pub fn from_args(parser: ArgParser<'_>) -> Self {
		match parser.args {
			Some(_) => Self::new(parser.parse("quality", Self::default().quality)),
			None => Self::default(),
		}
	}
}
