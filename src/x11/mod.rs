pub mod display;
pub mod window;

use crate::app::AppSettings;
use crate::image::{Capture, Image};
use crate::x11::display::Display;
use std::ffi::CStr;
use std::os::raw::c_char;
use x11::xlib;

pub struct WindowSystem {
	display: Display,
}

impl WindowSystem {
	pub fn init() -> Option<Self> {
		if let Some(display) = Display::open() {
			unsafe { xlib::XSetErrorHandler(Some(x11_error_handler)) };
			Some(Self { display })
		} else {
			None
		}
	}

	pub fn get_record_func(
		&mut self,
		settings: AppSettings,
	) -> impl Fn() -> Option<Image> {
		let mut focused_window = self.display.get_focused_window();
		if !settings.args.is_present("command") {
			focused_window = self
				.display
				.select_window(settings.get_color())
				.unwrap();
		}
		focused_window.reset_position();
		move || focused_window.get_image()
	}
}

unsafe extern "C" fn x11_error_handler(
	display: *mut xlib::Display,
	error: *mut xlib::XErrorEvent,
) -> i32 {
	let mut error_msg = String::from("X Error");
	let mut error_text: Vec<u8> = Vec::with_capacity(1024);
	if xlib::XGetErrorText(
		display,
		(*error).error_code as i32,
		error_text.as_mut_ptr() as *mut c_char,
		error_text.capacity() as i32,
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
