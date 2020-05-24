/* Timeout value for the window selection */
pub const SELECT_WINDOW_TIMEOUT: u128 = 30 * 1000;
/* Time interval between focused window checks */
pub const SELECTION_INTERVAL: u64 = 10;

/* Display settings regarding to window selection */
pub struct DisplaySettings {
	pub timeout: u128,
	pub interval: u64,
}

impl DisplaySettings {
	/**
	 * Create a new DisplaySettings object.
	 *
	 * @param  timeout
	 * @param  interval
	 * @return DisplaySettings
	 */
	#[allow(dead_code)]
	pub fn new(timeout: u128, interval: u64) -> Self {
		Self { timeout, interval }
	}
}

/* Default state of the display settings */
impl Default for DisplaySettings {
	fn default() -> Self {
		Self {
			timeout: SELECT_WINDOW_TIMEOUT,
			interval: SELECTION_INTERVAL,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn test_display_settings() {
		assert_eq!(SELECT_WINDOW_TIMEOUT, DisplaySettings::default().timeout);
		assert_eq!(SELECTION_INTERVAL, DisplaySettings::default().interval);
	}
}
