/* BGR color fields and padding */
#[derive(Debug)]
pub struct Bgra {
	b: u8,
	g: u8,
	r: u8,
	a: u8,
}

impl Bgra {
	/**
	 * Create a new Bgra object.
	 *
	 * @param  b
	 * @param  g
	 * @param  r
	 * @return Bgra
	 */
	#[allow(dead_code)]
	pub fn new(b: u8, g: u8, r: u8) -> Self {
		Self { b, g, r, a: 0 }
	}

	/**
	 * Convert a BGR slice to RGB pixel vector.
	 *
	 * @param  bgra_data
	 * @return Vector of u8
	 */
	pub fn get_rgb_pixels(bgra_data: &[Bgra]) -> Vec<u8> {
		let mut pixels = Vec::new();
		for bgr in bgra_data {
			pixels.extend(&[bgr.r, bgr.g, bgr.b])
		}
		pixels
	}
}
