use crate::args::parser::ArgParser;
use crate::gif::settings::GifSettings;
use crate::record::settings::RecordSettings;
use crate::util::cmd::Command;
use crate::util::settings::SaveSettings;
use crate::util::state::InputState;
use clap::ArgMatches;

/* General application settings */
#[derive(Debug)]
pub struct AppSettings<'a> {
	pub args: &'a ArgMatches<'a>,
	pub record: RecordSettings,
	pub gif: GifSettings,
	pub save: SaveSettings,
	pub input_state: InputState,
}

impl<'a> AppSettings<'a> {
	/**
	 * Create a new AppSettings object.
	 *
	 * @param  args
	 * @return AppSettings
	 */
	pub fn new(args: &'a ArgMatches<'a>) -> Self {
		let parser = ArgParser::new(Some(args));
		Self {
			args,
			record: RecordSettings::from_args(
				parser.args_from_subcommand(vec!["record"]),
			),
			gif: GifSettings::from_args(
				parser.args_from_subcommand(vec!["record", "gif"]),
			),
			save: SaveSettings::from_args(
				parser.args_from_subcommand(vec!["record", "gif", "save"]),
			),
			input_state: InputState::new(),
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
}
