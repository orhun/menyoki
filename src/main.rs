mod image;
mod record;
mod util;
mod x11;

use self::image::gif::Gif;
use self::image::Capture;
use self::record::Recorder;
use self::x11::display::Display;
use gif::Repeat;
use std::fs::File;
use std::process::Command;

fn main() {
	let _args = util::parse_args();
	println!("thank god it's friday");

	let display = Display::open().expect("Cannot open display.");
	let mut focused_window = display.get_focused_window();
	focused_window.reset_position();
	let mut gif = Gif::new(
		File::create("target/test.gif").unwrap(),
		focused_window.geometry,
		15,
		Repeat::Infinite,
	)
	.unwrap();

	let recorder = Recorder::new(20);
	let record = recorder.record(move || focused_window.get_image());

	match Command::new("kmon").spawn() {
		Ok(mut child) => {
			child.wait().unwrap();
		}
		Err(_) => panic!("Failed to run the command."),
	}

	record.finish().unwrap();
	let frames = record.thread.join().unwrap();
	println!("frames: {}", frames.len());
	gif.save(frames).unwrap();
}
