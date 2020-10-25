#![allow(clippy::tabs_in_doc_comments)]
#![warn(rust_2018_idioms)]

#[macro_use]
extern crate log;

mod analyze;
mod app;
mod args;
mod edit;
mod file;
mod gif;
mod image;
mod record;
mod settings;
mod util;
mod x11;
use self::app::{App, AppResult, WindowAccess};
use self::args::matches::ArgMatches;
use self::args::Args;
use self::settings::AppSettings;
use self::x11::WindowSystem;

fn main() -> AppResult {
	util::check_friday();
	let args = Args::parse();
	let matches = ArgMatches::new(&args);
	let mut settings = AppSettings::new(&matches);
	if !matches.is_present("quiet") {
		util::init_logger(
			settings.args.occurrences_of("verbose"),
			&settings.save.file.format,
			settings.get_main_color(),
		)
		.expect("Failed to initialize the logger");
	}
	settings.check();
	let window = if settings.window_required {
		WindowSystem::init(&settings)
			.expect("Failed to access the window system")
			.get_window()
	} else {
		None
	};
	App::new(window, &settings).start()
}
