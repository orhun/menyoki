use crate::image::{Geometry, Image, Padding};
use crate::record::settings::RecordSettings;
use crate::record::Record;

#[derive(Clone, Copy, Debug)]
pub struct TestWindow {
	pub geometry: Geometry,
}

/* Default initialization values for TestWindow */
impl Default for TestWindow {
	fn default() -> Self {
		Self::new(Geometry::new(0, 0, 1, 1, Padding::default()))
	}
}

impl TestWindow {
	/**
	 * Create a new TestWindow object.
	 *
	 * @param  geometry
	 * @return TestWindow
	 */
	pub fn new(geometry: Geometry) -> Self {
		Self { geometry }
	}
}

/* Test recording implementation for TestWindow */
impl Record for TestWindow {
	/**
	 * Get the test image.
	 *
	 * @return Image (Option)
	 */
	fn get_image(&self) -> Option<Image> {
		Some(Image::new(vec![255, 255, 255], self.geometry))
	}

	/* Do not show countdown for testing window. */
	fn show_countdown(&self, _: RecordSettings) {}
}
