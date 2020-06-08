use crate::record::settings::RecordSettings;
use crate::util::device::DeviceState;
use crate::x11::window::Window;
use std::mem::MaybeUninit;
use std::ptr;
use std::thread;
use std::time::{Duration, Instant};
use x11::xlib;

/* X11 display */
pub struct Display {
	display: *mut xlib::Display,
	settings: RecordSettings,
}

/* Implementation for thread-safe usage */
unsafe impl Send for Display {}

impl Display {
	/**
	 * Open a display.
	 *
	 * @param  settings (Option)
	 * @return Display  (Option)
	 */
	pub fn open(settings: Option<RecordSettings>) -> Option<Self> {
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
				self.settings,
			)
		}
	}

	/**
	 * Get the focused window.
	 *
	 * @return Window (Option)
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
				Some(Window::new(
					*focus_window.as_ptr(),
					self.display,
					self.settings,
				))
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
	 * Select a Window from display with user interaction.
	 *
	 * @return Window (Option)
	 */
	pub fn select_window(&self) -> Option<Window> {
		let mut device_state = DeviceState::new();
		let mut focused_window = self
			.get_focused_window()
			.expect("Failed to get the focused window");
		let mut xid = None;
		let now = Instant::now();
		while !device_state.check_mouse_clicked() {
			focused_window = self
				.get_focused_window()
				.expect("Failed to get the focused window");
			focused_window.draw_borders();
			if device_state.check_cancel_pressed() {
				warn!("User interrupt detected.");
				xid = None;
				break;
			} else if now.elapsed().as_secs() > self.settings.time.timeout {
				warn!("The operation timed out.");
				xid = None;
				break;
			} else if xid != Some(focused_window.xid) {
				debug!("Window ID: {:?}", focused_window.xid);
				info!("{}", focused_window);
				xid = Some(focused_window.xid);
			}
			thread::sleep(Duration::from_millis(self.settings.time.interval));
		}
		focused_window.clear_area();
		if xid.is_some() {
			Some(focused_window)
		} else {
			None
		}
	}
}

/* Close the display when Display object goes out of scope */
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
	use crate::image::padding::Padding;
	use crate::record::settings::{RecordTime, RecordWindow};
	#[test]
	fn test_display_mod() {
		let settings = RecordSettings::new(
			10,
			0x00ff_00ff,
			0,
			Padding::default(),
			RecordTime::new(0, 10, 0),
			RecordWindow::Select,
		);
		let display = Display::open(Some(settings)).unwrap();
		display
			.set_focused_window(display.get_root_window().xid, xlib::RevertToParent);
		assert_eq!(
			display.get_root_window().xid,
			display.get_focused_window().unwrap().xid
		);
		assert!(display.select_window().is_none());
	}
}
