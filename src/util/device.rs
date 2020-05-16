use device_query::{DeviceQuery, DeviceState as DevState, Keycode};

pub struct DeviceState {
	state: DevState,
	mouse: Vec<bool>,
	keys: Vec<Keycode>,
	pub mouse_clicked: bool,
	pub exit_keys_pressed: bool,
}

impl DeviceState {
	pub fn new() -> Self {
		Self {
			state: DevState::new(),
			mouse: Vec::new(),
			keys: Vec::new(),
			mouse_clicked: false,
			exit_keys_pressed: false,
		}
	}

	pub fn update(&mut self) {
		self.mouse = self.state.get_mouse().button_pressed;
		self.keys = self.state.get_keys();
		self.mouse_clicked = self.check_mouse_clicked();
		self.exit_keys_pressed = self.check_exit_keys_pressed();
	}

	fn check_mouse_clicked(&mut self) -> bool {
		self.mouse[1] || self.mouse[3]
	}

	fn check_exit_keys_pressed(&mut self) -> bool {
		self.keys.contains(&Keycode::Escape)
			|| (self.keys.contains(&Keycode::LControl)
				&& self.keys.contains(&Keycode::D))
	}
}
