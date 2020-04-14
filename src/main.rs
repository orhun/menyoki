mod util;
mod x11;

use self::x11::Handler;

fn main() {
	let _args = util::parse_args();
	println!("thank god it's friday");

	let x11_handler = Handler::new().expect("Failed to create X11 handler");
}
