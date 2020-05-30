/* GIF and frame settings */
#[derive(Clone, Copy, Debug)]
pub struct GifSettings {
	pub repeat: i32,
	pub speed: u32,
}

impl GifSettings {
	/**
	 * Create a new GifSettings object.
	 *
	 * @param  repeat
	 * @param  speed
	 * @return GifSettings
	 */
	pub fn new(repeat: i32, speed: u32) -> Self {
		Self { repeat, speed }
	}
}

/* Default initialization values for GifSettings */
impl Default for GifSettings {
	fn default() -> Self {
		Self {
			repeat: -1,
			speed: 10,
		}
	}
}
