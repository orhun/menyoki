use crate::x11::display::Display;
use crate::x11::window::Window;

struct Overlay {
	display: Display,
	window: Window,
}

impl Overlay {
	pub fn new(display: Display, window: Window) -> Self {
		Self { display, window }
	}
}
