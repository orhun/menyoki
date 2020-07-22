use crate::args::parser::ArgParser;
use crate::edit::Editor;
use crate::image::geometry::Geometry;
use crate::image::padding::Padding;

/* Flip direction */
#[derive(Clone, Copy, Debug)]
pub enum Flip {
	Horizontal,
	Vertical,
}

/* Image editing settings */
#[derive(Clone, Copy, Debug)]
pub struct EditSettings {
	pub crop: Padding,
	pub resize: Geometry,
	pub ratio: f32,
	pub flip: Option<Flip>,
	pub rotate: u32,
	pub blur: f32,
	pub grayscale: bool,
}

/* Default initialization values for GifSettings */
impl Default for EditSettings {
	fn default() -> Self {
		Self {
			crop: Padding::default(),
			resize: Geometry::default(),
			ratio: 1.,
			flip: None,
			rotate: 0,
			blur: 0.,
			grayscale: false,
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
	 * @param  flip (Option)
	 * @param  rotate
	 * @param  blur
	 * @param  grayscale
	 * @return EditSettings
	 */
	pub fn new(
		crop: Padding,
		resize: Geometry,
		ratio: f32,
		flip: Option<Flip>,
		rotate: u32,
		blur: f32,
		grayscale: bool,
	) -> Self {
		Self {
			crop,
			resize,
			ratio,
			flip,
			rotate,
			blur,
			grayscale,
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
				match matches.value_of("flip") {
					Some("horizontal") => Some(Flip::Horizontal),
					Some("vertical") => Some(Flip::Vertical),
					_ => None,
				},
				parser.parse("rotate", Self::default().rotate),
				parser.parse("blur", Self::default().blur),
				matches.is_present("grayscale"),
			),
			None => Self::default(),
		}
	}

	/**
	 * Get Editor object from EditSettings.
	 *
	 * @return Editor
	 */
	pub fn get_editor(self) -> Editor {
		Editor::new(self)
	}
}
