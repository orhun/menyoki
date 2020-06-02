/* BGR color fields and padding */
#[derive(Debug)]
pub struct Bgr {
	b: u8,
	g: u8,
	r: u8,
	_p: u8,
}

impl Bgr {
	/**
	 * Create a new Bgr object.
	 *
	 * @param  b
	 * @param  g
	 * @param  r
	 * @return Bgr
	 */
    #[allow(dead_code)]
	pub fn new(b: u8, g: u8, r: u8) -> Self {
		Self { b, g, r, _p: 0 }
	}

	/**
	 * Convert a BGR slice to RGB pixel vector.
	 *
	 * @param  bgr_data
	 * @return Vector of u8
	 */
	pub fn get_rgb_pixels(bgr_data: &[Bgr]) -> Vec<u8> {
		let mut pixels = Vec::new();
		for bgr in bgr_data {
			pixels.extend(&[bgr.r, bgr.g, bgr.b])
		}
		pixels
	}
}
