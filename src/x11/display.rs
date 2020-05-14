use crate::x11::window::Window;
use std::mem::MaybeUninit;
use std::ptr;
use x11::xlib;

/* X11 display */
pub struct Display {
	display: *mut xlib::Display,
}

/* Implementations for thread-safe usage */
unsafe impl Send for Display {}

impl Display {
	/**
	 * Open a display.
	 *
	 * @return Display (Option)
	 */
	pub fn open() -> Option<Self> {
		let display = unsafe { xlib::XOpenDisplay(ptr::null()) };
		if !display.is_null() {
			Some(Self { display })
		} else {
			None
		}
	}

	pub unsafe fn get_default_screen(&self) -> *mut xlib::Screen {
		xlib::XDefaultScreenOfDisplay(self.display)
	}

	/**
	 * Get the root window of the default screen.
	 *
	 * @return Window
	 */
	#[allow(dead_code)]
	pub fn get_root_window(&self) -> Window {
		unsafe {
			Window::new(
				xlib::XRootWindowOfScreen(self.get_default_screen()),
				self.display,
			)
		}
	}

	/**
	 * Get the focused window.
	 *
	 * @return Window
	 */
	pub fn get_focused_window(&self) -> Window {
		unsafe {
			let mut focus_window = MaybeUninit::<u64>::uninit();
			let mut revert_to_return = MaybeUninit::<i32>::uninit();
			xlib::XGetInputFocus(
				self.display,
				focus_window.as_mut_ptr(),
				revert_to_return.as_mut_ptr(),
			);
			Window::new(*focus_window.as_ptr(), self.display)
		}
	}
}

/* Close the display when the Display object went out of scope. */
impl Drop for Display {
	fn drop(&mut self) {
		unsafe {
			xlib::XCloseDisplay(self.display);
		}
	}
}
