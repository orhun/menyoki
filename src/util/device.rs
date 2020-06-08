use device_query::{DeviceQuery, DeviceState as DevState, Keycode};

/* State of the mouse and keyboard */
pub struct DeviceState {
	state: DevState,
	mouse: Vec<bool>,
	keys: Vec<Keycode>,
	pub mouse_clicked: bool,
	pub cancel_keys_pressed: bool,
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
			mouse: Vec::new(),
			keys: Vec::new(),
			mouse_clicked: false,
			cancel_keys_pressed: false,
		}
	}

	/* Update the keyboard and mouse states. */
	pub fn update(&mut self) {
		self.mouse = self.state.get_mouse().button_pressed;
		self.keys = self.state.get_keys();
		self.mouse_clicked = self.mouse[1] || self.mouse[3];
		self.cancel_keys_pressed = self.check_cancel_keys_pressed();
	}

	/**
	 * Check if the exit keys are pressed.
	 *
	 * @return bool
	 */
	fn check_cancel_keys_pressed(&mut self) -> bool {
		self.keys.contains(&Keycode::Escape)
			|| (self.keys.contains(&Keycode::LControl)
				&& self.keys.contains(&Keycode::D))
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
