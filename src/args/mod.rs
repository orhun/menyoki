pub mod parser;
use crate::util::file::File;
use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};

/* Main commands of the app */
#[derive(Debug, PartialEq)]
enum BaseCommand {
	Record,
	Capture,
}

/* Command-line arguments */
pub struct Args<'a, 'b> {
	record: App<'a, 'b>,
	capture: App<'a, 'b>,
	edit: App<'a, 'b>,
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
			record: Self::get_base_args(BaseCommand::Record),
			capture: Self::get_base_args(BaseCommand::Capture),
			edit: Self::get_edit_args(),
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
			.global_settings(&[
				AppSettings::ColorAuto,
				AppSettings::ColoredHelp,
				AppSettings::InferSubcommands,
				AppSettings::VersionlessSubcommands,
				AppSettings::DeriveDisplayOrder,
			])
			.setting(AppSettings::SubcommandRequiredElseHelp)
			.arg(
				Arg::with_name("command")
					.value_name("COMMAND")
					.help("Sets the command to run"),
			)
			.arg(
				Arg::with_name("verbose")
					.short("v")
					.long("verbose")
					.help("Increases the logging verbosity")
					.multiple(true)
					.display_order(1000),
			)
			.arg(
				Arg::with_name("quiet")
					.short("q")
					.long("quiet")
					.help("Shows no output")
					.display_order(1001),
			)
			.subcommand(
				args.record
					.subcommand(
						Self::get_gif_args(false)
							.subcommand(Self::get_save_args("t.gif")),
					)
					.subcommand(Self::get_save_args("t.*")),
			)
			.subcommand(Self::get_image_args(args.capture))
			.subcommand(Self::get_image_args(args.edit.subcommand(
				Self::get_gif_args(true).subcommand(Self::get_save_args("t.gif")),
			)))
			.get_matches()
	}

	/**
	 * Get the main subcommand arguments from BaseCommand.
	 *
	 * @param  base_command
	 * @return App
	 */
	fn get_base_args(base_command: BaseCommand) -> App<'a, 'b> {
		SubCommand::with_name(&format!("{:?}", base_command).to_lowercase())
			.about(match base_command {
				BaseCommand::Record => "Records a window",
				BaseCommand::Capture => "Takes the screenshot of a window",
			})
			.arg(
				Arg::with_name("fps")
					.short("f")
					.long("fps")
					.value_name("FPS")
					.default_value("10")
					.help("Sets the FPS (frames per second) value")
					.takes_value(true)
					.hidden(base_command == BaseCommand::Capture),
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
				Arg::with_name("border")
					.short("b")
					.long("border")
					.value_name("BORDER")
					.default_value("1")
					.help("Sets the border padding value")
					.takes_value(true),
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
					.takes_value(true),
			)
			.arg(
				Arg::with_name("select")
					.short("s")
					.long("select")
					.value_name("SIZE")
					.default_value("W:H")
					.help(match base_command {
						BaseCommand::Record => {
							"Sets the record area size and enables selection"
						}
						BaseCommand::Capture => {
							"Sets the capture area size and enables selection"
						}
					})
					.takes_value(true),
			)
			.arg(
				Arg::with_name("duration")
					.short("d")
					.long("duration")
					.value_name("S")
					.default_value("\u{221E}")
					.help("Sets the recording duration")
					.takes_value(true)
					.hidden(base_command == BaseCommand::Capture),
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
					.takes_value(true),
			)
			.arg(
				Arg::with_name("timeout")
					.short("t")
					.long("timeout")
					.value_name("S")
					.default_value("60")
					.help("Sets the timeout for window selection")
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
			.arg(Arg::with_name("root").short("r").long("root").help(
				match base_command {
					BaseCommand::Record => "Records the root window",
					BaseCommand::Capture => "Captures the root window",
				},
			))
			.arg(
				Arg::with_name("focus")
					.short("w")
					.long("focus")
					.conflicts_with("root")
					.help(match base_command {
						BaseCommand::Record => "Records the focus window",
						BaseCommand::Capture => "Captures the focus window",
					}),
			)
			.arg(
				Arg::with_name("with-alpha")
					.short("a")
					.long("with-alpha")
					.help(match base_command {
						BaseCommand::Record => {
							"Records with the alpha channel for transparency"
						}
						BaseCommand::Capture => {
							"Captures with the alpha channel for transparency"
						}
					}),
			)
			.arg(
				Arg::with_name("no-border")
					.short("n")
					.long("no-border")
					.help("Shows no border for window selection"),
			)
	}

	/**
	 * Get gif subcommand arguments.
	 *
	 * @param  edit_mode
	 * @return App
	 */
	fn get_gif_args(edit_mode: bool) -> App<'a, 'b> {
		SubCommand::with_name("gif")
			.about("Changes the GIF encoder settings")
			.arg(
				Arg::with_name("quality")
					.short("q")
					.long("quality")
					.value_name("QUALITY")
					.default_value("75")
					.help("Sets the frame quality (1-100)")
					.takes_value(true),
			)
			.arg(
				Arg::with_name("repeat")
					.short("r")
					.long("repeat")
					.value_name("REPEAT")
					.default_value("\u{221E}")
					.help("Sets the number of repetitions")
					.takes_value(true),
			)
			.arg(
				Arg::with_name("speed")
					.short("s")
					.long("speed")
					.value_name("SPEED")
					.default_value("1.0")
					.help("Sets the GIF speed")
					.hidden(!edit_mode)
					.takes_value(true),
			)
			.arg(
				Arg::with_name("fast")
					.short("f")
					.long("fast")
					.help("Encodes 3 times faster (lower quality and bigger file)")
					.hidden(!cfg!(feature = "ski") || !edit_mode),
			)
	}

	/**
	 * Get the image editing arguments.
	 *
	 * @return App
	 */
	fn get_edit_args() -> App<'a, 'b> {
		SubCommand::with_name("edit")
			.about("Edits an image/GIF")
			.arg(
				Arg::with_name("input")
					.value_name("FILE")
					.help("Sets the input file path")
					.required(true),
			)
			.arg(
				Arg::with_name("convert")
					.long("convert")
					.help("Converts the image to the given format"),
			)
			.arg(
				Arg::with_name("crop")
					.long("crop")
					.value_name("PADDING")
					.default_value("0:0:0:0")
					.help("Applies the given padding to crop the GIF")
					.takes_value(true),
			)
			.arg(
				Arg::with_name("resize")
					.long("resize")
					.value_name("SIZE")
					.default_value("W:H")
					.help("Changes the GIF size and aspect ratio")
					.takes_value(true),
			)
			.arg(
				Arg::with_name("ratio")
					.long("ratio")
					.value_name("RATIO")
					.default_value("1.0")
					.help("Resizes the GIF by changing the aspect ratio")
					.takes_value(true),
			)
			.arg(
				Arg::with_name("flip")
					.long("flip")
					.value_name("FLIP")
					.help("Flips the GIF")
					.possible_values(&["horizontal", "vertical"])
					.takes_value(true),
			)
			.arg(
				Arg::with_name("rotate")
					.long("rotate")
					.value_name("ROTATE")
					.help("Rotates the GIF clockwise")
					.possible_values(&["90", "180", "270"])
					.takes_value(true),
			)
			.arg(
				Arg::with_name("blur")
					.long("blur")
					.value_name("SIGMA")
					.default_value("0.0")
					.help("Blurs the GIF")
					.takes_value(true),
			)
			.arg(
				Arg::with_name("grayscale")
					.long("grayscale")
					.help("Converts GIF to grayscale"),
			)
			.arg(
				Arg::with_name("invert")
					.long("invert")
					.help("Inverts the GIF colors"),
			)
			.arg(
				Arg::with_name("brighten")
					.long("brighten")
					.value_name("BRIGHTNESS")
					.default_value("0")
					.help("Brightens the GIF")
					.allow_hyphen_values(true)
					.takes_value(true),
			)
			.arg(
				Arg::with_name("hue-rotate")
					.long("hue-rotate")
					.value_name("HUE")
					.default_value("0")
					.help("Hue rotates the GIF")
					.allow_hyphen_values(true)
					.takes_value(true),
			)
			.arg(
				Arg::with_name("contrast")
					.long("contrast")
					.value_name("CONTRAST")
					.default_value("0.0")
					.help("Adjusts the contrast of the GIF")
					.allow_hyphen_values(true)
					.takes_value(true),
			)
	}

	/**
	 * Add image related subcommands to the given arguments.
	 *
	 * @param  args
	 * @return App
	 */
	fn get_image_args(args: App<'a, 'b>) -> App<'a, 'b> {
		args.subcommand(
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
					.takes_value(true),
			)
			.arg(
				Arg::with_name("filter")
					.short("f")
					.long("filter")
					.value_name("FILTER")
					.possible_values(&["none", "sub", "up", "avg", "paeth"])
					.default_value("sub")
					.help("Sets the filter algorithm that processes the image data")
					.takes_value(true),
			).subcommand(Self::get_save_args("t.png")),
		)
		.subcommand(SubCommand::with_name("jpg")
		.about("Changes the JPG encoder settings")
		.arg(
			Arg::with_name("quality")
				.short("q")
				.long("quality")
				.value_name("QUALITY")
				.default_value("90")
				.help("Sets the JPG quality (1-100)")
				.takes_value(true),
		).subcommand(Self::get_save_args("t.jpg")))
		.subcommand(
			SubCommand::with_name("bmp")
				.about("Changes the BMP encoder settings")
				.subcommand(Self::get_save_args("t.bmp")),
		)
		.subcommand(
			SubCommand::with_name("tiff")
				.about("Changes the TIFF encoder settings")
				.subcommand(Self::get_save_args("t.tiff")),
		)
		.subcommand(
			SubCommand::with_name("ff")
				.about("Changes the farbfeld encoder settings")
				.subcommand(Self::get_save_args("t.ff")),
		)
		.subcommand(Self::get_save_args("t.*"))
	}

	/**
	 * Get save subcommand arguments.
	 *
	 * @param  default_file
	 * @return App
	 */
	fn get_save_args(default_file: &'a str) -> App<'a, 'b> {
		let default_path = Box::leak(Box::new(File::get_default_path(default_file)))
			.to_str()
			.unwrap_or_default();
		SubCommand::with_name("save")
			.about("Changes the output file settings")
			.arg(
				Arg::with_name("output")
					.value_name("FILE")
					.default_value(default_path)
					.help("Sets the output file path"),
			)
			.arg(
				Arg::with_name("date")
					.short("d")
					.long("date")
					.value_name("FORMAT")
					.default_value("%Y%m%dT%H%M%S")
					.help("Adds date and time to the file name")
					.takes_value(true),
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
}
