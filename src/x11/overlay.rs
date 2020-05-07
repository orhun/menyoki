use crate::x11::display::Display;
use crate::x11::window::Window;
use std::os;
use std::ptr;
use x11::xlib;

pub struct Overlay {
	pub display: Display,
	parent_window: Window,
	overlay_window: u64,
}

impl Overlay {
	pub fn new(display: Display, parent_window: Window) -> Self {
		Self {
			display,
			parent_window,
			overlay_window: 0,
		}
	}

	pub fn init(&mut self) {
		unsafe {
			let default_screen = xlib::XDefaultScreen(self.display.get());
			let mut window_attributes: xlib::XSetWindowAttributes =
				std::mem::zeroed();
			window_attributes.background_pixel =
				xlib::XBlackPixel(self.display.get(), default_screen);
			self.overlay_window = xlib::XCreateWindow(
				self.display.get(),
				self.parent_window.xid,
				self.parent_window.geometry.x,
				self.parent_window.geometry.y,
				self.parent_window.geometry.height,
				self.parent_window.geometry.width,
				0,
				0,
				xlib::InputOnly as os::raw::c_uint,
				ptr::null_mut(),
				xlib::CWBackPixel,
				&mut window_attributes,
			)
		}
	}
}
