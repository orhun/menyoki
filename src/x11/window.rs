use crate::encode::{Bgr, Capture, Geometry, Image};
use std::convert::TryFrom;
use std::ffi::CString;
use std::fmt;
use std::mem::MaybeUninit;
use std::ptr;
use std::slice;
use std::thread;
use std::time::Duration;
use x11::xlib;

/* X11 window id, geometric properties and its display */
#[derive(Clone, Copy, Debug)]
pub struct Window {
	pub xid: u64,
	pub display: *mut xlib::Display,
	pub geometry: Geometry,
}

/* Implementations for thread-safe usage */
unsafe impl Sync for Window {}
unsafe impl Send for Window {}

/* Display implementation for user-facing output */
impl fmt::Display for Window {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"{:>4}x{:<4} ({})",
			self.geometry.width,
			self.geometry.height,
			self.get_name().unwrap_or_default()
		)
	}
}

impl Window {
	/**
	 * Create a new Window object.
	 *
	 * @param  xid
	 * @param  display
	 * @return Window
	 */
	pub fn new(xid: u64, display: *mut xlib::Display) -> Self {
		unsafe {
			Self {
				xid,
				display,
				geometry: Geometry::default(),
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
		Geometry::new(x, y, width, height)
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

	/* Set (x, y) of the window geometry to (0, 0) */
	pub fn reset_position(&mut self) {
		self.geometry.x = 0;
		self.geometry.y = 0;
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

	/**
	 * Draw a rectangle inside the window.
	 *
	 * @param fg_color
	 * @param padding
	 */
	pub fn draw_borders(&self, fg_color: u64, padding: u32) {
		unsafe {
			xlib::XDrawRectangle(
				self.display,
				self.xid,
				self.get_gc(fg_color),
				self.geometry.x + i32::try_from(padding).unwrap_or_default(),
				self.geometry.y + i32::try_from(padding).unwrap_or_default(),
				self.geometry.width - (padding * 2),
				self.geometry.height - (padding * 2),
			);
		}
	}

	/**
	 * Draw a text on the window.
	 *
	 * @param text
	 * @param fg_color
	 * @param x
	 * @param y
	 */
	fn draw_string(&self, text: &str, fg_color: u64, x: i32, y: i32) {
		unsafe {
			xlib::XDrawString(
				self.display,
				self.xid,
				self.get_gc(fg_color),
				x,
				y,
				CString::new(text).unwrap_or_default().as_ptr(),
				i32::try_from(text.len()).unwrap_or_default(),
			);
		}
	}

	pub fn show_countdown(&self, count: u64, fg_color: u64) {
		for i in 0..count {
			self.clear_area();
			for _ in 0..1000 {
				self.draw_string(
					&format!("[{}]", count - i),
					fg_color,
					i32::try_from(self.geometry.width - 25).unwrap_or(20),
					20,
				);
				thread::sleep(Duration::from_millis(1));
			}
		}
		self.clear_area();
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
}

impl Capture for Window {
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
				let data = Bgr::get_rgb_pixels(slice::from_raw_parts::<Bgr>(
					image.data as *const Bgr,
					image.width as usize * image.height as usize,
				));
				xlib::XDestroyImage(window_image);
				Some(Image::new(data, self.geometry))
			} else {
				None
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::x11::display::Display;
	#[test]
	fn test_window_mod() {
		let display = Display::open(None).unwrap();
		let mut window = display.get_root_window();
		window.reset_position();
		unsafe {
			xlib::XStoreName(
				window.display,
				window.xid,
				CString::new("root-window").unwrap_or_default().as_ptr(),
			);
		};
		window.draw_borders(0x00ff_0ff, 0);
		window.clear_area();
		assert_eq!("1366x768  (root-window)", format!("{}", window));
		assert_eq!((0, 0), (window.geometry.x, window.geometry.y));
		assert_eq!("root-window", window.get_name().unwrap());
		assert_eq!(1366 * 768 * 3, window.get_image().unwrap().data.len());
	}
}
