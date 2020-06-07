use crate::args::parser::ArgParser;
use crate::gif::settings::GifSettings;
use crate::record::settings::RecordSettings;
use crate::util;
use crate::util::cmd::Command;
use crate::util::settings::SaveSettings;
use chrono::Local;
use clap::ArgMatches;

/* General application settings */
#[derive(Debug)]
pub struct AppSettings<'a> {
	pub args: ArgMatches<'a>,
	pub gif: GifSettings,
	pub record: RecordSettings,
	pub save: SaveSettings,
}

impl<'a> AppSettings<'a> {
	/**
	 * Create a new AppSettings object.
	 *
	 * @param  args
	 * @return AppSettings
	 */
	pub fn new(args: ArgMatches<'a>) -> Self {
		Self {
			args: args.clone(),
			gif: Self::get_gif_settings(args.clone()),
			record: Self::get_record_settings(args.clone()),
			save: Self::get_save_settings(args),
		}
	}

	/**
	 * Get a Command object from parsed arguments.
	 *
	 * @return Command
	 */
	pub fn get_command(&self) -> Command {
		match self.args.value_of("command") {
			Some(cmd) => {
				let cmd = String::from(cmd);
				if !cmd.contains(' ') {
					Command::new(cmd, Vec::new())
				} else {
					Command::new(String::from("sh"), vec![String::from("-c"), cmd])
				}
			}
			None => panic!("No command specified to run"),
		}
	}

	/**
	 * Get the output file from parsed arguments.
	 *
	 * @return String
	 */
	pub fn get_output_file(&self) -> String {
		match ArgParser::from_subcommand(&self.args, vec!["record", "gif", "save"])
			.args
		{
			Some(matches) => {
				let mut file_name =
					String::from(matches.value_of("output").unwrap_or_default());
				if matches.is_present("prompt") {
					file_name = rprompt::prompt_reply_stdout("Enter file name: ")
						.unwrap_or(file_name);
				}
				if matches.is_present("date") || matches.is_present("timestamp") {
					util::update_file_name(
						file_name,
						if matches.is_present("date") {
							Local::now().format("%Y%m%dT%H%M%S").to_string()
						} else {
							Local::now().timestamp().to_string()
						},
					)
				} else {
					file_name
				}
			}
			None => String::from("t.gif"),
		}
	}

	/**
	 * Get recording settings from parsed arguments.
	 *
	 * @param  args
	 * @return RecordSettings
	 */
	fn get_record_settings(args: ArgMatches<'a>) -> RecordSettings {
		RecordSettings::from_args(ArgParser::from_subcommand(&args, vec!["record"]))
	}

	/**
	 * Get save settings from parsed arguments.
	 *
	 * @param  args
	 * @return SaveSettings
	 */
	fn get_save_settings(args: ArgMatches<'a>) -> SaveSettings {
		SaveSettings::from_args(ArgParser::from_subcommand(
			&args,
			vec!["record", "gif", "save"],
		))
	}

	/**
	 * Get GIF settings from parsed arguments.
	 *
	 * @param  args
	 * @return GifSettings
	 */
	fn get_gif_settings(args: ArgMatches<'a>) -> GifSettings {
		GifSettings::from_args(ArgParser::from_subcommand(
			&args,
			vec!["record", "gif"],
		))
	}
}
