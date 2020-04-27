mod image;
mod record;
mod util;
mod x11;
use self::image::gif::Gif;
use self::image::Capture;
use self::record::Recorder;
use self::x11::display::Display;
use chrono::Local;
use gif::Repeat;
use rprompt;
use std::fs::File;

fn main() -> Result<(), std::io::Error> {
	let args = util::parse_args();
	println!("thank god it's friday");

	let output_file = match args.subcommand_matches("save") {
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
	};

	let fps = args
		.value_of("fps")
		.unwrap_or_default()
		.parse()
		.unwrap_or(10);
	let command = args
		.value_of("command")
		.expect("No command specified to run");

	let speed;
	let repeat;

	match args.subcommand_matches("gif") {
		Some(matches) => {
			speed = matches
				.value_of("speed")
				.unwrap_or_default()
				.parse()
				.unwrap_or(10);
			repeat = match matches.value_of("repeat") {
				Some(num) => Repeat::Finite(num.parse().unwrap_or(1)),
				None => Repeat::Infinite,
			}
		}
		None => {
			speed = 10;
			repeat = Repeat::Infinite;
		}
	}

	let display = Display::open().expect("Cannot open display");
	let mut focused_window = display.get_focused_window();
	focused_window.reset_position();
	let geometry = focused_window.geometry;
	let get_image = move || focused_window.get_image();

	let mut gif = Gif::new(
		File::create(output_file).expect("Failed to create file"),
		geometry,
		speed,
		repeat,
	)?;

	let recorder = Recorder::new(fps);
	let record = recorder.record(get_image);
	util::exec_cmd("sh", &["-c", command]).expect("Failed to run the command");
	record.finish().expect("Failed to finish the recording");
	let frames = record.thread.join().expect("Failed to retrieve the frames");
	println!("frames: {}", frames.len());
	gif.save(frames)?;
	Ok(())
}
