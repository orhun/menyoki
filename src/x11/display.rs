use crate::util::device::DeviceState;
use crate::x11::window::Window;
use std::mem::MaybeUninit;
use std::ptr;
use std::thread;
use std::time::{Duration, Instant};
use x11::xlib;

const SELECT_WINDOW_TIMEOUT: u128 = 30 * 1000;
const SELECTION_INTERVAL: u64 = 10;

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
		mut device_state: DeviceState,
		gc: xlib::GC,
	) -> Option<Window> {
		let mut focused_window = self.get_focused_window();
		let mut selection_canceled = false;
		let now = Instant::now();
		while !(device_state.mouse_clicked || device_state.exit_keys_pressed) {
			focused_window = self.get_focused_window();
			focused_window.draw_borders(gc, 5);
			device_state.update();
			if device_state.exit_keys_pressed
				|| now.elapsed().as_millis() > SELECT_WINDOW_TIMEOUT
			{
				selection_canceled = true;
				break;
			}
			thread::sleep(Duration::from_millis(SELECTION_INTERVAL));
		}
		focused_window.clear_area();
		if !selection_canceled {
			Some(focused_window)
		} else {
			None
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
