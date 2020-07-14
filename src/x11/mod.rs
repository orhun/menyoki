pub mod display;
pub mod window;

use crate::app::WindowAccess;
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

impl<'a> WindowAccess<'a, Window> for WindowSystem<'a> {
	/**
	 * Initialize the X11 window system.
	 *
	 * @param  settings
	 * @return WindowSystem (Option)
	 */
	fn init(settings: &'a AppSettings<'a>) -> Option<Self> {
		trace!("{:?}", settings);
		if let Some(display) = Display::open(Some(settings.record)) {
			unsafe { xlib::XSetErrorHandler(Some(handle_x11_errors)) };
			Some(Self { display, settings })
		} else {
			error!("Cannot open display.");
			None
		}
	}

	/**
	 * Get the window to record.
	 *
	 * @return Window (Option)
	 */
	fn get_window(&mut self) -> Option<Window> {
		debug!("Record window: {:?}", self.settings.record.window);
		match self.settings.record.window {
			RecordWindow::Focus(None) => self.display.get_focused_window(),
			RecordWindow::Root(None) => Some(self.display.get_root_window()),
			_ => {
				if self.settings.args.is_present("command") {
					self.display.get_focused_window()
				} else {
					self.display.select_window(&self.settings.input_state)
				}
			}
		}
	}
}

/* Error handler implemention for X11 */
unsafe extern "C" fn handle_x11_errors(
	display: *mut xlib::Display,
	error: *mut xlib::XErrorEvent,
) -> i32 {
	let mut error_text = Vec::with_capacity(1024);
	error!(
		"{}[Opcode: {}, Serial: {}]",
		if xlib::XGetErrorText(
			display,
			(*error).error_code.try_into().unwrap_or_default(),
			error_text.as_mut_ptr() as *mut c_char,
			error_text.capacity().try_into().unwrap_or_default(),
		) == 0
		{
			format!(
				"{} ",
				CStr::from_ptr(error_text.as_mut_ptr() as *mut c_char)
					.to_string_lossy()
					.into_owned()
			)
		} else {
			String::from("Unknown X Error")
		},
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
	use image::ColorType;
	#[test]
	fn test_x11_mod() {
		let args = Args::parse();
		let settings = AppSettings::new(&args);
		let mut window_system = WindowSystem::init(&settings).unwrap();
		window_system.display.set_focused_window(
			window_system.display.get_root_window().xid,
			xlib::RevertToParent,
		);
		assert_eq!(
			1366 * 768 * 3,
			window_system
				.get_window()
				.unwrap()
				.get_image()
				.unwrap()
				.get_data(ColorType::Rgb8)
				.len()
		);
		unsafe {
			handle_x11_errors(
				window_system.display.display,
				&mut xlib::XErrorEvent {
					type_: 0,
					display: window_system.display.display,
					resourceid: 0,
					serial: 0,
					error_code: 0,
					request_code: 0,
					minor_code: 0,
				},
			);
		}
	}
}
