pub mod parser;
use clap::{App, Arg, ArgMatches, SubCommand};
use std::fmt;

/* Main SubCommand for the app */
#[derive(Debug)]
enum BaseCommand {
	Record,
	Capture,
}

impl<'a> BaseCommand {
	/**
	 * Get the description of a BaseCommand.
	 *
	 * @return str
	 */
	fn get_description(&self) -> &'a str {
		match self {
			Self::Record => "Records a window",
			Self::Capture => "Takes a screenshot of a window",
		}
	}
}

/* Display implementation for user-facing output */
impl fmt::Display for BaseCommand {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"{}",
			match self {
				Self::Record => "record",
				Self::Capture => "capture",
			}
		)
	}
}

/* Command-line arguments */
pub struct Args<'a, 'b> {
	record: App<'a, 'b>,
	capture: App<'a, 'b>,
	gif: App<'a, 'b>,
	png: App<'a, 'b>,
}

impl<'a, 'b> Args<'a, 'b>
where
	'a: 'b,
{
	/**
	 * Initialize the arguments for parsing.
	 *
	 * @return Args
	 */
	fn init() -> Self {
		Self {
			record: Self::get_record_args(),
			capture: Self::get_capture_args(),
			gif: Self::get_gif_args(),
			png: Self::get_png_args(),
		}
	}

	/**
	 * Parse command line arguments.
	 *
	 * @return ArgMatches
	 */
	pub fn parse() -> ArgMatches<'a> {
		let args = Self::init();
		App::new(env!("CARGO_PKG_NAME"))
			.version(env!("CARGO_PKG_VERSION"))
			.author(env!("CARGO_PKG_AUTHORS"))
			.about(env!("CARGO_PKG_DESCRIPTION"))
			.arg(
				Arg::with_name("command")
					.value_name("COMMAND")
					.help("Sets the command to run"),
			)
			.subcommand(
				args.record
					.subcommand(args.gif.subcommand(Self::get_save_args("t.gif"))),
			)
			.subcommand(
				args.capture
					.subcommand(args.png.subcommand(Self::get_save_args("t.png"))),
			)
			.get_matches()
	}

	/**
	 * Get record subcommand arguments.
	 *
	 * @return App
	 */
	fn get_record_args() -> App<'a, 'b> {
		Self::get_base_args(BaseCommand::Record).arg(
			Arg::with_name("fps")
				.short("f")
				.long("fps")
				.value_name("FPS")
				.default_value("10")
				.help("Sets the FPS (frames per second) value")
				.takes_value(true)
				.display_order(0),
		)
	}

	/**
	 * Get capture subcommand arguments.
	 *
	 * @return App
	 */
	fn get_capture_args() -> App<'a, 'b> {
		Self::get_base_args(BaseCommand::Capture)
	}

	/**
	 * Get the main subcommand arguments from BaseCommand.
	 *
	 * @param  base_command
	 * @return App
	 */
	fn get_base_args(base_command: BaseCommand) -> App<'a, 'b> {
		SubCommand::with_name(&base_command.to_string())
			.about(base_command.get_description())
			.arg(
				Arg::with_name("color")
					.short("x")
					.long("color")
					.value_name("HEX")
					.default_value("FF00FF")
					.help("Sets the main color to use")
					.takes_value(true)
					.display_order(1),
			)
			.arg(
				Arg::with_name("border")
					.short("b")
					.long("border")
					.value_name("BORDER")
					.default_value("5")
					.help("Sets the border padding value")
					.takes_value(true)
					.display_order(2),
			)
			.arg(
				Arg::with_name("padding")
					.short("p")
					.long("padding")
					.value_name("PADDING")
					.default_value("0:0:0:0")
					.help(match base_command {
						BaseCommand::Record => "Sets the record area padding value",
						BaseCommand::Capture => {
							"Sets the capture area padding value"
						}
					})
					.takes_value(true)
					.display_order(3),
			)
			.arg(
				Arg::with_name("countdown")
					.short("c")
					.long("countdown")
					.value_name("S")
					.default_value(match base_command {
						BaseCommand::Record => "3",
						BaseCommand::Capture => "0",
					})
					.help(match base_command {
						BaseCommand::Record => {
							"Sets the countdown value for recording"
						}
						BaseCommand::Capture => {
							"Sets the countdown value for capturing"
						}
					})
					.takes_value(true)
					.display_order(4),
			)
			.arg(
				Arg::with_name("timeout")
					.short("t")
					.long("timeout")
					.value_name("S")
					.default_value("30")
					.help("Sets the timeout for window selection")
					.takes_value(true)
					.display_order(5),
			)
			.arg(
				Arg::with_name("interval")
					.short("i")
					.long("interval")
					.value_name("MS")
					.default_value("10")
					.help("Sets the interval time for window selection")
					.takes_value(true)
					.display_order(6),
			)
			.arg(
				Arg::with_name("root")
					.short("r")
					.long("root")
					.help(match base_command {
						BaseCommand::Record => "Records the root window",
						BaseCommand::Capture => "Captures the root window",
					})
					.display_order(1),
			)
			.arg(
				Arg::with_name("focus")
					.short("w")
					.long("focus")
					.conflicts_with("root")
					.help(match base_command {
						BaseCommand::Record => "Records the focus window",
						BaseCommand::Capture => "Captures the focus window",
					})
					.display_order(2),
			)
			.arg(
				Arg::with_name("no-border")
					.short("n")
					.long("no-border")
					.help("Shows no border for window selection")
					.display_order(3),
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
	 * Get png subcommand arguments.
	 *
	 * @return App
	 */
	fn get_png_args() -> App<'a, 'b> {
		SubCommand::with_name("png")
			.about("Changes the PNG encoder settings")
			.arg(
				Arg::with_name("compression")
					.short("c")
					.long("compress")
					.value_name("COMPRESSION")
					.possible_values(&["default", "fast", "best", "huffman", "rle"])
					.default_value("fast")
					.help("Sets the compression level of PNG encoder")
					.takes_value(true)
					.display_order(1),
			)
			.arg(
				Arg::with_name("filter")
					.short("f")
					.long("filter")
					.value_name("FILTER")
					.possible_values(&["none", "sub", "up", "avg", "paeth"])
					.default_value("sub")
					.help("Sets the filter algorithm that processes the image data")
					.takes_value(true)
					.display_order(2),
			)
	}

	/**
	 * Get save subcommand arguments.
	 *
	 * @param  default_file
	 * @return App
	 */
	fn get_save_args(default_file: &'a str) -> App<'a, 'b> {
		SubCommand::with_name("save")
			.about("Changes the output file settings")
			.arg(
				Arg::with_name("output")
					.value_name("FILE")
					.default_value(default_file)
					.help("Sets the output file"),
			)
			.arg(
				Arg::with_name("date")
					.short("d")
					.long("date")
					.help("Adds date and time to the file name")
					.display_order(1),
			)
			.arg(
				Arg::with_name("timestamp")
					.short("t")
					.long("timestamp")
					.help("Adds timestamp to the file name")
					.display_order(2),
			)
			.arg(
				Arg::with_name("prompt")
					.short("p")
					.long("prompt")
					.help("Shows prompt for the file name input")
					.display_order(3),
			)
	}
}
