pub mod display;
pub mod settings;
pub mod window;

use crate::encode::{Capture, Image};
use crate::settings::AppSettings;
use crate::x11::display::Display;
use crate::x11::window::Window;
use std::convert::TryFrom;
use std::ffi::CStr;
use std::os::raw::c_char;
use x11::xlib;

/* X11 window system */
pub struct WindowSystem {
	display: Display,
	settings: AppSettings,
}

impl WindowSystem {
	/**
	 * Initialize the X11 window system.
	 *
	 * @param  settings
	 * @return WindowSystem (Option)
	 */
	pub fn init(settings: AppSettings) -> Option<Self> {
		if let Some(display) = Display::open(None) {
			unsafe { xlib::XSetErrorHandler(Some(x11_error_handler)) };
			Some(Self { display, settings })
		} else {
			None
		}
	}

	/**
	 * Get the window to record.
	 *
	 * @return Window (Option)
	 */
	fn get_record_window(&self) -> Option<Window> {
		if self.settings.args.is_present("root") {
			Some(self.display.get_root_window())
		} else if self.settings.args.is_present("command") {
			Some(
				self.display
					.get_focused_window()
					.expect("Failed to get the focused window"),
			)
		} else {
			self.display.select_window(self.settings.get_color())
		}
	}

	/**
	 * Get the window recording function of the selected window.
	 *
	 * @return Fn (Option)
	 */
	pub fn get_record_func(&mut self) -> Option<impl Fn() -> Option<Image>> {
		if let Some(mut window) = self.get_record_window() {
			window.reset_position();
			Some(move || window.get_image())
		} else {
			None
		}
	}
}

/* Error handler implemention for X11 */
unsafe extern "C" fn x11_error_handler(
	display: *mut xlib::Display,
	error: *mut xlib::XErrorEvent,
) -> i32 {
	let mut error_msg = String::from("X Error");
	let mut error_text: Vec<u8> = Vec::with_capacity(1024);
	if xlib::XGetErrorText(
		display,
		i32::try_from((*error).error_code).unwrap_or_default(),
		error_text.as_mut_ptr() as *mut c_char,
		i32::try_from(error_text.capacity()).unwrap_or_default(),
	) == 0
	{
		error_msg += &format!(
			": {}",
			CStr::from_ptr(error_text.as_mut_ptr() as *mut c_char)
				.to_string_lossy()
				.into_owned()
		);
	}
	error!(
		"{} Opcode: {}, Serial: {}",
		error_msg,
		(*error).request_code,
		(*error).serial
	);
	0
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::util;
	#[test]
	fn test_x11_mod() {
		let settings = AppSettings::new(util::parse_args());
		let mut window_system = WindowSystem::init(settings).unwrap();
		window_system.display.set_focused_window(
			window_system.display.get_root_window().xid,
			xlib::RevertToParent,
		);
		assert_eq!(
			1366 * 768 * 3,
			(window_system.get_record_func().unwrap())()
				.unwrap()
				.data
				.len()
		);
	}
}
