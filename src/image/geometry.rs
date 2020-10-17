use crate::image::padding::Padding;
use std::convert::TryFrom;
use std::fmt;
use std::iter::FromIterator;

/* Position and size in 2D */
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Geometry {
	pub x: i32,
	pub y: i32,
	pub width: u32,
	pub height: u32,
}

/* Implementation for building Geometry from an iterator */
impl FromIterator<u32> for Geometry {
	fn from_iter<I: IntoIterator<Item = u32>>(iter: I) -> Self {
		let mut iter = iter.into_iter();
		Self::new(
			0,
			0,
			iter.next().unwrap_or_default(),
			iter.next().unwrap_or_default(),
		)
	}
}

/* Display implementation for user-facing output */
impl fmt::Display for Geometry {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}x{}", self.width, self.height)
	}
}

impl Geometry {
	/**
	 * Create a new Geometry object.
	 *
	 * @param  x
	 * @param  y
	 * @param  width
	 * @param  height
	 * @return Geometry
	 */
	pub fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
		Self {
			x,
			y,
			width,
			height,
		}
	}

	/**
	 * Parse Geometry from a string.
	 *
	 * @param  geometry
	 * @return Geometry
	 */
	pub fn parse(geometry: &str) -> Self {
		geometry
			.split('x')
			.map(|v| v.parse::<u32>().unwrap_or_default())
			.collect()
	}

	/**
	 * Check if width and height values are zero.
	 *
	 * @return bool
	 */
	pub fn is_zero(&self) -> bool {
		self.width == 0 && self.height == 0
	}

	/**
	 * Get a new Geometry object with padding value.
	 *
	 * @param padding
	 */
	pub fn with_padding(&mut self, padding: Padding) -> Self {
		self.x = self
			.x
			.checked_add(i32::try_from(padding.left).unwrap_or_default())
			.unwrap_or_default();
		self.y = self
			.y
			.checked_add(i32::try_from(padding.top).unwrap_or_default())
			.unwrap_or_default();
		self.width = self
			.width
			.checked_sub(padding.right + padding.left)
			.unwrap_or_default();
		self.height = self
			.height
			.checked_sub(padding.top + padding.bottom)
			.unwrap_or_default();
		*self
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use pretty_assertions::assert_eq;
	#[test]
	fn test_geometry() {
		let padding = Padding::new(10, 20, 30, 40);
		let geometry = Geometry::new(0, 0, 200, 200).with_padding(padding);
		assert_eq!(40, geometry.x);
		assert_eq!(10, geometry.y);
		assert_eq!(140, geometry.width);
		assert_eq!(160, geometry.height);
		let values = "45x28";
		let geometry = Geometry::parse(values);
		assert_eq!(values, geometry.to_string());
	}
}
