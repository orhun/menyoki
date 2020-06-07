use crate::args::parser::ArgParser;
use chrono::Local;

/* Output file settings */
#[derive(Debug)]
pub struct SaveSettings {
	file: String,
}

/* Default initialization values for SaveSettings */
impl Default for SaveSettings {
	fn default() -> Self {
		Self {
			file: String::from("t.gif"),
		}
	}
}

impl SaveSettings {
	/**
	 * Create a new SaveSettings object.
	 *
	 * @param  file
	 * @return SaveSettings
	 */
	pub fn new(file: String) -> Self {
		Self { file }
	}

	/**
	 * Create a SaveSettings object from parsed arguments.
	 *
	 * @param  parser
	 * @return SaveSettings
	 */
	pub fn from_args(parser: ArgParser<'_>) -> Self {
		match parser.args {
			Some(matches) => {
				let mut file_name =
					String::from(matches.value_of("output").unwrap_or_default());
				if matches.is_present("prompt") {
					file_name = rprompt::prompt_reply_stdout("Enter file name: ")
						.unwrap_or(file_name);
				}
				SaveSettings::new(
					if matches.is_present("date") || matches.is_present("timestamp")
					{
						Self::update_file_name(
							file_name,
							if matches.is_present("date") {
								Local::now().format("%Y%m%dT%H%M%S").to_string()
							} else {
								Local::now().timestamp().to_string()
							},
						)
					} else {
						file_name
					},
				)
			}
			None => SaveSettings::default(),
		}
	}

	/**
	 * Append the given information to the file name.
	 *
	 * @param  file_name
	 * @param  info
	 * @return String
	 */
	fn update_file_name(file_name: String, info: String) -> String {
		file_name
			.split('.')
			.enumerate()
			.map(|(i, s)| {
				if i == 0 {
					format!("{}_{}", s, info)
				} else {
					format!(".{}", s)
				}
			})
			.collect::<String>()
	}
}
