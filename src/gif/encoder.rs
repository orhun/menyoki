use crate::gif::settings::GifSettings;
use crate::image::geometry::Geometry;
use crate::image::Image;
use crate::util::state::InputState;
use std::io::{Error, Write};

/* Required encoding methods for Gif */
pub trait Encoder<Output: Write> {
	fn new(
		geometry: Geometry,
		output: Output,
		fps: u32,
		settings: GifSettings,
	) -> Result<Self, Error>
	where
		Self: Sized;
	fn save(
		self,
		images: Vec<Image>,
		input_state: &'static InputState,
	) -> Result<(), Error>;
}
