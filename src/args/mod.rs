pub mod parser;
use clap::{App, Arg, ArgMatches, SubCommand};

/* Command-line arguments */
pub struct Args<'a, 'b> {
	save: App<'a, 'b>,
	gif: App<'a, 'b>,
	record: App<'a, 'b>,
}

/* Default initialization values for Args */
impl Default for Args<'_, '_> {
	fn default() -> Self {
		Self {
			save: Self::get_save_args(),
			gif: Self::get_gif_args(),
			record: Self::get_record_args(),
		}
	}
}

impl<'a, 'b> Args<'a, 'b>
where
	'a: 'b,
{
	/**
	 * Parse command line arguments.
	 *
	 * @return ArgMatches
	 */
	pub fn parse() -> ArgMatches<'a> {
		let args = Self::default();
		App::new(env!("CARGO_PKG_NAME"))
			.version(env!("CARGO_PKG_VERSION"))
			.author(env!("CARGO_PKG_AUTHORS"))
			.about(env!("CARGO_PKG_DESCRIPTION"))
			.arg(
				Arg::with_name("command")
					.value_name("COMMAND")
					.help("Sets the command to run"),
			)
			.subcommand(args.record.subcommand(args.gif.subcommand(args.save)))
			.get_matches()
	}

	/**
	 * Get save subcommand arguments.
	 *
	 * @return App
	 */
	fn get_save_args() -> App<'a, 'b> {
		SubCommand::with_name("save")
			.about("Changes the output file settings")
			.arg(
				Arg::with_name("output")
					.value_name("FILE")
					.default_value("t.gif")
					.help("Sets the output file"),
			)
			.arg(
				Arg::with_name("date")
					.short("d")
					.long("date")
					.help("Adds date and time to the file name"),
			)
			.arg(
				Arg::with_name("timestamp")
					.short("t")
					.long("timestamp")
					.help("Adds timestamp to the file name"),
			)
			.arg(
				Arg::with_name("prompt")
					.short("p")
					.long("prompt")
					.help("Shows prompt for the file name input"),
			)
	}

	/**
	 * Get gif subcommand arguments.
	 *
	 * @return App
	 */
	fn get_gif_args() -> App<'a, 'b> {
		SubCommand::with_name("gif")
			.about("Changes the GIF encoder settings")
			.arg(
				Arg::with_name("speed")
					.short("s")
					.long("speed")
					.value_name("SPEED")
					.default_value("10")
					.help("Sets the frame encoding speed (1-30)")
					.takes_value(true),
			)
			.arg(
				Arg::with_name("repeat")
					.short("r")
					.long("repeat")
					.value_name("REPEAT")
					.help("Sets the number of repetitions [default: \u{221E}]")
					.takes_value(true),
			)
	}

	/**
	 * Get record subcommand arguments.
	 *
	 * @return App
	 */
	fn get_record_args() -> App<'a, 'b> {
		SubCommand::with_name("record")
			.about("Changes the recording settings")
			.arg(
				Arg::with_name("root")
					.short("r")
					.long("root")
					.help("Records the root window"),
			)
			.arg(
				Arg::with_name("focus")
					.short("w")
					.long("focus")
					.conflicts_with("root")
					.help("Records the focus window"),
			)
			.arg(
				Arg::with_name("countdown")
					.short("c")
					.long("countdown")
					.value_name("S")
					.default_value("3")
					.help("Sets the countdown value for recording")
					.takes_value(true),
			)
			.arg(
				Arg::with_name("color")
					.short("x")
					.long("color")
					.value_name("HEX")
					.default_value("FF00FF")
					.help("Sets the main color to use")
					.takes_value(true),
			)
			.arg(
				Arg::with_name("fps")
					.short("f")
					.long("fps")
					.value_name("FPS")
					.default_value("10")
					.help("Sets the FPS (frames per second) value")
					.takes_value(true),
			)
			.arg(
				Arg::with_name("interval")
					.short("i")
					.long("interval")
					.value_name("MS")
					.default_value("10")
					.help("Sets the interval time for window selection")
					.takes_value(true),
			)
			.arg(
				Arg::with_name("border")
					.short("b")
					.long("border")
					.value_name("BORDER")
					.default_value("5")
					.help("Sets the border padding value")
					.takes_value(true),
			)
			.arg(
				Arg::with_name("padding")
					.short("p")
					.long("padding")
					.value_name("PADDING")
					.default_value("\"0:0:0:0\"")
					.help("Sets the recording area padding value")
					.takes_value(true),
			)
			.arg(
				Arg::with_name("timeout")
					.short("t")
					.long("timeout")
					.value_name("S")
					.default_value("30")
					.help("Sets the timeout for window selection")
					.takes_value(true),
			)
	}
}
