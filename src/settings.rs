use crate::app::AppResult;
use crate::args::parser::ArgParser;
use crate::edit::settings::EditSettings;
use crate::gif::settings::{GifSettings, SplitSettings};
use crate::image::settings::{JpgSettings, PngSettings};
use crate::record::settings::RecordSettings;
use crate::util::cmd::Command;
use crate::util::file::FileFormat;
use crate::util::settings::SaveSettings;
use crate::util::state::InputState;
use clap::ArgMatches;
use std::io::{Error, ErrorKind};
use std::str::FromStr;

/* General application settings */
#[derive(Debug)]
pub struct AppSettings<'a> {
	pub args: &'a ArgMatches<'a>,
	pub record: RecordSettings,
	pub gif: GifSettings,
	pub png: PngSettings,
	pub jpg: JpgSettings,
	pub save: SaveSettings,
	pub edit: EditSettings<'a>,
	pub split: SplitSettings<'a>,
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
		let edit = EditSettings::from_args(ArgParser::from_subcommand(args, "edit"));
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
			gif: GifSettings::from_args(ArgParser::from_subcommand(
				args,
				if args.is_present("make") {
					"make"
				} else {
					"gif"
				},
			)),
			png: PngSettings::from_args(ArgParser::from_subcommand(args, "png")),
			jpg: JpgSettings::from_args(ArgParser::from_subcommand(args, "jpg")),
			save: SaveSettings::from_args(
				ArgParser::from_subcommand(args, "save"),
				if edit.convert {
					FileFormat::from_args(args)
				} else {
					FileFormat::from_str(
						edit.path
							.extension()
							.unwrap_or_default()
							.to_str()
							.unwrap_or_default(),
					)
					.unwrap_or_else(|_| FileFormat::from_args(args))
				},
			),
			edit,
			split: SplitSettings::from_args(ArgParser::from_subcommand(
				args, "split",
			)),
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

	/* Check the settings and warn the user. */
	pub fn check(&self) -> AppResult {
		trace!("{:?}", self);
		if self.jpg.quality <= 25 {
			Ok(warn!("Image will be encoded in low quality."))
		} else if self.gif.quality <= 20 {
			Ok(warn!("GIF will be encoded in low quality."))
		} else if self.args.is_present("make") && self.gif.frames.is_empty() {
			error!("No frames found to make a GIF.");
			Err(Error::new(ErrorKind::Other, "No frames specified"))
		} else {
			Ok(())
		}
	}
}
