use crate::analyze::settings::AnalyzeSettings;
use crate::args::matches::ArgMatches;
use crate::args::parser::ArgParser;
use crate::edit::settings::EditSettings;
use crate::file::settings::SaveSettings;
use crate::file::FileFormat;
use crate::gif::settings::{GifSettings, SplitSettings};
use crate::image::geometry::Geometry;
use crate::image::settings::{JpgSettings, PngSettings, PnmSettings};
use crate::record::settings::{RecordSettings, RecordWindow};
use crate::util::keys::ActionKeys;
use crate::util::state::InputState;
use colored::Color;
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
	pub analyze: AnalyzeSettings,
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
			SplitSettings::from_parser(ArgParser::from_subcommand(args, "split"));
		let png = PngSettings::from_parser(ArgParser::from_subcommand(args, "png"));
		let jpg = JpgSettings::from_parser(ArgParser::from_subcommand(args, "jpg"));
		let pnm = PnmSettings::from_parser(ArgParser::from_subcommand(args, "pnm"));
		let edit =
			EditSettings::from_parser(ArgParser::from_subcommand(args, "edit"));
		let analyze = AnalyzeSettings::from_parser(
			ArgParser::from_subcommand(args, "analyze"),
			Self::get_color(args),
		);
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
			analyze,
			save,
			input_state,
			window_required,
		}
	}

	/**
	 * Get the main color from parsed arguments. (exposed)
	 *
	 * @return Color (Option)
	 */
	pub fn get_main_color(&self) -> Option<Color> {
		Self::get_color(self.args)
	}

	/**
	 * Get the main color from parsed arguments.
	 *
	 * @param  args
	 * @return Color (Option)
	 */
	fn get_color(args: &'a ArgMatches<'a>) -> Option<Color> {
		if let Some(color) = args.value_of("color") {
			match hex::decode(color) {
				Ok(rgb) => Some(Color::TrueColor {
					r: rgb[0],
					g: rgb[1],
					b: rgb[2],
				}),
				Err(_) => None,
			}
		} else {
			None
		}
	}

	/**
	 * Get RecordSettings from parsed arguments.
	 *
	 * @param  args
	 * @return RecordSettings
	 */
	fn get_record_settings(args: &'a ArgMatches<'a>) -> RecordSettings {
		RecordSettings::from_parser(
			ArgParser::from_subcommand(
				args,
				if args.is_present("capture") {
					"capture"
				} else {
					"record"
				},
			),
			args.value_of("color").unwrap_or_default(),
		)
	}

	/**
	 * Get GifSettings from parsed arguments.
	 *
	 * @param  args
	 * @return GifSettings
	 */
	fn get_gif_settings(args: &'a ArgMatches<'a>) -> GifSettings {
		GifSettings::from_parser(ArgParser::from_subcommand(
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
		let format = FileFormat::from_args(args, Some(pnm.subtype));
		SaveSettings::from_parser(
			ArgParser::from_subcommand(args, "save"),
			if edit.convert {
				format
			} else {
				FileFormat::from_str(
					edit.path
						.extension()
						.unwrap_or_default()
						.to_str()
						.unwrap_or_default(),
				)
				.unwrap_or(format)
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

	/* Check the settings and update it if necessary. */
	pub fn check(&mut self) {
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
		if self.save.file.format == FileFormat::Ico {
			self.set_icon_size()
		}
	}

	/* Set the area size to 256x256 for encoding ICO. */
	fn set_icon_size(&mut self) {
		let ico_geometry = Geometry::new(0, 0, 256, 256);
		match self.record.window {
			RecordWindow::Focus(None) => {
				self.record.window = RecordWindow::Focus(Some(ico_geometry))
			}
			RecordWindow::Root(None) => {
				self.record.window = RecordWindow::Root(Some(ico_geometry))
			}
			RecordWindow::Focus(Some(ref mut geometry))
			| RecordWindow::Root(Some(ref mut geometry)) => {
				if geometry.width == 0 || geometry.width > ico_geometry.width {
					geometry.width = ico_geometry.width;
				}
				if geometry.height == 0 || geometry.height > ico_geometry.width {
					geometry.height = ico_geometry.height;
				}
			}
		}
	}
}
