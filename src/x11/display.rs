use std::ptr;
use x11::xlib;

pub struct Display {
	display: *mut xlib::Display,
}

impl Display {
	pub fn open() -> Option<Self> {
		unsafe {
			let display = xlib::XOpenDisplay(ptr::null());
			if !display.is_null() {
				Some(Self { display })
			} else {
				None
			}
		}
	}
}
