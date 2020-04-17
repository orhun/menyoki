use gif::Frame;
use std::slice::{self, Iter};
use x11::xlib;

#[derive(Debug)]
struct Bgr {
	b: u8,
	g: u8,
	r: u8,
	_p: u8,
}

impl Bgr {
	fn get_rgb_pixels(bgr_data: Iter<'static, Bgr>) -> Vec<u8> {
		let mut pixels = Vec::new();
		for bgr in bgr_data {
			pixels.extend(&[bgr.r, bgr.g, bgr.b])
		}
		pixels
	}
}

#[derive(Debug)]
pub struct Rect {
	pub x: i32,
	pub y: i32,
	pub width: u32,
	pub height: u32,
}

impl Rect {
	pub fn origin(rect: Self) -> Self {
		Rect {
			x: 0,
			y: 0,
			width: rect.width,
			height: rect.height,
		}
	}
}

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

	pub fn get_image(&self, rect: Rect) -> Option<Frame> {
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
			let frame = gif::Frame::from_rgb_speed(
				rect.width as u16,
				rect.height as u16,
				&Bgr::get_rgb_pixels(
					unsafe {
						slice::from_raw_parts::<Bgr>(
							image.data as *const _,
							image.width as usize * image.height as usize,
						)
					}
					.iter(),
				),
				30,
			);
			unsafe {
				xlib::XDestroyImage(window_image as *mut _);
			};
			Some(frame)
		} else {
			None
		}
	}
}
