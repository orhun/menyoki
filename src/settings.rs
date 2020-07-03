use crate::args::parser::ArgParser;
use crate::gif::settings::GifSettings;
use crate::image::settings::{JpgSettings, PngSettings};
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
	pub png: PngSettings,
	pub jpg: JpgSettings,
	pub save: SaveSettings,
	pub input_state: &'static InputState,
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
			record: RecordSettings::from_args(ArgParser::from_subcommand(
				args,
				if args.is_present("capture") {
					"capture"
				} else {
					"record"
				},
			)),
			gif: GifSettings::from_args(ArgParser::from_subcommand(args, "gif")),
			png: PngSettings::from_args(ArgParser::from_subcommand(args, "png")),
			jpg: JpgSettings::from_args(ArgParser::from_subcommand(args, "jpg")),
			save: SaveSettings::from_args(
				args,
				ArgParser::from_subcommand(args, "save"),
			),
			input_state: Box::leak(Box::new(InputState::new())),
		}
	}

	/**
	 * Get a Command object from parsed arguments.
	 *
	 * @return Command (Option)
	 */
	pub fn get_command(&self) -> Option<Command> {
		match self.args.value_of("command") {
			Some(cmd) => {
				let cmd = String::from(cmd);
				if !cmd.contains(' ') {
					Some(Command::new(cmd, Vec::new()))
				} else {
					Some(Command::new(
						String::from("sh"),
						vec![String::from("-c"), cmd],
					))
				}
			}
			None => None,
		}
	}
}
