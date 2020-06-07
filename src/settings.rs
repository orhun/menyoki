use crate::args::parser::ArgParser;
use crate::gif::settings::GifSettings;
use crate::record::settings::RecordSettings;
use crate::util::cmd::Command;
use crate::util::settings::SaveSettings;
use clap::ArgMatches;

/* General application settings */
#[derive(Debug)]
pub struct AppSettings<'a> {
	pub args: &'a ArgMatches<'a>,
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
	pub fn new(args: &'a ArgMatches<'a>) -> Self {
		Self {
			args,
			gif: Self::get_gif_settings(args),
			record: Self::get_record_settings(args),
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
	 * Get recording settings from parsed arguments.
	 *
	 * @param  args
	 * @return RecordSettings
	 */
	fn get_record_settings(args: &'a ArgMatches<'a>) -> RecordSettings {
		RecordSettings::from_args(ArgParser::from_subcommand(&args, vec!["record"]))
	}

	/**
	 * Get GIF settings from parsed arguments.
	 *
	 * @param  args
	 * @return GifSettings
	 */
	fn get_gif_settings(args: &'a ArgMatches<'a>) -> GifSettings {
		GifSettings::from_args(ArgParser::from_subcommand(
			&args,
			vec!["record", "gif"],
		))
	}

	/**
	 * Get output file settings from parsed arguments.
	 *
	 * @param  args
	 * @return SaveSettings
	 */
	fn get_save_settings(args: &'a ArgMatches<'a>) -> SaveSettings {
		SaveSettings::from_args(ArgParser::from_subcommand(
			&args,
			vec!["record", "gif", "save"],
		))
	}
}
