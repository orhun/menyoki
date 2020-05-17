pub mod display;
pub mod window;

use crate::app::AppSettings;
use crate::image::{Capture, Image};
use crate::util::device::DeviceState;
use crate::x11::display::Display;

pub struct WindowSystem {
	display: Display,
}

impl WindowSystem {
	pub fn init() -> Option<Self> {
		if let Some(display) = Display::open() {
			Some(Self { display })
		} else {
			None
		}
	}

	pub fn get_record_func(
		&mut self,
		settings: AppSettings,
	) -> impl Fn() -> Option<Image> {
		let mut focused_window = self.display.get_focused_window();
		if !settings.args.is_present("command") {
			focused_window = self
				.display
				.select_focused_window(
					DeviceState::new(),
					focused_window.create_gc(0x00ff_ffff),
				)
				.unwrap();
		}
		focused_window.reset_position();
		move || focused_window.get_image()
	}
}
