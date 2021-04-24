#![allow(clippy::tabs_in_doc_comments)]
#![allow(clippy::manual_map)]
#![warn(rust_2018_idioms)]

#[macro_use]
extern crate log;

mod analyze;
mod anim;
mod apng;
mod app;
mod args;
mod edit;
mod file;
mod gif;
mod image;
mod record;
mod settings;
mod util;
mod window;
#[cfg(not(all(unix, not(target_os = "macos"))))]
mod ws;
#[cfg(all(unix, not(target_os = "macos")))]
mod x11;
use self::app::{App, AppResult};
use self::args::matches::ArgMatches;
use self::args::Args;
use self::settings::AppSettings;
use self::util::logger::Logger;
use self::window::Access;
#[cfg(not(all(unix, not(target_os = "macos"))))]
use self::ws::WindowSystem;
#[cfg(all(unix, not(target_os = "macos")))]
use self::x11::WindowSystem;

fn main() -> AppResult {
	let args = Args::parse();
	let matches = ArgMatches::new(&args);
	let mut settings = AppSettings::new(&matches);
	Logger::new(&settings)
		.init()
		.expect("Failed to initialize the logger");
	settings.check();
	let window = if settings.window_required {
		if let Some(window) = WindowSystem::init(&settings)
			.expect("Failed to access the window system")
			.get_window()
		{
			Some(window)
		} else {
			return Ok(());
		}
	} else {
		None
	};
	App::new(window, &settings).start()
}
