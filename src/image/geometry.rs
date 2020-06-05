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
	 * @param  padding
	 * @return Geometry
	 */
	pub fn new(
		x: i32,
		y: i32,
		width: u32,
		height: u32,
		padding: Option<Padding>,
	) -> Self {
		let padding = padding.unwrap_or_default();
		Self {
			x: x + i32::try_from(padding.right).unwrap_or_default(),
			y: y + i32::try_from(padding.top).unwrap_or_default(),
			width: width - (padding.right + padding.left),
			height: height - (padding.top + padding.bottom),
		}
	}
}
