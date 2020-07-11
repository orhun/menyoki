use crate::image::padding::Padding;
use std::convert::TryFrom;

/* Position and size in 2D */
#[derive(Clone, Copy, Debug, Default)]
pub struct Geometry {
	pub x: i32,
	pub y: i32,
	pub width: u32,
	pub height: u32,
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
	 * Get a new Geometry object with padding value.
	 *
	 * @param padding
	 */
	pub fn with_padding(&mut self, padding: Padding) -> Self {
		self.x = self
			.x
			.checked_add(i32::try_from(padding.right).unwrap_or_default())
			.unwrap_or(self.x);
		self.y = self
			.y
			.checked_add(i32::try_from(padding.top).unwrap_or_default())
			.unwrap_or(self.y);
		self.width = self
			.width
			.checked_sub(padding.right + padding.left)
			.unwrap_or(self.width);
		self.height = self
			.height
			.checked_sub(padding.top + padding.bottom)
			.unwrap_or(self.height);
		*self
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn test_geometry() {
		let padding = Padding::new(10, 20, 30, 40);
		let geometry = Geometry::new(0, 0, 200, 200).with_padding(padding);
		assert_eq!(20, geometry.x);
		assert_eq!(10, geometry.y);
		assert_eq!(140, geometry.width);
		assert_eq!(160, geometry.height);
	}
}
