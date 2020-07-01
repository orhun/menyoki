use crate::image::geometry::Geometry;
use crate::image::Image;
use crate::record::fps::FpsClock;
use crate::record::settings::RecordSettings;
use crate::record::Record;
use image::Bgra;
use std::convert::{TryFrom, TryInto};
use std::ffi::CString;
use std::fmt;
use std::mem::MaybeUninit;
use std::ptr;
use std::slice;
use x11::xlib;

/* X11 window id, geometric properties and its display */
#[derive(Clone, Copy, Debug)]
pub struct Window {
	pub xid: u64,
	pub display: *mut xlib::Display,
	pub geometry: Geometry,
	pub settings: RecordSettings,
}

/* Implementations for thread-safe usage */
unsafe impl Sync for Window {}
unsafe impl Send for Window {}

/* Display implementation for user-facing output */
impl fmt::Display for Window {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"\nWindow title    -> \"{}\"\nWindow size     -> [{}x{}]",
			self.get_name().unwrap_or_else(|| String::from("(?)")),
			self.geometry.width,
			self.geometry.height,
		)
	}
}

impl Window {
	/**
	 * Create a new Window object.
	 *
	 * @param  xid
	 * @param  display
	 * @param  settings
	 * @return Window
	 */
	pub fn new(
		xid: u64,
		display: *mut xlib::Display,
		settings: RecordSettings,
	) -> Self {
		unsafe {
			Self {
				xid,
				display,
				geometry: Geometry::default(),
				settings,
			}
			.set_geometry()
		}
	}

	/**
	 * Get the geometric properties of the window.
	 *
	 * @return Geometry
	 */
	unsafe fn get_geometry(&self) -> Geometry {
		let mut root: xlib::Window = 0;
		let (mut x, mut y, mut width, mut height, mut border_width, mut depth) =
			(0, 0, 0, 0, 0, 0);
		xlib::XGetGeometry(
			self.display,
			self.xid,
			&mut root,
			&mut x,
			&mut y,
			&mut width,
			&mut height,
			&mut border_width,
			&mut depth,
		);
		Geometry::new(0, 0, width, height, Some(self.settings.padding))
	}

	/**
	 * Set the geometric properties of the window.
	 *
	 * @return Window
	 */
	unsafe fn set_geometry(&mut self) -> Self {
		self.geometry = self.get_geometry();
		*self
	}

	/**
	 * Get the name of the window.
	 *
	 * @return String (Option)
	 */
	pub fn get_name(&self) -> Option<String> {
		unsafe {
			let mut window_name = MaybeUninit::<*mut i8>::uninit();
			if xlib::XFetchName(self.display, self.xid, window_name.as_mut_ptr())
				!= 0
			{
				Some(
					CString::from_raw(*window_name.as_ptr())
						.into_string()
						.unwrap_or_default(),
				)
			} else {
				None
			}
		}
	}

	/**
	 * Get the graphics context from window.
	 *
	 * @param  fg_color
	 * @return GC
	 */
	fn get_gc(&self, fg_color: u64) -> xlib::GC {
		unsafe {
			let gc = xlib::XCreateGC(self.display, self.xid, 0, ptr::null_mut());
			xlib::XSetForeground(self.display, gc, fg_color);
			gc
		}
	}

	/* Draw a rectangle inside the window. */
	pub fn draw_borders(&self) {
		if let Some(border) = self.settings.border {
			unsafe {
				xlib::XDrawRectangle(
					self.display,
					self.xid,
					self.get_gc(self.settings.color),
					self.geometry.x + i32::try_from(border).unwrap_or_default(),
					self.geometry.y + i32::try_from(border).unwrap_or_default(),
					self.geometry.width - (border * 2),
					self.geometry.height - (border * 2),
				);
			}
		}
	}

