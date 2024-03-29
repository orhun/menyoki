use crate::util::keys::{ActionKeys, CancelKeys, KeyType};
use device_query::{DeviceQuery, DeviceState};
use std::fmt;

/* State of the mouse and keyboard inputs */
pub struct InputState {
	pub state: DeviceState,
	pub action_keys: ActionKeys,
	pub cancel_keys: CancelKeys,
	check_mouse: bool,
}

/* Default initialization values for InputState */
impl Default for InputState {
	fn default() -> Self {
		Self::new(
			ActionKeys::default(KeyType::ActionKeys),
			CancelKeys::default(KeyType::CancelKeys),
			false,
		)
	}
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
	 * @param  action_keys
	 * @param  cancel_keys
	 * @param  check_mouse
	 * @return InputState
	 */
	pub fn new(
		action_keys: ActionKeys,
		cancel_keys: CancelKeys,
		check_mouse: bool,
	) -> Self {
		Self {
			state: DeviceState::new(),
			action_keys,
			cancel_keys,
			check_mouse,
		}
	}

	/**
	 * Get the Box'ed value.
	 *
	 * @return Box
	 */
	pub fn into_boxed_state(self) -> Box<Self> {
		Box::new(self)
	}

	/**
	 * Check for action keys and mouse state to see if there is any action.
	 *
	 * @return bool
	 */
	pub fn check_action(&self) -> bool {
		let keys_pressed = self.action_keys.check(self.state.get_keys());
		if self.check_mouse {
			keys_pressed || self.state.get_mouse().button_pressed[1]
		} else {
			keys_pressed
		}
	}

	/**
	 * Check if the cancel keys are pressed.
	 *
	 * @return bool
	 */
	pub fn check_cancel_keys(&self) -> bool {
		self.cancel_keys.check(self.state.get_keys())
	}
}

#[cfg(test)]
#[cfg(feature = "test-ws")]
mod tests {
	use super::*;
	#[test]
	fn test_input_state() {
		let input_state = InputState::default().into_boxed_state();
		assert!(!input_state.check_action());
		assert!(!input_state.check_cancel_keys());
		assert!(format!("{:?}", input_state).len() > 0);
	}
}
