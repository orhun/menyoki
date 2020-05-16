use device_query::{DeviceQuery, DeviceState as DevState, Keycode};

pub struct DeviceState {
	state: DevState,
	mouse: Vec<bool>,
	keys: Vec<Keycode>,
}

impl DeviceState {
	pub fn new() -> Self {
		Self {
			state: DevState::new(),
			mouse: Vec::new(),
			keys: Vec::new(),
		}
	}

	fn update_state(&mut self) {
		self.mouse = self.state.get_mouse().button_pressed;
		self.keys = self.state.get_keys();
	}

	pub fn get_mouse_clicked(&mut self) -> bool {
		self.update_state();
		self.mouse[1] || self.mouse[3]
	}

	pub fn get_exit_keys_pressed(&mut self) -> bool {
		self.update_state();
		self.keys.contains(&Keycode::Escape)
			|| (self.keys.contains(&Keycode::LControl)
				&& self.keys.contains(&Keycode::D))
	}
}
