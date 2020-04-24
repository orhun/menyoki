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

fn main() -> Result<(), std::io::Error> {
	let _args = util::parse_args();
	println!("thank god it's friday");

	let display = Display::open().expect("Cannot open display");
	let mut focused_window = display.get_focused_window();
	focused_window.reset_position();
	let mut gif = Gif::new(
		File::create("target/test.gif").expect("Failed to create file"),
		focused_window.geometry,
		15,
		Repeat::Infinite,
	)?;

	let recorder = Recorder::new(20);
	let record = recorder.record(move || focused_window.get_image());
	util::exec_cmd("kmon", &["-c", "blue"]).expect("Failed to run the command");
	record.finish().expect("Failed to finish the recording");
	match record.thread.join() {
		Ok(frames) => {
			println!("frames: {}", frames.len());
			gif.save(frames)?;
		}
		Err(_) => panic!("Failed to retrieve the frames"),
	};
	Ok(())
}
