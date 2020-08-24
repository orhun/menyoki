use device_query::Keycode;
use std::str::FromStr;

/* Operational keys and combinations */
#[derive(Debug)]
pub struct ActionKeys {
	pub main_key: Keycode,
	pub opt_keys: Vec<Keycode>,
}

/* Default initialization values for ActionKeys */
impl Default for ActionKeys {
	fn default() -> Self {
		Self {
			main_key: Keycode::LAlt,
			opt_keys: vec![Keycode::S, Keycode::Enter],
		}
	}
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
			Keycode::from_str(keys.get(0).unwrap_or(&"LAlt")).expect("Invalid key"),
			keys.get(1)
				.unwrap_or(&"S/Enter")
				.split('/')
				.map(|k| {
					Keycode::from_str(k)
						.unwrap_or_else(|_| panic!(format!("Invalid key ({})", k)))
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

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn test_action_keys() {
		let action_keys = ActionKeys::parse("LControl-Q/W");
		assert_eq!(Keycode::LControl, action_keys.main_key);
		assert_eq!(vec![Keycode::Q, Keycode::W], action_keys.opt_keys);
		assert!(!action_keys.check(vec![Keycode::RAlt, Keycode::X]));
		assert!(!action_keys.check(vec![Keycode::LControl, Keycode::X]));
		assert!(!action_keys.check(vec![Keycode::LControl]));
		assert!(!action_keys.check(vec![Keycode::W]));
		assert!(action_keys.check(vec![Keycode::LControl, Keycode::Q]));
		assert!(action_keys.check(vec![Keycode::LControl, Keycode::W]));
	}
}
