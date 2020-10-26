pub mod command;
pub mod keys;
pub mod logger;
pub mod state;
use chrono::{Datelike, Utc, Weekday};

/* Check if today is friday. */
pub fn check_friday() {
	if Utc::now().weekday() == Weekday::Fri {
		println!("(ﾉ◕ヮ◕)ﾉ*:･ﾟ✧ Thank God It's Friday! ┬──┬ ノ( ゜-゜ノ)");
	}
}
