mod app;
mod cmd;
mod image;
mod record;
mod util;
mod x11;
use self::app::App;
use self::image::Capture;
use self::x11::display::Display;

fn main() -> Result<(), std::io::Error> {
	let args = util::parse_args();
	println!("thank god it's friday");

	let display = Display::open().expect("Cannot open display");
	let mut focused_window = display.get_focused_window();
	focused_window.reset_position();

	let app = App::new(args.clone());
	let frames = app.record(move || focused_window.get_image());
	println!("frames: {}", frames.len());
	app.save_gif(frames, focused_window.geometry)?;

	Ok(())
}
