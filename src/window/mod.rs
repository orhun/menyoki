use crate::image::Image;
use crate::settings::AppSettings;
use std::fmt::Debug;

/* Window system functions for accessing a window */
pub trait Access<'a, Window: Capture + Send + Sync + Copy + Debug + 'static> {
	fn init(settings: &'a AppSettings<'a>) -> Option<Self>
	where
		Self: Sized;
	fn get_window(&mut self) -> Option<Window>;
}

/* Window methods for capturing an image */
pub trait Capture {
	fn get_image(&self) -> Option<Image>;
	fn show_countdown(&self);
	fn release(&self);
}
