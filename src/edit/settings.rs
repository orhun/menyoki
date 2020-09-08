use crate::args::parser::ArgParser;
use crate::edit::ImageOps;
use crate::image::geometry::Geometry;
use crate::image::padding::Padding;
use image::imageops::FilterType;
use std::path::PathBuf;

/* Image settings */
#[derive(Clone, Copy, Debug)]
pub struct ImageSettings {
	pub crop: Padding,
	pub resize: Geometry,
	pub ratio: f32,
	pub flip: Option<Flip>,
	pub rotate: u32,
	pub blur: f32,
	pub filter: FilterType,
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
			filter: FilterType::Lanczos3,
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
	 * @param  filter
	 */
	pub fn new(
		crop: Padding,
		resize: Geometry,
		ratio: f32,
		flip: Option<Flip>,
		rotate: u32,
		blur: f32,
		filter: FilterType,
	) -> Self {
		Self {
			crop,
			resize,
			ratio,
			flip,
			rotate,
			blur,
			filter,
		}
	}
}

/* Image color settings */
#[derive(Clone, Copy, Debug)]
pub struct ColorSettings {
	pub grayscale: bool,
	pub invert: bool,
	pub hue: i32,
	pub contrast: f32,
	pub brightness: i32,
}

/* Default initialization values for ColorSettings */
impl Default for ColorSettings {
	fn default() -> Self {
		Self {
			grayscale: false,
			invert: false,
			hue: 0,
			contrast: 0.,
			brightness: 0,
		}
	}
}

impl ColorSettings {
	/**
	 * Create a new ColorSettings object.
	 *
	 * @param  grayscale
	 * @param  invert
	 * @param  hue
	 * @param  contrast
	 * @param  brightness
	 * @return ColorSettings
	 */
	pub fn new(
		grayscale: bool,
		invert: bool,
		hue: i32,
		contrast: f32,
		brightness: i32,
	) -> Self {
		Self {
			grayscale,
			invert,
			hue,
			contrast,
			brightness,
		}
	}
}

/* Flip direction */
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Flip {
	Horizontal,
	Vertical,
}

/* Image editing settings */
#[derive(Debug)]
pub struct EditSettings {
	pub path: PathBuf,
	pub convert: bool,
	pub image: ImageSettings,
	pub color: ColorSettings,
}

/* Default initialization values for EditSettings */
impl Default for EditSettings {
	fn default() -> Self {
		Self {
			path: PathBuf::new(),
			convert: false,
			image: ImageSettings::default(),
			color: ColorSettings::default(),
		}
	}
}

impl EditSettings {
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
		path: PathBuf,
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
	pub fn from_args(parser: ArgParser<'_>) -> Self {
		match parser.args {
			Some(matches) => Self::new(
				PathBuf::from(matches.value_of("input").unwrap_or_default()),
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
					match matches.value_of("filter") {
						Some("nearest") => FilterType::Nearest,
						Some("triangle") => FilterType::Triangle,
						Some("catmull-rom") => FilterType::CatmullRom,
						Some("gaussian") => FilterType::Gaussian,
						_ => FilterType::Lanczos3,
					},
				),
				ColorSettings::new(
					matches.is_present("grayscale"),
					matches.is_present("invert"),
					parser.parse("hue", ColorSettings::default().hue),
					parser.parse("contrast", ColorSettings::default().contrast),
					parser.parse("brightness", ColorSettings::default().brightness),
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
	pub fn get_imageops(&self) -> ImageOps<'_> {
		ImageOps::new(self)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use clap::{App, Arg};
	use pretty_assertions::assert_eq;
	#[test]
	fn test_edit_settings() {
		let args = App::new("test")
			.arg(Arg::with_name("input"))
			.arg(Arg::with_name("convert").long("convert"))
			.arg(Arg::with_name("grayscale").long("grayscale"))
			.arg(Arg::with_name("invert").long("invert"))
			.arg(Arg::with_name("crop").long("crop").takes_value(true))
			.arg(Arg::with_name("resize").long("resize").takes_value(true))
			.arg(Arg::with_name("ratio").long("ratio").takes_value(true))
			.arg(Arg::with_name("flip").long("flip").takes_value(true))
			.arg(Arg::with_name("rotate").long("rotate").takes_value(true))
			.arg(Arg::with_name("blur").long("blur").takes_value(true))
			.arg(Arg::with_name("hue").long("hue").takes_value(true))
			.arg(
				Arg::with_name("contrast")
					.long("contrast")
					.allow_hyphen_values(true)
					.takes_value(true),
			)
			.arg(
				Arg::with_name("brightness")
					.long("brightness")
					.takes_value(true),
			)
			.arg(Arg::with_name("filter").long("filter").takes_value(true))
			.get_matches_from(vec![
				"test",
				"x",
				"--convert",
				"--crop",
				"10",
				"--resize",
				"100:100",
				"--ratio",
				"0.5",
				"--flip",
				"horizontal",
				"--rotate",
				"90",
				"--blur",
				"1.5",
				"--grayscale",
				"--invert",
				"--hue",
				"3",
				"--contrast",
				"-5",
				"--brightness",
				"2",
				"--filter",
				"triangle",
			]);
		let edit_settings = EditSettings::from_args(ArgParser::new(Some(&args)));
		assert_eq!(PathBuf::from("x"), edit_settings.path);
		assert_eq!(true, edit_settings.convert);
		assert_eq!(10, edit_settings.image.crop.top);
		assert_eq!(0.5, edit_settings.image.ratio);
		assert_eq!(Some(Flip::Horizontal), edit_settings.image.flip);
		assert_eq!(90, edit_settings.image.rotate);
		assert_eq!(1.5, edit_settings.image.blur);
		assert_eq!(true, edit_settings.color.grayscale);
		assert_eq!(true, edit_settings.color.invert);
		assert_eq!(2, edit_settings.color.brightness);
		assert_eq!(3, edit_settings.color.hue);
		assert_eq!(-5., edit_settings.color.contrast);
		assert_eq!("Triangle", format!("{:?}", edit_settings.image.filter));
	}
}