	/**
	 * Draw a text on the window.
	 *
	 * @param text
	 * @param x
	 * @param y
	 */
	fn draw_text(&self, text: &str, x: i32, y: i32) {
		unsafe {
			xlib::XDrawString(
				self.display,
				self.xid,
				self.get_gc(self.settings.color),
				x,
				y,
				CString::new(text).unwrap_or_default().as_ptr(),
				text.len().try_into().unwrap_or_default(),
			);
		}
	}

	/**
	 * Show a text on the window for a given duration.
	 *
	 * @param  text (Option)
	 * @param  clock
	 */
	pub fn show_text(&self, text: Option<String>, mut clock: FpsClock) {
		let text = text.unwrap_or_default();
		for _ in 0..clock.fps {
			self.draw_text(
				text.as_str(),
				self.geometry.x
					+ (self.geometry.width - 25).try_into().unwrap_or(20),
				self.geometry.y + 20,
			);
			clock.tick();
		}
	}

	/* Clear the area of the window and regenerate the Expose event. */
	pub fn clear_area(&self) {
		unsafe {
			xlib::XClearArea(
				self.display,
				self.xid,
				self.geometry.x,
				self.geometry.y,
				self.geometry.width,
				self.geometry.height,
				xlib::True,
			);
		}
	}

	/**
	 * Grab a key in the window.
	 *
	 * @param key
	 */
	pub fn grab_key(&self, key: u32) {
		unsafe {
			xlib::XGrabKey(
				self.display,
				xlib::XKeysymToKeycode(self.display, key.into())
					.try_into()
					.expect("Failed to get the keycode"),
				xlib::AnyModifier,
				self.xid,
				xlib::False,
				xlib::GrabModeAsync,
				xlib::GrabModeAsync,
			);
		}
	}
}

/* Record implementation for X11 Window */
impl Record for Window {
	/**
	 * Get the image of the window.
	 *
	 * @return Image (Option)
	 */
	fn get_image(&self) -> Option<Image> {
		unsafe {
			let window_image = xlib::XGetImage(
				self.display,
				self.xid,
				self.geometry.x,
				self.geometry.y,
				self.geometry.width,
				self.geometry.height,
				xlib::XAllPlanes(),
				xlib::ZPixmap,
			);
			if !window_image.is_null() {
				let image = &mut *window_image;
				let data = slice::from_raw_parts::<Bgra<u8>>(
					image.data as *const Bgra<u8>,
					image.width as usize * image.height as usize,
				)
				.to_vec();
				xlib::XDestroyImage(window_image);
				Some(Image::new(data, self.settings.alpha, self.geometry))
			} else {
				None
			}
		}
	}

	/* Show a countdown on the corner of window. */
	fn show_countdown(&self) {
		if self.settings.time.countdown != 0 {
			let clock = FpsClock::new(1000);
			for i in 0..(self.settings.time.countdown + 1) {
				self.clear_area();
				self.show_text(
					if i != self.settings.time.countdown {
						Some(format!("[{}]", self.settings.time.countdown - i))
					} else {
						None
					},
					clock,
				);
			}
		}
		self.clear_area();
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::image::padding::Padding;
	use crate::record::settings::{RecordTime, RecordWindow};
	use crate::x11::display::Display;
	use image::ColorType;
	#[test]
	fn test_window_mod() {
		let settings = RecordSettings::new(
			10,
			0x00ff_00ff,
			Some(0),
			false,
			Padding::default(),
			RecordTime::new(0.0, 1, 0, 10),
			RecordWindow::Select,
		);
		let display = Display::open(Some(settings)).unwrap();
		let window = display.get_root_window();
		unsafe {
			xlib::XStoreName(
				window.display,
				window.xid,
				CString::new("root-window").unwrap_or_default().as_ptr(),
			);
		};
		window.draw_borders();
		window.show_countdown();
		window.clear_area();
		assert_eq!("1366x768  (root-window)", format!("{}", window));
		assert_eq!((0, 0), (window.geometry.x, window.geometry.y));
		assert_eq!("root-window", window.get_name().unwrap());
		assert_eq!(
			1366 * 768 * 3,
			window.get_image().unwrap().get_data(ColorType::Rgb8).len()
		);
	}
}
