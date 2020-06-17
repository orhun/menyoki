use crate::image::padding::Padding;
use device_query::Keycode;

/* Value, and keys for modifying it */
#[derive(Debug)]
pub struct ValueModifier<'a, Value> {
	pub value: &'a mut Value,
	pub increase: Keycode,
	pub decrease: Keycode,
}

impl<'a> ValueModifier<'a, u32> {
	/**
	 * Create a new ValueModifier object.
	 *
	 * @param  value
	 * @param  increase
	 * @param  decrease
	 * @return ValueModifier
	 */
	pub fn new(value: &'a mut u32, increase: Keycode, decrease: Keycode) -> Self {
		Self {
			value,
			increase,
			decrease,
		}
	}

	/**
	 * Create a Vector of ValueModifier object from a Padding.
	 *
	 * @param  padding
	 * @return Vector of ValueModifier
	 */
	pub fn from_padding(padding: &'a mut Padding) -> Vec<Self> {
		vec![
			Self::new(&mut padding.top, Keycode::Down, Keycode::Up),
			Self::new(&mut padding.right, Keycode::Right, Keycode::Left),
			Self::new(&mut padding.bottom, Keycode::Up, Keycode::Down),
			Self::new(&mut padding.left, Keycode::Left, Keycode::Right),
		]
	}
}
