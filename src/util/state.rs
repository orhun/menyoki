use device_query::{DeviceQuery, DeviceState, Keycode};
use std::fmt;

/* State of the mouse and keyboard inputs */
pub struct InputState {
	pub state: DeviceState,
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
		match self.state.get_keys().as_slice() {
			[Keycode::S, Keycode::LAlt] => true,
			[Keycode::Enter, Keycode::LAlt] => true,
			_ => false,
		}
	}

	/**
	 * Check if the cancel keys are pressed.
	 *
	 * @return bool
	 */
	pub fn check_cancel_keys(&self) -> bool {
		match self.state.get_keys().as_slice() {
			[Keycode::Escape] => true,
			[Keycode::LControl, Keycode::D] => true,
			_ => false,
		}
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
		enigo.mouse_move_to(0, 0);
		assert_eq!(
			format!("{:?}", input_state),
			"InputState { mouse: MouseState { coords: (0, 0), \
			button_pressed: [false, false, false, false, false, false] }, keys: [] }"
		);
	}
}
