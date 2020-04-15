mod util;
mod x11;

use self::x11::display::Display;

fn main() {
	let _args = util::parse_args();
	println!("thank god it's friday");

	let display = Display::open().expect("Cannot open display.");
}
