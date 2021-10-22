use crate::image::geometry::Geometry;
use crate::image::Image;
use crate::record::fps::FpsClock;
use crate::window::Capture;
use crate::x11::display::Display;
use image::Bgra;
use std::ffi::CString;
use std::fmt;
use std::io::{self, Write};
use std::mem::MaybeUninit;
use std::os::raw::{c_char, c_uint, c_ulong};
use std::ptr;
use std::slice;
use x11::{xlib, xrandr};

/* Maximum height of the text to show on window */
const MAX_TEXT_HEIGHT: u32 = 40;
/* Offset for placing the text on the corner of window */
const TEXT_CORNER_OFFSET: i32 = 20;
/* Padding value to apply to window borders */
const BORDER_PADDING: u32 = 1;

/* X11 window id, geometric properties and its display */
#[derive(Clone, Copy, Debug)]
pub struct Window {
	pub xid: c_ulong,
	display: Display,
	gc: xlib::GC,
	pub geometry: Geometry,
	pub area: Geometry,
}

/* Implementations for thread-safe usage */
unsafe impl Sync for Window {}
unsafe impl Send for Window {}

/* Display implementation for user-facing output */
impl fmt::Display for Window {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"\n Window title  -> \"{}\"\n Window size   -> [{}x{}]",
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
	 * @return Window
	 */
	pub fn new(xid: c_ulong, display: Display) -> Self {
		unsafe {
			Self {
				xid,
				display,
				gc: ptr::null::<xlib::GC>() as xlib::GC,
				geometry: Geometry::default(),
				area: Geometry::default(),
			}
			.set_geometry()
			.set_gc()
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
			self.display.inner,
			self.xid,
			&mut root,
			&mut x,
			&mut y,
			&mut width,
			&mut height,
			&mut border_width,
			&mut depth,
		);
		Geometry::new(0, 0, width, height)
	}

	/**
	 * Set the geometric properties of the window.
	 *
	 * @return Window
	 */
	unsafe fn set_geometry(&mut self) -> Self {
		let mut geometry = self.get_geometry();
		self.geometry = geometry;
		self.area = geometry.with_padding(self.display.settings.padding);
		*self
	}

	/**
	 * Get the graphics context from window.
	 *
	 * @return GC
	 */
	unsafe fn get_gc(&self) -> xlib::GC {
		let gc = xlib::XCreateGC(self.display.inner, self.xid, 0, ptr::null_mut());
		xlib::XSetForeground(
			self.display.inner,
			gc,
			self.display.settings.color as c_ulong,
		);
		xlib::XSetLineAttributes(
			self.display.inner,
			gc,
			self.display.settings.border.unwrap_or(1),
			xlib::LineSolid,
			xlib::CapNotLast,
			xlib::JoinMiter,
		);
		if let Some(xfont) = self.display.font {
			xlib::XSetFont(self.display.inner, gc, (*xfont).fid);
		}
		gc
	}

	/**
	 * Set the graphics context of Window object.
	 *
	 * @return Window
	 */
	unsafe fn set_gc(&mut self) -> Self {
		self.gc = self.get_gc();
		*self
	}

	/**
	 * Get the parent window.
	 *
	 * @return Window (Option)
	 */
	pub unsafe fn get_parent(&self) -> Option<Self> {
		let mut root = MaybeUninit::<c_ulong>::uninit();
		let mut parent = MaybeUninit::<c_ulong>::uninit();
		let mut children = MaybeUninit::<*mut c_ulong>::uninit();
		let mut nchildren = MaybeUninit::<c_uint>::uninit();
		if xlib::XQueryTree(
			self.display.inner,
			self.xid,
			root.as_mut_ptr(),
			parent.as_mut_ptr(),
			children.as_mut_ptr(),
			nchildren.as_mut_ptr(),
		) == xlib::True
		{
			Some(Window::new(*parent.as_ptr(), self.display))
		} else {
			None
		}
	}

	/**
	 * Get CRTC (VDC) information of the window.
	 *
	 * @param Vector of Geometry
	 */
	pub fn get_crtc_info(&self) -> Vec<Geometry> {
		let mut crtc_info = Vec::new();
		unsafe {
			let resources =
				xrandr::XRRGetScreenResources(self.display.inner, self.xid);
			for crtc in slice::from_raw_parts(
				(*resources).crtcs,
				(*resources).ncrtc.try_into().unwrap_or_default(),
			)
			.iter()
			.map(|v| xrandr::XRRGetCrtcInfo(self.display.inner, resources, *v))
			{
				if (*crtc).noutput > 0 {
					crtc_info.push(Geometry::new(
						(*crtc).x as i32,
						(*crtc).y as i32,
						(*crtc).width as u32,
						(*crtc).height as u32,
					));
				}
				xrandr::XRRFreeCrtcInfo(crtc);
			}
			xrandr::XRRFreeScreenResources(resources);
		}
		crtc_info
	}

