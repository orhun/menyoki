use crate::args::parser::ArgParser;
use crate::edit::ImageOps;
use crate::image::geometry::Geometry;
use crate::image::padding::Padding;
use std::path::Path;

/* Image settings */
#[derive(Clone, Copy, Debug)]
pub struct ImageSettings {
	pub crop: Padding,
	pub resize: Geometry,
	pub ratio: f32,
	pub flip: Option<Flip>,
	pub rotate: u32,
	pub blur: f32,
}

/* Default initialization values for ImageSettings */
impl Default for ImageSettings {
	fn default() -> Self {
		Self {
			crop: Padding::default(),
			resize: Geometry::default(),
			ratio: 1.,
			flip: None,
			rotate: 0,
			blur: 0.,
		}
	}
}

impl ImageSettings {
	/**
	 * Create a new ImageSettings object.
	 *
	 * @param  crop
	 * @param  resize
	 * @param  ratio
	 * @param  flip (Option)
	 * @param  rotate
	 * @param  blur
	 */
	pub fn new(
		crop: Padding,
		resize: Geometry,
		ratio: f32,
		flip: Option<Flip>,
		rotate: u32,
		blur: f32,
	) -> Self {
		Self {
			crop,
			resize,
			ratio,
			flip,
			rotate,
			blur,
		}
	}
}

/* Image color settings */
#[derive(Clone, Copy, Debug)]
pub struct ColorSettings {
	pub grayscale: bool,
	pub invert: bool,
	pub brightness: i32,
	pub hue: i32,
	pub contrast: f32,
}

/* Default initialization values for ColorSettings */
impl Default for ColorSettings {
	fn default() -> Self {
		Self {
			grayscale: false,
			invert: false,
			brightness: 0,
			hue: 0,
			contrast: 0.,
		}
	}
}

impl ColorSettings {
	/**
	 * Create a new ColorSettings object.
	 *
	 * @param  grayscale
	 * @param  invert
	 * @param  brightness
	 * @param  hue
	 * @param  contrast
	 * @return ColorSettings
	 */
	pub fn new(
		grayscale: bool,
		invert: bool,
		brightness: i32,
		hue: i32,
		contrast: f32,
	) -> Self {
		Self {
			grayscale,
			invert,
			brightness,
			hue,
			contrast,
		}
	}
}

/* Flip direction */
#[derive(Clone, Copy, Debug)]
pub enum Flip {
	Horizontal,
	Vertical,
}

/* Image editing settings */
#[derive(Clone, Copy, Debug)]
pub struct EditSettings<'a> {
	pub path: &'a Path,
	pub convert: bool,
	pub image: ImageSettings,
	pub color: ColorSettings,
}

/* Default initialization values for EditSettings */
impl Default for EditSettings<'_> {
	fn default() -> Self {
		Self {
			path: Path::new(""),
			convert: false,
			image: ImageSettings::default(),
			color: ColorSettings::default(),
		}
	}
}

impl<'a> EditSettings<'a> {
	/**
	 * Create a new EditSettings object.
	 *
	 * @param  path
	 * @param  convert
	 * @param  image
	 * @param  color
	 * @return EditSettings
	 */
	pub fn new(
		path: &'a Path,
		convert: bool,
		image: ImageSettings,
		color: ColorSettings,
	) -> Self {
		Self {
			path,
			convert,
			image,
			color,
		}
	}

	/**
	 * Create a EditSettings object from parsed arguments.
	 *
	 * @param  parser
	 * @return EditSettings
	 */
	pub fn from_args(parser: ArgParser<'a>) -> Self {
		match parser.args {
			Some(matches) => Self::new(
				Path::new(matches.value_of("input").unwrap_or_default()),
				matches.is_present("convert"),
				ImageSettings::new(
					Padding::parse(matches.value_of("crop").unwrap_or_default()),
					Geometry::parse(matches.value_of("resize").unwrap_or_default()),
					parser.parse("ratio", ImageSettings::default().ratio),
					match matches.value_of("flip") {
						Some("horizontal") => Some(Flip::Horizontal),
						Some("vertical") => Some(Flip::Vertical),
						_ => None,
					},
					parser.parse("rotate", ImageSettings::default().rotate),
					parser.parse("blur", ImageSettings::default().blur),
				),
				ColorSettings::new(
					matches.is_present("grayscale"),
					matches.is_present("invert"),
					parser.parse("brighten", ColorSettings::default().brightness),
					parser.parse("hue-rotate", ColorSettings::default().hue),
					parser.parse("contrast", ColorSettings::default().contrast),
				),
			),
			None => Self::default(),
		}
	}

	/**
	 * Get ImageOps object from EditSettings.
	 *
	 * @return ImageOps
	 */
	pub fn get_imageops(self) -> ImageOps<'a> {
		ImageOps::new(self)
	}
}
