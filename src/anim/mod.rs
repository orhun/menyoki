pub mod decoder;
pub mod settings;

use crate::image::Image;
use std::fmt;

/* Images to encode and FPS value */
pub type Frames = (Vec<Image>, u32);

/* Animation format */
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AnimFormat {
	Gif,
	Apng,
}

/* Display implementation for user-facing output */
impl fmt::Display for AnimFormat {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{self:?}")
	}
}

/* Animation related subcommands */
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
			Self::Record(f) | Self::Edit(f) => f == &format,
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
				Self::Record(format) | Self::Edit(format) => format.to_string(),
				_ => format!("{self:?}"),
			}
			.to_lowercase()
		)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use pretty_assertions::assert_eq;
	#[test]
	fn test_anim_mode() {
		let anim_format = AnimFormat::Apng;
		assert_eq!("Apng", anim_format.to_string().as_str());
		let anim_mode = AnimMode::Record(anim_format);
		assert!(!anim_mode.is_edit());
		assert!(anim_mode.has_format(AnimFormat::Apng));
		assert_eq!("Use the APNG encoder", anim_mode.get_description());
		assert_eq!("apng", anim_mode.to_string().as_str());
		let anim_mode = AnimMode::Edit(AnimFormat::Gif);
		assert!(anim_mode.is_edit());
		assert!(anim_mode.has_format(AnimFormat::Gif));
		assert_eq!("Use the GIF encoder", anim_mode.get_description());
		assert_eq!("gif", anim_mode.to_string().as_str());
		let anim_mode = AnimMode::Make;
		assert!(!anim_mode.is_edit());
		assert!(!anim_mode.has_format(AnimFormat::Apng));
		assert_eq!("Make an animation from frames", anim_mode.get_description());
		assert_eq!("make", anim_mode.to_string().as_str());
	}
}
