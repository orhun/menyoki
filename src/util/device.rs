use device_query::{DeviceQuery, DeviceState as DevState, Keycode};
use std::fmt;

/* State of the mouse and keyboard */
pub struct DeviceState {
	state: DevState,
}

/* Debug implementation for programmer-facing output */
impl fmt::Debug for DeviceState {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("DeviceState")
			.field("mouse", &self.state.get_mouse())
			.field("keys", &self.state.get_keys())
			.finish()
	}
}

impl DeviceState {
	/**
	 * Create a new DeviceState object.
	 *
	 * @return DeviceState
	 */
	pub fn new() -> Self {
		Self {
			state: DevState::new(),
		}
	}

	/**
	 * Check if the mouse left/right buttons are clicked.
	 *
	 * @return bool
	 */
	pub fn check_mouse_clicked(&self) -> bool {
		let mouse = self.state.get_mouse().button_pressed;
		mouse[1] || mouse[3]
	}

	/**
	 * Check if the cancel keys are pressed.
	 *
	 * @return bool
	 */
	pub fn check_cancel_pressed(&self) -> bool {
		let keys = self.state.get_keys();
		keys.contains(&Keycode::Escape)
			|| (keys.contains(&Keycode::LControl) && keys.contains(&Keycode::D))
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn test_device_mod() {
		let mut device_state = DeviceState::new();
		device_state.update();
		assert!(!device_state.check_mouse_clicked());
		assert!(!device_state.check_cancel_keys_pressed());
	}
}
