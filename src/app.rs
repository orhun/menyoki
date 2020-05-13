use crate::image::gif::{Frame, Gif};
use crate::image::{Geometry, Image};
use crate::record::Recorder;
use crate::util;
use crate::util::cmd::Command;
use chrono::Local;
use clap::ArgMatches;
use std::fs::File;
use std::io::Error;

/* Application and main functionalities */
pub struct App {
	args: ArgMatches<'static>,
}

impl App {
	/**
	 * Create a new App object.
	 *
	 * @param  args
	 * @return App
	 */
	pub fn new(args: ArgMatches<'static>) -> Self {
		Self { args }
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
		let recorder = Recorder::new(
			self.args
				.value_of("fps")
				.unwrap_or_default()
				.parse()
				.unwrap_or(10),
			get_image,
		);
		if self.args.is_present("command") {
			let record = recorder.record_async();
			Command::get(&self.args)
				.execute()
				.expect("Failed to run the command");
			record.finish().expect("Failed to finish the recording");
			record.thread.join().expect("Failed to retrieve the frames")
		} else {
			recorder.record_sync()
		}
	}

	/**
	 * Save frames as a GIF file.
	 *
	 * @param  frames
	 * @param  geometry
	 * @return Result
	 */
	pub fn save_gif(
		&self,
		frames: Vec<Frame>,
		geometry: Geometry,
	) -> Result<(), Error> {
		let mut gif = Gif::new(
			File::create(self.get_output_file()).expect("Failed to create file"),
			geometry,
			Gif::get_settings(&self.args),
		)?;
		gif.save(frames)?;
		Ok(())
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
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn test_app_mod() -> Result<(), Error> {
		let app = App::new(util::parse_args());
		let geometry = Geometry::new(0, 0, 1, 1);
		let frames = app.record(move || Some(Image::new(vec![0, 0, 0], geometry)));
		app.save_gif(frames, geometry)?;
		Command::new(String::from("rm"), vec![String::from("t.gif")]).execute()?;
		Ok(())
	}
}
