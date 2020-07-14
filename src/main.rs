#![allow(clippy::tabs_in_doc_comments)]
#![warn(rust_2018_idioms)]

#[macro_use]
extern crate log;
mod app;
mod args;
mod gif;
mod image;
mod record;
mod settings;
mod test;
mod util;
mod x11;
use self::app::{App, WindowAccess};
use self::args::Args;
use self::settings::AppSettings;
use self::x11::WindowSystem;
use std::fs::File;
use std::io::Error;

fn main() -> Result<(), Error> {
	let args = Args::parse();
	util::check_friday();
	if !args.is_present("quiet") {
		util::init_logger(args.occurrences_of("verbose"))
			.expect("Failed to initialize the logger");
	}
	let settings = AppSettings::new(&args);
	let mut window_system =
		WindowSystem::init(&settings).expect("Failed to access the window system");
	App::new(
		window_system
			.get_window()
			.expect("Failed to get the window"),
		&settings,
	)
	.start(
		File::create(&settings.save.file.path).expect("Failed to create the file"),
	)
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	#[should_panic]
	fn test_main() {
		main().unwrap();
	}
}
