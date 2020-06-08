use device_query::{DeviceQuery, DeviceState as DevState, Keycode};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

/* State of the mouse and keyboard */
pub struct DeviceState {
	state: DevState,
	mouse: Vec<bool>,
	keys: Vec<Keycode>,
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
		}
	}

	/**
	 * Check if the mouse left/right button clicked.
	 *
	 * @return bool
	 */
	pub fn check_mouse_click(&mut self) -> bool {
		self.mouse = self.state.get_mouse().button_pressed;
		self.mouse[1] || self.mouse[3]
	}

	/**
	 * Check if Ctrl-C is pressed.
	 *
	 * @return AtomicBool (Arc)
	 */
	pub fn check_exit_pressed(&mut self) -> Arc<AtomicBool> {
		let exit_pressed = Arc::new(AtomicBool::new(true));
		let press_state = exit_pressed.clone();
		ctrlc::set_handler(move || {
			press_state.store(false, Ordering::SeqCst);
		})
		.expect("Failed to set the signal handler");
		exit_pressed
	}

	/**
	 * Check if the cancel keys are pressed.
	 *
	 * @return bool
	 */
	pub fn check_cancel_press(&mut self) -> bool {
		self.keys = self.state.get_keys();
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
