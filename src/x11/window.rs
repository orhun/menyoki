use crate::image::{Bgr, Geometry, Image, ImageHandler};
use std::slice;
use x11::xlib;

/* X11 window id, geometric properties and its display */
#[derive(Clone, Copy, Debug)]
pub struct Window {
	pub xid: usize,
	pub display: *mut xlib::Display,
	pub geometry: Geometry,
}

impl Window {
	/**
	 * Create a new Window object.
	 *
	 * @param  xid
	 * @param  display
	 * @return Window
	 */
	pub fn new(xid: usize, display: *mut xlib::Display) -> Self {
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
				self.xid as u64,
				&mut root,
				&mut x,
				&mut y,
				&mut width,
				&mut height,
				&mut border_width,
				&mut depth,
			);
		}
		Geometry {
			x,
			y,
			width,
			height,
		}
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
}

impl ImageHandler for Window {
	/**
	 * Get the image of the window.
	 *
	 * @return Image (Option)
	 */
	fn get_image(&self) -> Option<Image> {
		let window_image = unsafe {
			xlib::XGetImage(
				self.display,
				self.xid as u64,
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
			Some(Image {
				geometry: self.geometry,
				data,
			})
		} else {
			None
		}
	}
}
