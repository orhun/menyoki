use crate::image::gif::{Frame, FrameSettings, Gif};
use crate::image::{Geometry, Image};
use crate::record::Recorder;
use crate::util;
use chrono::Local;
use clap::ArgMatches;
use std::io::Error;
use std::fs::File;

pub struct App {
	args: ArgMatches<'static>,
}

impl App {
	pub fn new(args: ArgMatches<'static>) -> Self {
		Self {
			args
		}
	}

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
		);
		let record = recorder.record(get_image);
		util::exec_cmd(
			"sh",
			&[
				"-c",
				self.args
					.value_of("command")
					.expect("No command specified to run"),
			],
		)
        .expect("Failed to run the command");
        record.finish().expect("Failed to finish the recording");
	    record.thread.join().expect("Failed to retrieve the frames")
    }

    pub fn save_gif(&self, frames: Vec<Frame>, geometry: Geometry) -> Result<(), Error> {
        let mut gif = Gif::new(
            File::create(self.get_output_file()).expect("Failed to create file"),
            geometry,
            self.get_frame_settings(),
        )?;
        gif.save(frames)?;
        Ok(())
    }

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

	fn get_frame_settings(&self) -> FrameSettings {
		match self.args.subcommand_matches("gif") {
			Some(matches) => FrameSettings::new(
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
			None => FrameSettings::new(-1, 10),
		}
	}
}
