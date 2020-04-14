use std::ptr;
use x11::xlib::{self, Display, Screen, Window};

pub struct Handler {
	display: *mut Display,
	screen: *mut Screen,
	window: Window,
}

impl Handler {
	pub fn new() -> Result<Self, &'static str> {
		unsafe {
			let display = xlib::XOpenDisplay(ptr::null());
			if !display.is_null() {
				return Err("Cannot open display.");
			}
			let screen = xlib::XDefaultScreenOfDisplay(display);
			let window = xlib::XRootWindowOfScreen(screen);
			Ok(Self {
				display,
				screen,
				window,
			})
		}
	}
}
