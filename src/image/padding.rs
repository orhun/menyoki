use device_query::Keycode;
use std::fmt;
use std::iter::FromIterator;

/* Padding properties */
#[derive(Clone, Copy, Debug, Default)]
pub struct Padding {
	pub top: u32,
	pub right: u32,
	pub bottom: u32,
	pub left: u32,
}

/* Implementation for building Padding from an iterator */
impl FromIterator<u32> for Padding {
	fn from_iter<I: IntoIterator<Item = u32>>(iter: I) -> Self {
		let mut iter = iter.into_iter();
		Self::new(
			iter.next().unwrap_or_default(),
			iter.next().unwrap_or_default(),
			iter.next().unwrap_or_default(),
			iter.next().unwrap_or_default(),
		)
	}
}

/* Display implementation for user-facing output */
impl fmt::Display for Padding {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"{}:{}:{}:{}",
			self.top, self.right, self.bottom, self.left
		)
	}
}

impl Padding {
	/**
	 * Create a new Padding object.
	 *
	 * @param  top
	 * @param  right
	 * @param  bottom
	 * @param  left
	 * @return Padding
	 */
	pub fn new(top: u32, right: u32, bottom: u32, left: u32) -> Self {
		Self {
			top,
			right,
			bottom,
			left,
		}
	}

	/**
	 * Parse padding from a string.
	 *
	 * @param  padding
	 * @return Padding
	 */
	pub fn parse(padding: &str) -> Self {
		padding
			.split(':')
			.map(|p| p.parse::<u32>().unwrap_or_default())
			.collect()
	}

	/**
	 * Check if the padding values are zero.
	 *
	 * @return bool
	 */
	pub fn is_zero(&self) -> bool {
		self.top == 0 && self.right == 0 && self.bottom == 0 && self.left == 0
	}

	/**
	 * Get Padding struct fields and their modifier key pairs.
	 *
	 * @return Vector of Tuple
	 */
	pub fn get_modifiers(&mut self) -> Vec<(&mut u32, Keycode, Keycode)> {
		vec![
			(&mut self.top, Keycode::Down, Keycode::Up),
			(&mut self.right, Keycode::Right, Keycode::Left),
			(&mut self.bottom, Keycode::Up, Keycode::Down),
			(&mut self.left, Keycode::Left, Keycode::Right),
		]
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn test_padding() {
		let values = "10:30:20:40";
		let padding = Padding::parse(values);
		assert_eq!(format!("{}", padding), values);
		assert!(!padding.is_zero());
		assert_eq!(10, padding.top);
		assert_eq!(30, padding.right);
		assert_eq!(20, padding.bottom);
		assert_eq!(40, padding.left);
	}
}
