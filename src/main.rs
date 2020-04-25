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

fn main() -> Result<(), std::io::Error> {
	let args = util::parse_args();
	println!("thank god it's friday");

	let display = Display::open().expect("Cannot open display");
	let mut focused_window = display.get_focused_window();
	focused_window.reset_position();
	let mut gif = Gif::new(
		File::create(match args.subcommand_matches("save") {
			Some(matches) => matches.value_of("output").unwrap_or_default(),
			_ => "t.gif",
		})
		.expect("Failed to create file"),
		focused_window.geometry,
		15,
		Repeat::Infinite,
	)?;

	let recorder = Recorder::new(20);
	let record = recorder.record(move || focused_window.get_image());
	util::exec_cmd("kmon", &["-c", "blue"]).expect("Failed to run the command");
	record.finish().expect("Failed to finish the recording");
	let frames = record.thread.join().expect("Failed to retrieve the frames");
	println!("frames: {}", frames.len());
	gif.save(frames)?;
	Ok(())
}
