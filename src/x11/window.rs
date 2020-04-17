use crate::image::{Bgr, Image, Rect};
use std::slice;
use x11::xlib;

#[derive(Clone, Copy, Debug)]
pub struct Window {
	pub xid: usize,
	pub display: *mut xlib::Display,
	pub rect: Rect,
}

impl Window {
	pub fn new(xid: usize, display: *mut xlib::Display) -> Self {
		Self {
			xid,
			display,
			rect: Rect::default(),
		}
		.set_rect()
	}

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

	fn set_rect(&mut self) -> Self {
		self.rect = self.get_rect();
		*self
	}

	pub fn get_image(&self) -> Option<Image> {
		let window_image = unsafe {
			xlib::XGetImage(
				self.display,
				self.xid as u64,
				self.rect.x,
				self.rect.y,
				self.rect.width,
				self.rect.height,
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
				rect: self.rect,
				data,
			})
		} else {
			None
		}
	}
}
