#![allow(clippy::tabs_in_doc_comments)]
#![warn(rust_2018_idioms)]

#[macro_use]
extern crate log;

mod app;
mod args;
mod edit;
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
use std::io::Error;

fn main() -> Result<(), Error> {
	let args = Args::parse();
	let settings = AppSettings::new(&args);
	if !args.is_present("quiet") {
		util::init_logger(args.occurrences_of("verbose"), settings.save.file.format)
			.expect("Failed to initialize the logger");
	}
	util::check_friday();
	settings.check();
	let window = if args.is_present("record") || args.is_present("capture") {
		WindowSystem::init(&settings)
			.expect("Failed to access the window system")
			.get_window()
	} else {
		None
	};
	App::new(window, &settings).start()
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
