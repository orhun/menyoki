use crate::encode::gif::{Frame, Gif, GifSettings};
use crate::encode::Image;
use crate::record::Recorder;
use crate::util;
use crate::util::cmd::Command;
use chrono::Local;
use clap::ArgMatches;
use std::fs::File;
use std::io::Error;

/* General application settings */
#[derive(Clone, Debug)]
pub struct AppSettings {
	pub args: ArgMatches<'static>,
}

impl AppSettings {
	/**
	 * Create a new AppSettings object.
	 *
	 * @return AppSettings
	 */
	pub fn new(args: ArgMatches<'static>) -> Self {
		Self { args }
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
	 * Get the main color from parsed arguments.
	 *
	 * @return u64
	 */
	pub fn get_color(&self) -> u64 {
		u64::from_str_radix(self.args.value_of("color").unwrap_or("FF00FF"), 16)
			.expect("Failed to parse the color HEX")
	}

	/**
	 * Get FPS value from parsed arguments.
	 *
	 * @return u32
	 */
	fn get_fps(&self) -> u32 {
		self.args
			.value_of("fps")
			.unwrap_or_default()
			.parse()
			.unwrap_or(10)
	}

	/**
	 * Get the output file from parsed arguments.
	 *
	 * @return String
	 */
	fn get_output_file(&self) -> String {
		match self.args.subcommand_matches("save") {
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
	 * Get GIF settings from parsed arguments.
	 *
	 * @return GifSettings
	 */
	fn get_gif_settings(&self) -> GifSettings {
		match self.args.subcommand_matches("gif") {
			Some(matches) => GifSettings::new(
				matches
					.value_of("repeat")
					.unwrap_or("-1")
					.parse()
					.unwrap_or_default(),
				matches
					.value_of("speed")
					.unwrap_or_default()
					.parse()
					.unwrap_or(10),
			),
			None => GifSettings::new(-1, 10),
		}
	}
}

/* Application and main functionalities */
pub struct App {
	settings: AppSettings,
}

impl App {
	/**
	 * Create a new App object.
	 *
	 * @param  settings
	 * @return App
	 */
	pub fn new(settings: AppSettings) -> Self {
		Self { settings }
	}

	/**
	 * Start recording the frames.
	 *
	 * @param  get_image (Fn)
	 * @return Vector of Frame
	 */
	pub fn record(
		&self,
		get_image: impl Fn() -> Option<Image> + Sync + Send + 'static,
	) -> Vec<Frame> {
		let mut recorder = Recorder::new(self.settings.get_fps(), get_image);
		if self.settings.args.is_present("command") {
			let record = recorder.record_async();
			self.settings
				.get_command()
				.execute()
				.expect("Failed to run the command");
			record.finish().expect("Failed to finish the recording");
			record.thread.join().expect("Failed to retrieve the frames")
		} else {
			recorder.record_sync()
		}
	}

	/**
	 * Save frames to a GIF file.
	 *
	 * @param  frames
	 * @return Result
	 */
	pub fn save_gif(&self, frames: Vec<Frame>) -> Result<(), Error> {
		let mut gif = Gif::new(
			File::create(self.settings.get_output_file())
				.expect("Failed to create file"),
			frames
				.first()
				.expect("No frames found to save")
				.image
				.geometry,
			self.settings.get_gif_settings(),
		)?;
		gif.save(frames)?;
		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::encode::Geometry;
	#[test]
	fn test_app_mod() -> Result<(), Error> {
		let settings = AppSettings::new(util::parse_args());
		let app = App::new(settings);
		let geometry = Geometry::new(0, 0, 1, 1);
		let mut frames =
			app.record(move || Some(Image::new(vec![0, 0, 0], geometry)));
		frames.push(Frame::new(Image::new(vec![255, 255, 255], geometry), 0));
		app.save_gif(frames)?;
		Command::new(String::from("rm"), vec![String::from("t.gif")]).execute()?;
		Ok(())
	}
}
