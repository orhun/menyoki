/* BGR color fields and padding */
#[derive(Debug)]
pub struct Bgra {
	b: u8,
	g: u8,
	r: u8,
	alpha: u8,
}

impl Bgra {
	/**
	 * Create a new Bgra object.
	 *
	 * @param  b
	 * @param  g
	 * @param  r
	 * @param  alpha
	 * @return Bgra
	 */
	#[allow(dead_code)]
	pub fn new(b: u8, g: u8, r: u8, alpha: u8) -> Self {
		Self { b, g, r, alpha }
	}

	/**
	 * Convert a BGRA slice to RGBA pixel vector.
	 *
	 * @param  bgra_data
	 * @param  alpha
	 * @return Vector of u8
	 */
	pub fn get_rgba_pixels(bgra_data: &[Bgra], alpha: bool) -> Vec<u8> {
		let mut pixels = Vec::new();
		for bgra in bgra_data {
			pixels.extend(&[
				bgra.r,
				bgra.g,
				bgra.b,
				if alpha { bgra.alpha } else { 255 },
			]);
		}
		pixels
	}
}
