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
mod view;
#[cfg(all(unix, not(target_os = "macos")))]
mod wayland;
mod window;
#[cfg(not(all(unix, not(target_os = "macos"))))]
mod ws;
#[cfg(all(unix, not(target_os = "macos")))]
mod x11;
use self::app::App;
use self::args::matches::ArgMatches;
use self::args::Args;
use self::settings::AppSettings;
use self::util::logger::Logger;
use self::window::{Access, Capture};
#[cfg(not(all(unix, not(target_os = "macos"))))]
use self::ws::{
	window::Window as DefaultWindow, WindowSystem as DefaultWindowSystem,
};
#[cfg(all(unix, not(target_os = "macos")))]
use self::x11::{
	window::Window as DefaultWindow, WindowSystem as DefaultWindowSystem,
};
use std::fmt::Debug;
use std::process;

/**
 * Retrieve the window from the given window system and start the application.
 *
 * @param settings
 */
fn start<'a, Window, Ws>(settings: &'a AppSettings<'a>)
where
	Window: Capture + Send + Sync + Copy + Debug + 'static,
	Ws: Access<'a, Window>,
{
	let window = if settings.window_required {
		match Ws::init(settings) {
			Some(mut ws) => match ws.get_window() {
				Some(window) => Some(window),
				None => {
					error!("Failed to retrieve the window.");
					process::exit(1);
				}
			},
			None => {
				error!("Failed to access the window system.");
				process::exit(1);
			}
		}
	} else {
		None
	};
	if let Err(e) = App::new(window, settings).start() {
		error!("{}", e);
		process::exit(1);
	}
}

fn main() {
	let args = Args::parse();
	let matches = ArgMatches::new(&args);
	let mut settings = AppSettings::new(&matches);
	Logger::new(&settings)
		.init()
		.expect("Failed to initialize the logger");
	settings.check();
	#[cfg(all(unix, not(target_os = "macos")))]
	if wayland::is_preferred() {
		start::<wayland::window::Window, wayland::WindowSystem<'_>>(&settings);
		return;
	}
	start::<DefaultWindow, DefaultWindowSystem<'_>>(&settings);
}
