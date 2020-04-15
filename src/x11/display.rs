use crate::x11::window::Window;
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
	pub fn get_root_window(&self) -> Window {
		let mut root_window: usize = 0;
		unsafe {
			let screen = xlib::XDefaultScreenOfDisplay(self.display);
			root_window = xlib::XRootWindowOfScreen(screen) as usize;
		};
		Window {
			xid: unsafe { root_window },
			display: self.display,
		}
	}
	pub fn get_focused_window(&self) -> Window {
		let focus_window: *mut xlib::Window = &mut 0;
		let revert_to_return: *mut i32 = &mut 0;
		unsafe {
			xlib::XGetInputFocus(self.display, focus_window, revert_to_return)
		};
		Window {
			xid: unsafe { *focus_window as usize },
			display: self.display,
		}
	}
}
