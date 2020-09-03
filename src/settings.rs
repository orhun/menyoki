use crate::args::parser::ArgParser;
use crate::edit::settings::EditSettings;
use crate::gif::settings::{GifSettings, SplitSettings};
use crate::image::settings::{JpgSettings, PngSettings, PnmSettings};
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
	pub split: SplitSettings,
	pub png: PngSettings,
	pub jpg: JpgSettings,
	pub pnm: PnmSettings,
	pub edit: EditSettings,
	pub save: SaveSettings,
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
		let record = Self::get_record_settings(args);
		let gif = Self::get_gif_settings(args);
		let split =
			SplitSettings::from_args(ArgParser::from_subcommand(args, "split"));
		let png = PngSettings::from_args(ArgParser::from_subcommand(args, "png"));
		let jpg = JpgSettings::from_args(ArgParser::from_subcommand(args, "jpg"));
		let pnm = PnmSettings::from_args(ArgParser::from_subcommand(args, "pnm"));
		let edit = EditSettings::from_args(ArgParser::from_subcommand(args, "edit"));
		let save = Self::get_save_settings(args, &edit, &pnm);
		let input_state = Self::get_input_state(window_required, &record);
		Self {
			args,
			record,
			gif,
			split,
			png,
			jpg,
			pnm,
			edit,
			save,
			input_state,
			window_required,
		}
	}

	/**
	 * Get RecordSettings from parsed arguments.
	 *
	 * @param  args
	 * @return RecordSettings
	 */
	fn get_record_settings(args: &'a ArgMatches<'a>) -> RecordSettings {
		RecordSettings::from_args(ArgParser::from_subcommand(
			args,
			if args.is_present("capture") {
				"capture"
			} else {
				"record"
			},
		))
	}

	/**
	 * Get GifSettings from parsed arguments.
	 *
	 * @param  args
	 * @return GifSettings
	 */
	fn get_gif_settings(args: &'a ArgMatches<'a>) -> GifSettings {
		GifSettings::from_args(ArgParser::from_subcommand(
			args,
			if args.is_present("make") {
				"make"
			} else {
				"gif"
			},
		))
	}

	/**
	 * Get SaveSettings from parsed arguments.
	 *
	 * @param  args
	 * @param  edit
	 * @return SaveSettings
	 */
	fn get_save_settings(
		args: &'a ArgMatches<'a>,
		edit: &EditSettings,
		pnm: &PnmSettings,
	) -> SaveSettings {
		SaveSettings::from_args(
			ArgParser::from_subcommand(args, "save"),
			if edit.convert {
				FileFormat::from_args(args, Some(pnm.subtype))
			} else {
				FileFormat::from_str(
					edit.path
						.extension()
						.unwrap_or_default()
						.to_str()
						.unwrap_or_default(),
				)
				.unwrap_or_else(|_| FileFormat::from_args(args, Some(pnm.subtype)))
			},
		)
	}

	/**
	 * Get InputState if a window is required.
	 *
	 * @param  window_required
	 * @param  record
	 * @return InputState (Option)
	 */
	fn get_input_state(
		window_required: bool,
		record: &RecordSettings,
	) -> Option<&'static InputState> {
		if window_required {
			Some(Box::leak(
				InputState::new(ActionKeys::parse(
					record.flag.keys.unwrap_or_default(),
				))
				.into_boxed_state(),
			))
		} else {
			None
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
