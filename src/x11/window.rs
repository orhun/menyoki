use x11::xlib;

#[derive(Debug)]
pub struct Rect {
	pub x: i32,
	pub y: i32,
	pub width: u32,
	pub height: u32,
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
}
