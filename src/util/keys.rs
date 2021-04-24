use device_query::Keycode;
use std::fmt;
use std::str::FromStr;

/* Operational keys and combinations */
#[derive(Debug)]
pub struct ActionKeys {
	key_groups: Vec<Vec<Keycode>>,
}

/* Display implementation for user-facing output */
impl fmt::Display for ActionKeys {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"{}",
			self.key_groups
				.iter()
				.map(|keys| keys
					.iter()
					.map(|key| format!("{:?}", key))
					.collect::<Vec<String>>()
					.join("-"))
				.collect::<Vec<String>>()
				.join(",")
		)
	}
}

/* Default initialization values for ActionKeys */
impl Default for ActionKeys {
	fn default() -> Self {
		Self {
			key_groups: vec![
				vec![Keycode::LAlt, Keycode::S],
				vec![Keycode::LAlt, Keycode::Enter],
			],
		}
	}
}

impl ActionKeys {
	/**
	 * Create a new ActionKeys object.
	 *
	 * @param  key_groups
	 * @return ActionKeys
	 */
	pub fn new(key_groups: Vec<Vec<Keycode>>) -> Self {
		Self { key_groups }
	}

	/**
	 * Return the primary keycodes.
	 *
	 * @return Vector of KeyCode
	 */
	pub fn get_primary(&self) -> Vec<&Keycode> {
		self.key_groups
			.iter()
			.filter_map(|keys| keys.get(0))
			.collect()
	}

	/**
	 * Parse ActionKeys from a string.
	 *
	 * @param  keys
	 * @return ActionKeys
	 */
	pub fn parse(keys: &str) -> Self {
		let keys = keys
			.split(',')
			.filter_map(|keys| {
				let group = keys
					.split('-')
					.filter_map(|v| Keycode::from_str(v).ok())
					.collect::<Vec<Keycode>>();
				(!group.is_empty()).then(|| group)
			})
			.collect::<Vec<Vec<Keycode>>>();
		if keys.is_empty() {
			Self::default()
		} else {
			Self::new(keys)
		}
	}

	/**
	 * Check if the given Vector contains action keys.
	 *
	 * @param  keys
	 * @return bool
	 */
	pub fn check(&self, keys: Vec<Keycode>) -> bool {
		for target_keys in &self.key_groups {
			if target_keys.len() == keys.len()
				&& target_keys.len()
					== target_keys.iter().filter(|k| keys.contains(k)).count()
			{
				return true;
			}
		}
		false
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use pretty_assertions::assert_eq;
	#[test]
	fn test_action_keys() {
		let keys_str = "LControl-Q,LControl-W";
		let keys = ActionKeys::parse(keys_str);
		assert_eq!(keys_str, keys.to_string());
		assert_eq!(
			vec![
				vec![Keycode::LControl, Keycode::Q],
				vec![Keycode::LControl, Keycode::W]
			],
			keys.key_groups
		);
		assert!(!keys.check(vec![Keycode::RAlt, Keycode::X]));
		assert!(!keys.check(vec![Keycode::LControl, Keycode::X]));
		assert!(!keys.check(vec![Keycode::LControl]));
		assert!(!keys.check(vec![Keycode::W]));
		assert!(keys.check(vec![Keycode::LControl, Keycode::Q]));
		assert!(keys.check(vec![Keycode::LControl, Keycode::W]));
		assert!(!ActionKeys::parse("S").check(vec![Keycode::S, Keycode::Slash]));
		assert!(ActionKeys::parse("X,Y").check(vec![Keycode::X]));
		assert!(ActionKeys::parse("LControl-J,A,B,C")
			.check(vec![Keycode::LControl, Keycode::J]));
		assert!(ActionKeys::parse("LControl-A,X,Y").check(vec![Keycode::Y]));
		assert_eq!(
			ActionKeys::default().key_groups,
			ActionKeys::parse("LCxntrxl-WW").key_groups
		);
		assert_eq!(
			vec![vec![Keycode::X]],
			ActionKeys::parse("test,X,...").key_groups
		);
		assert_eq!(
			vec![vec![Keycode::LControl], vec![Keycode::X]],
			ActionKeys::parse("LControl-test,X").key_groups
		);
		assert_eq!(
			vec![&Keycode::A, &Keycode::C],
			ActionKeys::parse("A-B,C-D,...").get_primary()
		);
	}
}
