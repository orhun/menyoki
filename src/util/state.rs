use device_query::{DeviceQuery, DeviceState, Keycode};
use std::fmt;

/* State of the mouse and keyboard inputs */
pub struct InputState {
	state: DeviceState,
}

/* Debug implementation for programmer-facing output */
impl fmt::Debug for InputState {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("InputState")
			.field("mouse", &self.state.get_mouse())
			.field("keys", &self.state.get_keys())
			.finish()
	}
}

/* Implementation for thread-safe usage */
unsafe impl Sync for InputState {}

impl InputState {
	/**
	 * Create a new InputState object.
	 *
	 * @return InputState
	 */
	pub fn new() -> Self {
		Self {
			state: DeviceState::new(),
		}
	}

	/**
	 * Check if the action keys are pressed.
	 *
	 * @return bool
	 */
	pub fn check_action_keys(&self) -> bool {
		self.check_key_combination(
			Some(self.state.get_keys()),
			vec![&Keycode::LAlt, &Keycode::S],
		) || self.check_key_combination(
			Some(self.state.get_keys()),
			vec![&Keycode::LAlt, &Keycode::Enter],
		)
	}

	/**
	 * Check if the cancel keys are pressed.
	 *
	 * @return bool
	 */
	pub fn check_cancel_keys(&self) -> bool {
		let keys = self.state.get_keys();
		keys.contains(&Keycode::Escape)
			|| self.check_key_combination(
				Some(keys),
				vec![&Keycode::LControl, &Keycode::D],
			)
	}

	/**
	 * Check if the given keys are pressed or not.
	 *
	 * @param  keys (Option)
	 * @param  target_keys
	 * @return bool
	 */
	pub fn check_key_combination(
		&self,
		keys: Option<Vec<Keycode>>,
		target_keys: Vec<&Keycode>,
	) -> bool {
		let keys = keys.unwrap_or_else(|| self.state.get_keys());
		let mut pressed = keys.len() == target_keys.len();
		for key in target_keys {
			if !keys.contains(key) {
				pressed = false;
				break;
			}
		}
		pressed
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use enigo::*;
	#[test]
	fn test_input_state() {
		let input_state = InputState::new();
		let mut enigo = Enigo::new();
		//enigo.key_down(Key::Escape);
		//assert!(input_state.check_cancel_keys());
		//enigo.key_up(Key::Escape);
		enigo.mouse_move_to(0, 0);
		assert_eq!(
			format!("{:?}", input_state),
			"InputState { mouse: MouseState { coords: (0, 0), \
			button_pressed: [false, false, false, false, false, false] }, keys: [] }"
		);
	}
}
