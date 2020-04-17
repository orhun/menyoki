use crate::image::{Bgr, Image, Rect};
use std::slice;
use x11::xlib;

pub struct Window {
	pub xid: usize,
	pub display: *mut xlib::Display,
}

impl Window {
	pub fn get_rect(&self) -> Rect {
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
		Rect {
			x,
			y,
			width,
			height,
		}
	}

	pub fn get_image(&self, rect: Rect) -> Option<Image> {
		let window_image = unsafe {
			xlib::XGetImage(
				self.display,
				self.xid as u64,
				rect.x,
				rect.y,
				rect.width,
				rect.height,
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
			Some(Image { rect, data })
		} else {
			None
		}
	}
}
