use std::iter::FromIterator;

/* Padding properties */
#[derive(Clone, Copy, Debug, Default)]
pub struct Padding {
	pub top: u32,
	pub right: u32,
	pub bottom: u32,
	pub left: u32,
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
