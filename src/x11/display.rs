use crate::util::device::DeviceState;
use crate::x11::window::Window;
use std::mem::MaybeUninit;
use std::ptr;
use std::thread;
use std::time::{Duration, Instant};
use x11::xlib;

/* Timeout value for the window selection */
const SELECT_WINDOW_TIMEOUT: u128 = 30 * 1000;
/* Time interval between focused window checks */
const SELECTION_INTERVAL: u64 = 10;

/* Display settings regarding to window selection */
pub struct DisplaySettings {
	timeout: u128,
	interval: u64,
}

impl DisplaySettings {
	/**
	 * Create a new DisplaySettings object.
	 *
	 * @param  timeout
	 * @param  interval
	 * @return DisplaySettings
	 */
	#[allow(dead_code)]
	fn new(timeout: u128, interval: u64) -> Self {
		Self { timeout, interval }
	}
}

/* Default state of the display settings */
impl Default for DisplaySettings {
	fn default() -> Self {
		Self {
			timeout: SELECT_WINDOW_TIMEOUT,
			interval: SELECTION_INTERVAL,
		}
	}
}

/* X11 display */
pub struct Display {
	display: *mut xlib::Display,
	settings: DisplaySettings,
}

/* Implementation for thread-safe usage */
unsafe impl Send for Display {}

impl Display {
	/**
	 * Open a display.
	 *
	 * @param  settings (Option)
	 * @return Display (Option)
	 */
	pub fn open(settings: Option<DisplaySettings>) -> Option<Self> {
		let display = unsafe { xlib::XOpenDisplay(ptr::null()) };
		if !display.is_null() {
			Some(Self {
				display,
				settings: settings.unwrap_or_default(),
			})
		} else {
			None
		}
	}

	/**
	 * Get the default screen of display.
	 *
	 * @return Screen
	 */
	unsafe fn get_default_screen(&self) -> *mut xlib::Screen {
		xlib::XDefaultScreenOfDisplay(self.display)
	}

	/**
	 * Get the root window of the default screen.
	 *
	 * @return Window
	 */
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
	pub fn get_focused_window(&self) -> Option<Window> {
		unsafe {
			let mut focus_window = MaybeUninit::<u64>::uninit();
			let mut focus_state = MaybeUninit::<i32>::uninit();
			xlib::XGetInputFocus(
				self.display,
				focus_window.as_mut_ptr(),
				focus_state.as_mut_ptr(),
			);
			if focus_state.assume_init() != xlib::RevertToNone {
				Some(Window::new(*focus_window.as_ptr(), self.display))
			} else {
				None
			}
		}
	}

	/**
	 * Set the focused window.
	 *
	 * @param  xid
	 * @param  focus_state
	 */
	#[allow(dead_code)]
	pub fn set_focused_window(&self, xid: u64, focus_state: i32) {
		unsafe {
			xlib::XSetInputFocus(self.display, xid, focus_state, xlib::CurrentTime)
		};
	}

	/**
	 * Select a Window from the display with the user interaction.
	 *
	 * @param  fg_color
	 * @return Window (Option)
	 */
	pub fn select_window(&self, fg_color: u64) -> Option<Window> {
		let mut device_state = DeviceState::new();
		let mut focused_window = self
			.get_focused_window()
			.expect("Failed to get the focused window");
		let mut xid = 0;
		let mut selection_canceled = false;
		let now = Instant::now();
		while !(device_state.mouse_clicked
			|| device_state.exit_keys_pressed
			|| selection_canceled)
		{
			focused_window = self
				.get_focused_window()
				.expect("Failed to get the focused window");
			focused_window.draw_borders(fg_color, 5);
			device_state.update();
			if device_state.exit_keys_pressed {
				warn!("User interrupt detected. Have a good day!");
				selection_canceled = true;
			} else if now.elapsed().as_millis() > self.settings.timeout {
				warn!("The operation timed out. Have a good day!");
				selection_canceled = true;
			} else if xid != focused_window.xid {
				debug!("Window ID: {:?}", focused_window.xid);
				info!("{}", focused_window);
				xid = focused_window.xid;
			}
			thread::sleep(Duration::from_millis(self.settings.interval));
		}
		focused_window.clear_area();
		if !selection_canceled {
			Some(focused_window)
		} else {
			None
		}
	}
}

/* Close the display when Display object went out of scope. */
impl Drop for Display {
	fn drop(&mut self) {
		unsafe {
			xlib::XCloseDisplay(self.display);
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn test_display_mod() {
		let display = Display::open(Some(DisplaySettings::new(20, 10))).unwrap();
		display
			.set_focused_window(display.get_root_window().xid, xlib::RevertToParent);
		assert_eq!(
			display.get_root_window().xid,
			display.get_focused_window().unwrap().xid
		);
		assert!(display.select_window(0x00ff_00ff).is_none());
		assert_eq!(SELECT_WINDOW_TIMEOUT, DisplaySettings::default().timeout);
		assert_eq!(SELECTION_INTERVAL, DisplaySettings::default().interval);
	}
}
