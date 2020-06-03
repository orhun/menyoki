#![allow(clippy::tabs_in_doc_comments)]
#![warn(rust_2018_idioms)]

#[macro_use]
extern crate log;
mod app;
mod gif;
mod image;
mod record;
mod settings;
mod test;
mod util;
mod x11;
use self::app::App;
use self::settings::AppSettings;
use self::util::args::Args;
use self::x11::WindowSystem;
use std::io::Error;

fn main() -> Result<(), Error> {
	let args = Args::parse();
	util::init_logger().expect("Failed to initialize the logger");

	println!("thank god it's friday");

	let settings = AppSettings::new(args);
	let app = App::new(settings.clone());
	let window_system = WindowSystem::init(settings).expect("Cannot open display");
	if let Some(window) = window_system.get_record_window() {
		let frames = app.record(window);
		info!("frames: {}", frames.len());
		app.save_gif(frames)?;
	} else {
		error!("Cannot the record the window.")
	}
	Ok(())
}
