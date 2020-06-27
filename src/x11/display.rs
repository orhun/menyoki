use crate::record::fps::FpsClock;
use crate::record::settings::RecordSettings;
use crate::util::modifier::ValueModifier;
use crate::util::state::InputState;
use crate::x11::window::Window;
use device_query::Keycode;
use std::convert::TryFrom;
use std::mem::MaybeUninit;
use std::ptr;
use std::thread;
use std::time::{Duration, Instant};
use x11::{keysym, xlib};

/* X11 display */
pub struct Display {
	pub display: *mut xlib::Display,
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
	 * @param  input_state
	 * @return Window (Option)
	 */
	pub fn select_window(&mut self, input_state: &InputState) -> Option<Window> {
		let mut focused_window = self
			.get_focused_window()
			.expect("Failed to get the focused window");
		let mut xid = None;
		let start_time = Instant::now();
		let window_padding = self.settings.padding;
		let padding_change =
			u32::try_from(self.settings.time.interval).unwrap_or_default() / 5;
		while !input_state.check_mouse() && !input_state.check_action_keys() {
			focused_window = self
				.get_focused_window()
				.expect("Failed to get the focused window");
			focused_window.draw_borders();
			self.update_padding(focused_window, input_state, padding_change);
			if input_state.check_cancel_keys() {
				warn!("User interrupt detected.");
				xid = None;
				break;
			} else if start_time.elapsed().as_secs() > self.settings.time.timeout {
				warn!("The operation timed out.");
				xid = None;
				break;
			} else if xid != Some(focused_window.xid) {
				debug!("Window ID: {:?}", focused_window.xid);
				info!("{}", focused_window);
				self.ungrab_keys(xid);
				self.settings.padding = window_padding;
				focused_window.clear_area();
				focused_window.grab_key(keysym::XK_Alt_L);
				xid = Some(focused_window.xid);
			}
			thread::sleep(Duration::from_millis(self.settings.time.interval));
		}
		if self.settings.border.is_some() {
			focused_window.clear_area();
			focused_window.show_text(Some(String::from(" ")), FpsClock::new(500));
		}
		if xid.is_some() {
			Some(focused_window)
		} else {
			None
		}
	}

	/**
	 * Update the window padding with checking the pressed keys.
	 *
	 * @param window
	 * @param input_state
	 * @param change
	 */
	fn update_padding(
		&mut self,
		window: Window,
		input_state: &InputState,
		change: u32,
	) {
		for modifier in ValueModifier::from_padding(&mut self.settings.padding) {
			if input_state.check_key_combination(
				None,
				vec![&Keycode::LAlt, &modifier.increase],
			) {
				*modifier.value =
					modifier.value.checked_add(change).unwrap_or_default();
				window.clear_area();
			} else if input_state.check_key_combination(
				None,
				vec![&Keycode::LAlt, &Keycode::X, &modifier.decrease],
			) {
				*modifier.value =
					modifier.value.checked_sub(change).unwrap_or_default();
				window.clear_area();
			}
		}
	}

	/**
	 * Ungrab the keys in the given window.
	 *
	 * @param xid (Option)
	 */
	fn ungrab_keys(&self, xid: Option<u64>) {
		if let Some(window) = xid {
			unsafe {
				xlib::XUngrabKey(
					self.display,
					xlib::AnyKey,
					xlib::AnyModifier,
					window,
				);
			}
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
			Some(0),
			false,
			Padding::default(),
			RecordTime::new(0, 0, 10),
			RecordWindow::Select,
		);
		let mut display = Display::open(Some(settings)).unwrap();
		display
			.set_focused_window(display.get_root_window().xid, xlib::RevertToParent);
		assert_eq!(
			display.get_root_window().xid,
			display.get_focused_window().unwrap().xid
		);
		assert!(display.select_window(&InputState::new()).is_none());
	}
}
