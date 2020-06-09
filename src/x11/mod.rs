pub mod display;
pub mod window;

use crate::record::settings::RecordWindow;
use crate::settings::AppSettings;
use crate::x11::display::Display;
use crate::x11::window::Window;
use std::convert::TryInto;
use std::ffi::CStr;
use std::os::raw::c_char;
use x11::xlib;

/* X11 window system */
pub struct WindowSystem<'a> {
	display: Display,
	settings: &'a AppSettings<'a>,
}

impl<'a> WindowSystem<'a> {
	/**
	 * Initialize the X11 window system.
	 *
	 * @param  settings
	 * @return WindowSystem (Option)
	 */
	pub fn init(settings: &'a AppSettings<'a>) -> Option<Self> {
		if let Some(display) = Display::open(Some(settings.record)) {
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
	pub fn get_record_window(&self) -> Option<Window> {
		match self.settings.record.window {
			RecordWindow::Focus => self.display.get_focused_window(),
			RecordWindow::Root => Some(self.display.get_root_window()),
			RecordWindow::Select => {
				if self.settings.args.is_present("command") {
					self.display.get_focused_window()
				} else {
					self.display.select_window(&self.settings.device_state)
				}
			}
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
		(*error).error_code.try_into().unwrap_or_default(),
		error_text.as_mut_ptr() as *mut c_char,
		error_text.capacity().try_into().unwrap_or_default(),
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
	use crate::args::Args;
	use crate::record::Record;
	#[test]
	fn test_x11_mod() {
		let args = Args::parse();
		let settings = AppSettings::new(&args);
		let window_system = WindowSystem::init(&settings).unwrap();
		window_system.display.set_focused_window(
			window_system.display.get_root_window().xid,
			xlib::RevertToParent,
		);
		assert_eq!(
			1366 * 768 * 3,
			window_system
				.get_record_window()
				.unwrap()
				.get_image()
				.unwrap()
				.data
				.len()
		);
	}
}
