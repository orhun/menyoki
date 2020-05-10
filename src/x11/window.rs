use crate::image::{Bgr, Capture, Geometry, Image};
use std::ffi::{c_void, CString};
use std::mem;
use std::ptr;
use std::slice;
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

impl Window {
	/**
	 * Create a new Window object.
	 *
	 * @param  xid
	 * @param  display
	 * @return Window
	 */
	pub fn new(xid: u64, display: *mut xlib::Display) -> Self {
		Self {
			xid,
			display,
			geometry: Geometry::default(),
		}
		.set_geometry()
	}

	/**
	 * Get the geometric properties of the window.
	 *
	 * @return Geometry
	 */
	fn get_geometry(&self) -> Geometry {
		let mut root: xlib::Window = 0;
		let (mut x, mut y, mut width, mut height, mut border_width, mut depth) =
			(0, 0, 0, 0, 0, 0);
		unsafe {
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
		}
		Geometry::new(x, y, width, height)
	}

	/**
	 * Set the geometric properties of the window.
	 *
	 * @return Window
	 */
	fn set_geometry(&mut self) -> Self {
		self.geometry = self.get_geometry();
		*self
	}

	/* Set (x, y) of the window geometry to (0, 0) */
	pub fn reset_position(&mut self) {
		self.geometry.x = 0;
		self.geometry.y = 0;
	}

	pub fn create_gc(&self, color: u64) -> xlib::GC {
		unsafe {
			let gc = xlib::XCreateGC(self.display, self.xid, 0, ptr::null_mut());
			xlib::XSetForeground(self.display, gc, color);
			gc
		}
	}

	pub fn get_window_list(&self) -> Option<Vec<Window>> {
		unsafe {
			let mut root_window = mem::MaybeUninit::uninit().assume_init();
			let mut parent_window = mem::MaybeUninit::uninit().assume_init();
			let mut children_return = mem::MaybeUninit::uninit().assume_init();
			let mut children_num = 0;
			if xlib::XQueryTree(
				self.display,
				self.xid,
				&mut root_window,
				&mut parent_window,
				&mut children_return,
				&mut children_num,
			) == 0
			{
				return None;
			}
			let mut window_list = Vec::new();
			for i in 0..children_num {
				window_list.push(Window::new(
					*children_return.offset(i as isize),
					self.display,
				));
			}
			xlib::XFree(children_return as *mut c_void);
			Some(window_list)
		}
	}

	pub fn get_window_name(&self) -> Option<String> {
		unsafe {
			let mut window_name = mem::MaybeUninit::uninit().assume_init();
			if xlib::XFetchName(self.display, self.xid, &mut window_name) != 0 {
				Some(
					CString::from_raw(window_name)
						.into_string()
						.unwrap_or_default(),
				)
			} else {
				None
			}
		}
	}

	pub fn draw_borders(&self, gc: xlib::GC, padding: u32) {
		unsafe {
			xlib::XDrawRectangle(
				self.display,
				self.xid,
				gc,
				self.geometry.x + padding as i32,
				self.geometry.y + padding as i32,
				self.geometry.width - (padding * 2),
				self.geometry.height - (padding * 2),
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
		let window_image = unsafe {
			xlib::XGetImage(
				self.display,
				self.xid,
				self.geometry.x,
				self.geometry.y,
				self.geometry.width,
				self.geometry.height,
				xlib::XAllPlanes(),
				xlib::ZPixmap,
			)
		};
		if !window_image.is_null() {
			let image = unsafe { &mut *window_image };
			let data = Bgr::get_rgb_pixels(unsafe {
				slice::from_raw_parts::<Bgr>(
					image.data as *const _,
					image.width as usize * image.height as usize,
				)
			});
			unsafe {
				xlib::XDestroyImage(window_image as *mut _);
			};
			Some(Image::new(data, self.geometry))
		} else {
			None
		}
	}
}
