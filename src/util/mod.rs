pub mod command;
pub mod keys;
pub mod logger;
pub mod state;
use chrono::{Datelike, Utc, Weekday};

/**
 * Map the given number from a range to another range.
 *
 * @param  value
 * @param  from_range
 * @param  to_range
 * @return f64
 */
pub fn map_range(value: f64, from_range: (f64, f64), to_range: (f64, f64)) -> f64 {
	to_range.0
		+ (value - from_range.0) * (to_range.1 - to_range.0)
			/ (from_range.1 - from_range.0)
}

/* Check if today is friday. */
pub fn check_friday() {
	if Utc::now().weekday() == Weekday::Fri {
		println!("(ﾉ◕ヮ◕)ﾉ*:･ﾟ✧ Thank God It's Friday! ┬──┬ ノ( ゜-゜ノ)");
	}
}
