use crate::args::matches::ArgMatches;
use chrono::Local;
use std::fmt;

/* Information to include in file name */
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FileInfo<'a> {
	Date(&'a str),
	Timestamp,
}

impl<'a> FileInfo<'a> {
	/**
	 * Create a FileInfo enum from parsed arguments.
	 *
	 * @param  args
	 * @return FileInfo (Option)
	 */
	pub fn from_args(args: &'a ArgMatches<'a>) -> Option<Self> {
		if args.is_present("timestamp") {
			Some(Self::Timestamp)
		} else if args.is_present("date") && args.occurrences_of("date") != 0 {
			Some(Self::Date(args.value_of("date").unwrap_or_default()))
		} else {
			None
		}
	}
}

/* Display implementation for user-facing output */
impl fmt::Display for FileInfo<'_> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"{}",
			match self {
				FileInfo::Date(format) => Local::now().format(format).to_string(),
				FileInfo::Timestamp => Local::now().timestamp().to_string(),
			}
		)
	}
}
