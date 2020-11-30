pub mod decoder;
pub mod settings;

use crate::image::Image;
use std::fmt;

/* Images to encode and FPS value */
pub type Frames = (Vec<Image>, u32);

/* Animation format */
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AnimFormat {
	Gif,
	Apng,
}

/* Display implementation for user-facing output */
impl fmt::Display for AnimFormat {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{:?}", self)
	}
}

/* Animation related subcommands */
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AnimMode {
	Record(AnimFormat),
	Edit(AnimFormat),
	Make,
}

impl AnimMode {
	/* Check if the animation mode is edit.
	 *
	 * @return bool
	 */
	pub fn is_edit(&self) -> bool {
		matches!(self, Self::Edit(_))
	}

	/* Check if the animation mode has a certain format.
	 *
	 * @param  format
	 * @return bool
	 */
	pub fn has_format(&self, format: AnimFormat) -> bool {
		match self {
			Self::Record(f) => f == &format,
			Self::Edit(f) => f == &format,
			_ => false,
		}
	}

	/* Get the mode description.
	 *
	 * @return str
	 */
	pub fn get_description<'a>(&self) -> &'a str {
		match self {
			Self::Record(format) | Self::Edit(format) => match format {
				AnimFormat::Gif => "Use the GIF encoder",
				AnimFormat::Apng => "Use the APNG encoder",
			},
			Self::Make => "Make an animation from frames",
		}
	}
}

/* Display implementation for user-facing output */
impl fmt::Display for AnimMode {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"{}",
			match self {
				Self::Record(format) => format.to_string(),
				Self::Edit(format) => format.to_string(),
				_ => format!("{:?}", self),
			}
		)
	}
}
