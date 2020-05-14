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

	pub fn get(&self) -> *mut xlib::Display {
		self.display
	}

	pub unsafe fn get_default_screen(&self) -> *mut xlib::Screen {
		xlib::XDefaultScreenOfDisplay(self.get())
	}

	/**
	 * Get the root window of the default screen.
	 *
	 * @return Window
	 */
	#[allow(dead_code)]
	pub fn get_root_window(&self) -> Window {
		let root_window: usize;
		unsafe {
			root_window =
				xlib::XRootWindowOfScreen(self.get_default_screen()) as usize;
		};
		Window::new(root_window as u64, self.display)
	}

	/**
	 * Get the focused window.
	 *
	 * @return Window
	 */
	pub fn get_focused_window(&self) -> Window {
		unsafe {
			let mut focus_window = MaybeUninit::<u64>::uninit().assume_init();
			let mut revert_to_return = MaybeUninit::<i32>::uninit().assume_init();
			xlib::XGetInputFocus(
				self.display,
				&mut focus_window,
				&mut revert_to_return,
			);
			Window::new(focus_window, self.display)
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
