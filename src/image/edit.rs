use crate::args::parser::ArgParser;
use crate::image::geometry::Geometry;
use crate::image::padding::Padding;

#[derive(Clone, Copy, Debug)]
pub struct EditSettings {
	pub crop: Padding,
	pub resize: Geometry,
	pub ratio: f32,
}

/* Default initialization values for GifSettings */
impl Default for EditSettings {
	fn default() -> Self {
		Self {
			crop: Padding::default(),
			resize: Geometry::default(),
			ratio: 1.,
		}
	}
}

impl EditSettings {
	/**
	 * Create a new EditSettings object.
	 *
	 * @param  crop
	 * @param  resize
	 * @param  ratio
	 * @return EditSettings
	 */
	pub fn new(crop: Padding, resize: Geometry, ratio: f32) -> Self {
		Self {
			crop,
			resize,
			ratio,
		}
	}

	/**
	 * Create a EditSettings object from parsed arguments.
	 *
	 * @param  parser
	 * @return EditSettings
	 */
	pub fn from_args(parser: ArgParser<'_>) -> Self {
		match parser.args {
			Some(matches) => Self::new(
				Padding::parse(matches.value_of("crop").unwrap_or_default()),
				Geometry::parse(matches.value_of("resize").unwrap_or_default()),
				parser.parse("ratio", Self::default().ratio),
			),
			None => Self::default(),
		}
	}
}
