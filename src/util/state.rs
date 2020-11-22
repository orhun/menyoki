use crate::util::keys::ActionKeys;
use device_query::{DeviceQuery, DeviceState, Keycode};
use std::fmt;

/* State of the mouse and keyboard inputs */
pub struct InputState {
	pub state: DeviceState,
	pub action_keys: ActionKeys,
}

/* Default initialization values for InputState */
impl Default for InputState {
	fn default() -> Self {
		Self::new(ActionKeys::default())
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
	 * @return InputState
	 */
	pub fn new(action_keys: ActionKeys) -> Self {
		Self {
			state: DeviceState::new(),
			action_keys,
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
	 * Check if the action keys are pressed.
	 *
	 * @return bool
	 */
	pub fn check_action_keys(&self) -> bool {
		self.action_keys.check(self.state.get_keys())
	}

	/**
	 * Check if the cancel keys are pressed.
	 *
	 * @return bool
	 */
	pub fn check_cancel_keys(&self) -> bool {
		matches!(
			self.state.get_keys().as_slice(),
			[Keycode::Escape] | [Keycode::LControl, Keycode::D]
		)
	}
}

#[cfg(test)]
#[cfg(feature = "test-ws")]
mod tests {
	use super::*;
	#[test]
	fn test_input_state() {
		let input_state = InputState::default().into_boxed_state();
		assert!(!input_state.check_action_keys());
		assert!(!input_state.check_cancel_keys());
		assert!(format!("{:?}", input_state).len() > 0);
	}
}
