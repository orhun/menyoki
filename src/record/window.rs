use crate::image::geometry::Geometry;
use crate::image::Image;
use crate::record::Capture;
use image::Bgra;

/* Testing window */
#[derive(Clone, Copy, Debug)]
pub struct TestWindow {
	pub geometry: Geometry,
}

/* Default initialization values for TestWindow */
impl Default for TestWindow {
	fn default() -> Self {
		Self::new(Geometry::new(0, 0, 1, 1))
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
impl Capture for TestWindow {
	/**
	 * Get the test image.
	 *
	 * @return Image (Option)
	 */
	fn get_image(&self) -> Option<Image> {
		Some(Image::new(
			vec![Bgra::from([255, 255, 255, 0])],
			false,
			self.geometry,
		))
	}

	/* Do not show countdown for testing window. */
	fn show_countdown(&self) {}

	/* Do not do anything with respect to release. */
	fn release(&self) {}
}
