use crate::args::parser::ArgParser;
use crate::edit::settings::EditSettings;
use crate::gif::settings::{GifSettings, SplitSettings};
use crate::image::settings::{JpgSettings, PngSettings};
use crate::record::settings::RecordSettings;
use crate::util::file::FileFormat;
use crate::util::keys::ActionKeys;
use crate::util::settings::SaveSettings;
use crate::util::state::InputState;
use clap::ArgMatches;
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
	pub edit: EditSettings,
	pub split: SplitSettings,
	pub input_state: Option<&'static InputState>,
	pub window_required: bool,
}

impl<'a> AppSettings<'a> {
	/**
	 * Create a new AppSettings object.
	 *
	 * @param  args
	 * @return AppSettings
	 */
	pub fn new(args: &'a ArgMatches<'a>) -> Self {
		let window_required =
			args.is_present("record") || args.is_present("capture");
		let record = RecordSettings::from_args(ArgParser::from_subcommand(
			args,
			if args.is_present("capture") {
				"capture"
			} else {
				"record"
			},
		));
		let edit = EditSettings::from_args(ArgParser::from_subcommand(args, "edit"));
		Self {
			args,
			record,
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
			input_state: if window_required {
				Some(Box::leak(
					InputState::new(ActionKeys::parse(
						record.flag.keys.unwrap_or_default(),
					))
					.into_boxed_state(),
				))
			} else {
				None
			},
			window_required,
		}
	}

	/* Check the settings and warn the user. */
	pub fn check(&self) {
		trace!("{:?}", self);
		if self.jpg.quality <= 25 {
			warn!("Image will be encoded in low quality.")
		}
		if self.gif.quality <= 20 {
			warn!("GIF will be encoded in low quality.")
		}
		if let Some(input_state) = self.input_state {
			if self.record.flag.keys != Some(&ActionKeys::default().to_string()) {
				info!(
					"Using custom action keys: {}",
					input_state.action_keys.to_string()
				);
			}
		}
	}
}
