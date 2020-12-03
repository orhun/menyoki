use crate::image::Image;
use crate::window::Capture;

/* Window implementation */
#[derive(Clone, Copy, Debug)]
pub struct Window {}

impl Window {
	/**
	 * Create a new Window object.
	 *
	 * @return Window
	 */
	#[allow(dead_code)]
	pub fn new() -> Self {
		unimplemented!()
	}
}

/* Methods for recording/capturing the window */
impl Capture for Window {
	/**
	 * Get an image of the window.
	 *
	 * @return Image (Option)
	 */
	fn get_image(&self) -> Option<Image> {
		unimplemented!()
	}

	/* Show countdown on the window. */
	fn show_countdown(&self) {
		unimplemented!()
	}

	/* Release the window. */
	fn release(&self) {
		unimplemented!()
	}
}