	/**
	 * Get the name of the window.
	 *
	 * @return String (Option)
	 */
	pub fn get_name(&self) -> Option<String> {
		unsafe {
			let mut window_name = MaybeUninit::<*mut c_char>::uninit();
			if xlib::XFetchName(
				self.display.inner,
				self.xid,
				window_name.as_mut_ptr(),
			) != 0
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

	/* Draw a rectangle inside the window. */
	pub fn draw_borders(&self) {
		if self.display.settings.border.is_some() {
			unsafe {
				xlib::XDrawRectangle(
					self.display.inner,
					self.xid,
					self.gc,
					self.area
						.x
						.checked_add(
							i32::try_from(BORDER_PADDING).unwrap_or_default(),
						)
						.unwrap_or(self.area.x),
					self.area
						.y
						.checked_add(
							i32::try_from(BORDER_PADDING).unwrap_or_default(),
						)
						.unwrap_or(self.area.y),
					self.area
						.width
						.checked_sub(BORDER_PADDING * 2)
						.unwrap_or(self.area.width),
					self.area
						.height
						.checked_sub(BORDER_PADDING * 2)
						.unwrap_or(self.area.height),
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
				self.display.inner,
				self.xid,
				self.get_gc(),
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
				self.area.x
					+ (self.area.width
						- (u32::try_from(TEXT_CORNER_OFFSET).unwrap_or_default()
							+ 5))
						.try_into()
						.unwrap_or(TEXT_CORNER_OFFSET),
				self.area.y + TEXT_CORNER_OFFSET,
			);
			clock.tick();
		}
	}

	/**
	 * Show a text on the center of the window.
	 *
	 * @param text (Option)
	 * @param font
	 */
	pub fn show_text_centered(
		&self,
		text: Option<String>,
		font: *mut xlib::XFontStruct,
	) {
		let area_text = CString::new(self.area.to_string()).unwrap_or_default();
		let text_width = unsafe {
			xlib::XTextWidth(
				font,
				area_text.as_ptr(),
				area_text.as_bytes().len().try_into().unwrap_or_default(),
			) as u64
		};
		if u64::from(self.area.width) > text_width + 10
			&& self.area.height > MAX_TEXT_HEIGHT
		{
			self.draw_text(
				text.as_deref().unwrap_or_default(),
				self.area.x + i32::try_from(self.area.width / 2).unwrap_or_default()
					- i32::try_from(text_width / 2).unwrap_or_default(),
				self.area.y
					+ i32::try_from(self.area.height / 2).unwrap_or_default(),
			)
		}
	}

	/* Clear the whole window and regenerate the Expose event. */
	pub fn clear_area(&self) {
		unsafe {
			xlib::XClearArea(
				self.display.inner,
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
	pub fn grab_key(&self, key: c_ulong) {
		unsafe {
			xlib::XGrabKey(
				self.display.inner,
				xlib::XKeysymToKeycode(self.display.inner, key).into(),
				xlib::AnyModifier,
				self.xid,
				xlib::False,
				xlib::GrabModeAsync,
				xlib::GrabModeAsync,
			);
		}
		trace!("Grabbed the key {} of {:?}", key, self.xid);
	}

	/* Ungrab the keys in the window.*/
	pub fn ungrab_keys(&self) {
		unsafe {
			xlib::XUngrabKey(
				self.display.inner,
				xlib::AnyKey,
				xlib::AnyModifier,
				self.xid,
			);
		}
		trace!("Ungrabbed the keys of {:?}", self.xid);
	}
}

/* Capture implementation for X11 Window */
impl Capture for Window {
	/**
	 * Get the image of the window.
	 *
	 * @return Image (Option)
	 */
	fn get_image(&self) -> Option<Image> {
		unsafe {
			let window_image = xlib::XGetImage(
				self.display.inner,
				self.xid,
				self.area.x,
				self.area.y,
				self.area.width,
				self.area.height,
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
				Some(Image::new(
					data,
					self.display.settings.flag.alpha,
					self.area,
				))
			} else {
				None
			}
		}
	}

	/* Show a countdown on the corner of window. */
	fn show_countdown(&self) {
		if self.display.settings.time.countdown != 0 {
			let clock = FpsClock::new(1000);
			for i in 0..(self.display.settings.time.countdown + 1) {
				self.clear_area();
				self.show_text(
					if i != self.display.settings.time.countdown {
						info!(
							"Starting in {}{}\r",
							self.display.settings.time.countdown - i,
							if self.display.settings.time.countdown > 9 {
								" "
							} else {
								""
							}
						);
						io::stdout().flush().expect("Failed to flush stdout");
						Some(format!(
							"[{}]",
							self.display.settings.time.countdown - i
						))
					} else {
						None
					},
					clock,
				);
			}
			info!("\r");
		}
		self.clear_area();
	}

	/* Close the display */
	fn release(&self) {
		trace!("Display closed.");
		unsafe {
			xlib::XCloseDisplay(self.display.inner);
		}
	}
}

#[cfg(test)]
#[cfg(feature = "test-ws")]
mod tests {
	use super::*;
	use crate::record::settings::RecordSettings;
	use crate::record::settings::RecordTime;
	use crate::x11::display::Display;
	use image::ExtendedColorType;
	use pretty_assertions::assert_eq;
	use x11::keysym;
	#[test]
	fn test_x11_window() {
		let mut settings = RecordSettings::default();
		settings.time = RecordTime::new(Some(0.0), 1, 0, 10);
		let display = Display::open(Some(settings)).unwrap();
		let window = display.get_root_window();
		unsafe {
			xlib::XStoreName(
				window.display.inner,
				window.xid,
				CString::new("root-window").unwrap_or_default().as_ptr(),
			);
		};
		window.grab_key(keysym::XK_space.into());
		window.ungrab_keys();
		window.draw_borders();
		window.show_countdown();
		window.clear_area();
		assert_eq!(Geometry::new(0, 0, 1366, 768), window.get_crtc_info()[0]);
		assert_eq!(0, unsafe { window.get_parent() }.unwrap().xid);
		assert_eq!(
			"\n Window title  -> \"root-window\"\n Window size   -> [1366x768]",
			format!("{}", window)
		);
		assert_eq!((0, 0), (window.geometry.x, window.geometry.y));
		assert_eq!("root-window", window.get_name().unwrap());
		assert_eq!(
			1366 * 768 * 3,
			window
				.get_image()
				.unwrap()
				.get_data(ExtendedColorType::Rgb8)
				.len()
		);
		window.release();
	}
}
