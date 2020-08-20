use device_query::Keycode;
use std::str::FromStr;

/* Operational keys and combinations */
#[derive(Debug)]
pub struct ActionKeys {
	main_key: Keycode,
	opt_keys: Vec<Keycode>,
}

impl ActionKeys {
	/**
	 * Create a new ActionKeys object.
	 *
	 * @param  main_key
	 * @param  opt_keys
	 * @return ActionKeys
	 */
	pub fn new(main_key: Keycode, opt_keys: Vec<Keycode>) -> Self {
		Self { main_key, opt_keys }
	}

	/**
	 * Parse ActionKeys from a string.
	 *
	 * @param  keys
	 * @return ActionKeys
	 */
	pub fn parse(keys: &str) -> Self {
		let keys = keys.split('-').collect::<Vec<&str>>();
		Self::new(
			Keycode::from_str(keys.get(0).unwrap_or(&"LAlt"))
				.unwrap_or(Keycode::LAlt),
			keys.get(1)
				.unwrap_or(&"S/Enter")
				.split('/')
				.map(|k| {
					Keycode::from_str(k)
						.expect(&format!("Failed to parse the keycode: {}", k))
				})
				.collect(),
		)
	}

	/**
	 * Check if the given Vector contains action keys.
	 *
	 * @param  keys
	 * @return bool
	 */
	pub fn check(&self, keys: Vec<Keycode>) -> bool {
		if !keys.contains(&self.main_key) {
			false
		} else {
			let mut pressed = false;
			for key in &self.opt_keys {
				if keys.contains(&key) {
					pressed = true;
					break;
				}
			}
			pressed
		}
	}
}
