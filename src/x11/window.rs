use x11::xlib;

pub struct Window {
	pub xid: usize,
	pub display: *mut xlib::Display,
}
