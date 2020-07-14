use crate::image::geometry::Geometry;
use crate::record::fps::FpsClock;
use crate::record::settings::{RecordSettings, RecordWindow};
use crate::util::state::InputState;
use crate::x11::window::Window;
use device_query::{DeviceQuery, Keycode};
use std::io::{self, Write};
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
	 * Get the type of Window given with RecordWindow enum.
	 *
	 * @return Window, width and height
	 */
	fn get_window(&self) -> (Window, (u32, u32)) {
		let (window, values) = match self.settings.window {
			RecordWindow::Focus(values) => (self.get_focused_window(), values),
			RecordWindow::Root(values) => (Some(self.get_root_window()), values),
		};
		(
			window.expect("Failed to get the window"),
			values.unwrap_or((0, 0)),
		)
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
			trace!("Ungrabbed the keys of {:?}", xid);
		}
	}

	/**
	 * Select a Window from display with user interaction.
	 *
	 * @param  input_state
	 * @return Window (Option)
	 */
	pub fn select_window(&mut self, input_state: &InputState) -> Option<Window> {
		let (mut window, (width, height)) = self.get_window();
		let mut xid = None;
		let start_time = Instant::now();
		let window_padding = self.settings.padding;
		let mut change_factor = 2;
		while !input_state.check_action_keys() {
			window = self.get_window().0;
			window.draw_borders();
			let reset_area =
				self.update_area(window, input_state, &mut change_factor);
			if input_state.check_cancel_keys() {
				warn!("User interrupt detected.");
				xid = None;
				break;
			} else if start_time.elapsed().as_secs() > self.settings.time.timeout {
				warn!("The operation timed out.");
				xid = None;
				break;
			} else if xid != Some(window.xid) || reset_area {
				if !reset_area {
					debug!("Window ID: {}", window.xid);
					info!("{}", window);
				}
				self.ungrab_keys(xid);
				self.settings.padding = window_padding;
				self.update_padding(width, height, window.geometry);
				window.clear_area();
				window.grab_key(keysym::XK_Alt_L);
				xid = Some(window.xid);
			}
			thread::sleep(Duration::from_millis(self.settings.time.interval));
		}
		trace!("{:?}", input_state);
		debug!("Selected window: {:?}", xid);
		self.ungrab_keys(xid);
		if self.settings.border.is_some() {
			window.clear_area();
			window.show_text(Some(String::from(" ")), FpsClock::new(500));
		}
		if xid.is_some() {
			Some(window)
		} else {
			None
		}
	}

	/**
	 * Update padding to set the given width and height.
	 *
	 * @param width
	 * @param height
	 * @param geometry
	 */
	fn update_padding(&mut self, width: u32, height: u32, geometry: Geometry) {
		if width != 0 && height != 0 {
			self.settings.padding.top = 0;
			self.settings.padding.right = 0;
			self.settings.padding.bottom =
				geometry.height.checked_sub(height).unwrap_or_default();
			self.settings.padding.left =
				geometry.width.checked_sub(width).unwrap_or_default();
		}
	}

	/**
	 * Update the recording area on associated key presses.
	 *
	 * @param  window
	 * @param  input_state
	 * @param  change
	 * @return bool
	 */
	fn update_area(
		&mut self,
		window: Window,
		input_state: &InputState,
		change: &mut u32,
	) -> bool {
		let mut reset_area = false;
		for (value, increase, decrease) in self.settings.padding.get_modifiers() {
			match input_state.state.get_keys().as_slice() {
				[Keycode::LAlt, key] => {
					if key == &increase {
						*value = value.checked_add(*change).unwrap_or(*value);
						window.clear_area();
					}
				}
				[Keycode::LControl, Keycode::LAlt, key] => {
					if key == &decrease {
						*value = value.checked_sub(*change).unwrap_or(*value);
						window.clear_area();
					}
				}
				[Keycode::LShift, Keycode::LAlt, key] => {
					if key == &increase {
						*value = value.checked_add(*change).unwrap_or(*value);
						window.clear_area();
					}
					if key == &decrease {
						*value = value.checked_sub(*change).unwrap_or(*value);
						window.clear_area();
					}
				}
				[Keycode::R, Keycode::LAlt] => {
					reset_area = true;
				}
				[key, Keycode::LAlt] => {
					let key = format!("{:?}", key);
					if key.contains("Key") {
						*change = key
							.trim_start_matches("Key")
							.parse::<u32>()
							.unwrap_or(*change);
					}
				}
				_ => {}
			}
		}
		info!(
			" Selected area -> [{}x{}] {}\r#",
			window.area.width,
			window.area.height,
			if window.settings.padding.is_zero() {
				String::new()
			} else {
				format!("p:[{}]{:<10}", window.settings.padding, " ")
			},
		);
		io::stdout().flush().expect("Failed to flush stdout");
		reset_area
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::image::padding::Padding;
	use crate::record::settings::RecordTime;
	#[test]
	fn test_display_mod() {
		let settings = RecordSettings::new(
			10,
			0x00ff_00ff,
			Some(0),
			false,
			Padding::default(),
			RecordTime::new(Some(0.0), 0, 0, 10),
			RecordWindow::Focus(Some((0, 0))),
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
