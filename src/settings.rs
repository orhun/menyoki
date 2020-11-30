use crate::analyze::settings::AnalyzeSettings;
use crate::anim::settings::{AnimSettings, SplitSettings};
use crate::args::matches::ArgMatches;
use crate::edit::settings::EditSettings;
use crate::file::format::FileFormat;
use crate::file::settings::SaveSettings;
use crate::image::geometry::Geometry;
use crate::image::settings::{JpgSettings, PngSettings, PnmSettings};
use crate::record::settings::{RecordSettings, RecordWindow};
use crate::util::keys::ActionKeys;
use crate::util::state::InputState;
use colored::Color;

/* General application settings */
#[derive(Debug)]
pub struct AppSettings<'a> {
	pub args: &'a ArgMatches<'a>,
	pub record: RecordSettings,
	pub anim: AnimSettings,
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
		let record = RecordSettings::from_args(args);
		let pnm = PnmSettings::from_args(args);
		let edit = EditSettings::from_args(args);
		let save = SaveSettings::from_args(args, &edit, &pnm);
		let input_state = Self::get_input_state(window_required, &record);
		Self {
			args,
			record,
			anim: AnimSettings::from_args(args, &save.file.format),
			split: SplitSettings::from_args(args),
			png: PngSettings::from_args(args),
			jpg: JpgSettings::from_args(args),
			analyze: AnalyzeSettings::from_args(args, Self::get_color(args)),
			pnm,
			edit,
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
				InputState::new(if let Some(keys) = record.flag.keys {
					ActionKeys::parse(keys)
				} else {
					ActionKeys::default()
				})
				.into_boxed_state(),
			))
		} else {
			None
		}
	}

	/* Check the settings and update if necessary. */
	pub fn check(&mut self) {
		trace!("{:?}", self);
		if self.jpg.quality <= 25 {
			warn!("Image will be encoded in low quality.")
		}
		if self.anim.quality <= 20 {
			warn!("Animation will be encoded in low quality.")
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

#[cfg(test)]
mod tests {
	use super::*;
	use clap::ArgMatches as Args;
	use pretty_assertions::assert_eq;
	use std::env;
	#[test]
	fn test_app_settings() {
		let args = Args::default();
		let matches = ArgMatches::new(&args);
		let mut settings = AppSettings::new(&matches);
		env::set_var(
			&format!("{}_general_color", env!("CARGO_PKG_NAME")).to_uppercase(),
			"d473d4",
		);
		assert_eq!(
			format!("{:?}", settings.get_main_color()),
			"Some(TrueColor { r: 212, g: 115, b: 212 })"
		);
		settings.jpg.quality = 0;
		settings.anim.quality = 0;
		settings.save.file.format = FileFormat::Ico;
		settings.record.window = RecordWindow::Focus(Some(Geometry::default()));
		settings.check();
	}
}
