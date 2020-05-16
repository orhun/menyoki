use crate::x11::window::Window;
use device_query::{DeviceQuery, DeviceState, Keycode};
use std::mem::MaybeUninit;
use std::ptr;
use std::thread;
use std::time::Duration;
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

	pub fn select_focused_window(
		&self,
		device_state: DeviceState,
		gc: xlib::GC,
	) -> Option<Window> {
		let mut mouse_state = device_state.get_mouse();
		let mut focused_window = self.get_focused_window();
		while !(mouse_state.button_pressed[1] || mouse_state.button_pressed[3]) {
			if device_state.get_keys().contains(&Keycode::Escape) {
				return None;
			}
			focused_window = self.get_focused_window();
			focused_window.draw_borders(gc, 5);
			mouse_state = device_state.get_mouse();
			thread::sleep(Duration::from_millis(10));
		}
		focused_window.clear_area();
		Some(focused_window)
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
