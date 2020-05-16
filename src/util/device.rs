use device_query::{DeviceQuery, DeviceState as DevState, Keycode, MouseState};

struct DeviceState {
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

	fn update(&mut self) {
		self.mouse = self.state.get_mouse().button_pressed;
		self.keys = self.state.get_keys();
	}

	pub fn mouse_clicked(&mut self) -> bool {
		self.update();
		self.mouse[1] && self.mouse[3]
	}

	pub fn exit_keys_pressed(&mut self) -> bool {
		self.update();
		self.keys.contains(&Keycode::Escape)
			|| (self.keys.contains(&Keycode::LControl)
				&& self.keys.contains(&Keycode::D))
	}
}
