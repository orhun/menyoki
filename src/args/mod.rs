pub mod parser;
use crate::util::file::{File, FileFormat};
use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};

#[derive(Debug, PartialEq)]
enum GifMode {
	Record,
	Edit,
	Make,
}

/* Command-line arguments */
pub struct Args<'a, 'b> {
	record: App<'a, 'b>,
	capture: App<'a, 'b>,
	edit: App<'a, 'b>,
	split: App<'a, 'b>,
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
			record: Self::get_record_args(false),
			capture: Self::get_record_args(true),
			edit: Self::get_edit_args(),
			split: Self::get_split_args(),
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
			.settings(if !cfg!(test) {
				&[AppSettings::SubcommandRequiredElseHelp]
			} else {
				&[]
			})
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
			.arg(
				Arg::with_name("ignored")
					.long("ignored")
					.help("[Placeholder argument for ignored tests]")
					.hidden(true),
			)
			.subcommand(
				args.record
					.subcommand(
						Self::get_gif_args(GifMode::Record)
							.subcommand(Self::get_save_args(FileFormat::Gif)),
					)
					.subcommand(Self::get_save_args(FileFormat::Gif)),
			)
			.subcommand(Self::get_image_args(args.capture, Vec::new()))
			.subcommand(Self::get_image_args(
				args.edit.subcommand(
					Self::get_gif_args(GifMode::Edit)
						.subcommand(Self::get_save_args(FileFormat::Gif)),
				),
				Vec::new(),
			))
			.subcommand(Self::get_image_args(args.split, vec![AppSettings::Hidden]))
			.subcommand(
				Self::get_gif_args(GifMode::Make)
					.subcommand(Self::get_save_args(FileFormat::Gif)),
			)
			.get_matches()
	}

	/**
	 * Get record/capture subcommand arguments.
	 *
	 * @param  capture_mode
	 * @return App
	 */
	fn get_record_args(capture_mode: bool) -> App<'a, 'b> {
		SubCommand::with_name(if capture_mode { "capture" } else { "record" })
			.about(if capture_mode {
				"Takes the screenshot of a window"
			} else {
				"Records a window"
			})
			.arg(
				Arg::with_name("command")
					.value_name("COMMAND")
					.help("Sets the command to run"),
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
					.default_value("T:R:B:L")
					.help(if capture_mode {
						"Sets the capture area padding value"
					} else {
						"Sets the record area padding value"
					})
					.takes_value(true),
			)
			.arg(
				Arg::with_name("select")
					.short("s")
					.long("select")
					.value_name("SIZE")
					.default_value("W:H")
					.help(if capture_mode {
						"Sets the capture area size and enables selection"
					} else {
						"Sets the record area size and enables selection"
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
					.hidden(capture_mode),
			)
			.arg(
				Arg::with_name("countdown")
					.short("c")
					.long("countdown")
					.value_name("S")
					.default_value_if("command", None, "3")
					.default_value(if capture_mode { "0" } else { "3" })
					.help(if capture_mode {
						"Sets the countdown value for capturing"
					} else {
						"Sets the countdown value for recording"
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
				if capture_mode {
					"Captures the root window"
				} else {
					"Records the root window"
				},
			))
			.arg(
				Arg::with_name("focus")
					.short("w")
					.long("focus")
					.conflicts_with("root")
					.help(if capture_mode {
						"Captures the focus window"
					} else {
						"Records the focus window"
					}),
			)
			.arg(
				Arg::with_name("with-alpha")
					.short("a")
					.long("with-alpha")
					.help(if capture_mode {
						"Captures with the alpha channel for transparency"
					} else {
						"Records with the alpha channel for transparency"
					}),
			)
			.arg(
				Arg::with_name("no-border")
					.long("no-border")
					.help("Shows no border for window selection"),
			)
			.arg(
				Arg::with_name("no-keys")
					.long("no-keys")
					.help("Disables the action keys while recording")
					.hidden(capture_mode),
			)
	}

	/**
	 * Get gif subcommand arguments.
	 *
	 * @param  mode
	 * @return App
	 */
	fn get_gif_args(mode: GifMode) -> App<'a, 'b> {
		SubCommand::with_name(if mode == GifMode::Make { "make" } else { "gif" })
			.about(if mode == GifMode::Make {
				"Makes a GIF from frames"
			} else {
				"Changes the GIF encoder settings"
			})
			.arg(
				Arg::with_name("fps")
					.short("f")
					.long("fps")
					.value_name("FPS")
					.default_value("20")
					.help("Sets the FPS value")
					.hidden(mode == GifMode::Edit)
					.takes_value(true),
			)
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
				Arg::with_name("fast")
					.long("fast")
					.help("Encodes 3 times faster (lower quality and bigger file)")
					.hidden(!cfg!(feature = "ski")),
			)
			.arg(
				Arg::with_name("speed")
					.short("s")
					.long("speed")
					.value_name("SPEED")
					.default_value("1.0")
					.help("Sets the GIF speed")
					.hidden(mode != GifMode::Edit)
					.takes_value(true),
			)
			.arg(
				Arg::with_name("cut-begin")
					.long("cut-begin")
					.value_name("S")
					.default_value("0.0")
					.help("Cuts the beginning of the GIF")
					.hidden(mode != GifMode::Edit)
					.takes_value(true),
			)
			.arg(
				Arg::with_name("cut-end")
					.long("cut-end")
					.value_name("S")
					.default_value("0.0")
					.help("Cuts the end of the GIF")
					.hidden(mode != GifMode::Edit)
					.takes_value(true),
			)
			.arg(
				Arg::with_name("frames")
					.value_name("FRAMES")
					.help("Sets the frames of the GIF")
					.min_values(1)
					.hidden(mode != GifMode::Make)
					.default_value_if("dir", None, "-")
					.required(mode == GifMode::Make)
					.empty_values(false)
					.takes_value(true),
			)
			.arg(
				Arg::with_name("no-sort")
					.short("n")
					.long("no-sort")
					.help("Uses frames in the order given")
					.hidden(mode != GifMode::Make),
			)
			.arg(
				Arg::with_name("dir")
					.short("d")
					.long("dir")
					.conflicts_with("frames")
					.value_name("DIRECTORY")
					.help("Sets the directory to read frames")
					.hidden(mode != GifMode::Make)
					.takes_value(true),
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
					.help("Converts image to the given format"),
			)
			.arg(
				Arg::with_name("crop")
					.long("crop")
					.value_name("PADDING")
					.default_value("T:R:B:L")
					.help("Applies the given padding to crop the image")
					.takes_value(true),
			)
			.arg(
				Arg::with_name("resize")
					.long("resize")
					.value_name("SIZE")
					.default_value("W:H")
					.help("Changes the image size and aspect ratio")
					.takes_value(true),
			)
			.arg(
				Arg::with_name("ratio")
					.long("ratio")
					.value_name("RATIO")
					.default_value("1.0")
					.help("Resizes the image by changing the aspect ratio")
					.takes_value(true),
			)
			.arg(
				Arg::with_name("flip")
					.long("flip")
					.value_name("FLIP")
					.help("Flips the image")
					.possible_values(&["horizontal", "vertical"])
					.takes_value(true),
			)
			.arg(
				Arg::with_name("rotate")
					.long("rotate")
					.value_name("ROTATE")
					.help("Rotates the image clockwise")
					.possible_values(&["90", "180", "270"])
					.takes_value(true),
			)
			.arg(
				Arg::with_name("blur")
					.long("blur")
					.value_name("SIGMA")
					.default_value("0.0")
					.help("Blurs the image")
					.takes_value(true),
			)
			.arg(
				Arg::with_name("grayscale")
					.long("grayscale")
					.help("Converts image to grayscale"),
			)
			.arg(
				Arg::with_name("invert")
					.long("invert")
					.help("Inverts the image colors"),
			)
			.arg(
				Arg::with_name("brighten")
					.long("brighten")
					.value_name("BRIGHTNESS")
					.default_value("0")
					.help("Brightens the image")
					.allow_hyphen_values(true)
					.takes_value(true),
			)
			.arg(
				Arg::with_name("hue-rotate")
					.long("hue-rotate")
					.value_name("HUE")
					.default_value("0")
					.help("Hue rotates the image")
					.allow_hyphen_values(true)
					.takes_value(true),
			)
			.arg(
				Arg::with_name("contrast")
					.long("contrast")
					.value_name("CONTRAST")
					.default_value("0.0")
					.help("Adjusts the contrast of the image")
					.allow_hyphen_values(true)
					.takes_value(true),
			)
			.arg(
				Arg::with_name("filter")
					.long("filter")
					.value_name("FILTER")
					.possible_values(&[
						"nearest",
						"triangle",
						"catmull-rom",
						"gaussian",
						"lanczos3",
					])
					.default_value("lanczos3")
					.help("Sets the sampling filter")
					.takes_value(true),
			)
	}

	/**
	 * Get the gif split arguments.
	 *
	 * @return App
	 */
	fn get_split_args() -> App<'a, 'b> {
		SubCommand::with_name("split")
			.about("Splits a GIF into frames")
			.arg(
				Arg::with_name("file")
					.value_name("FILE")
					.help("Sets the GIF file to split")
					.required(true),
			)
			.arg(
				Arg::with_name("dir")
					.short("d")
					.long("dir")
					.value_name("DIRECTORY")
					.help("Sets the output directory")
					.takes_value(true),
			)
	}

	/**
	 * Add image related subcommands to the given arguments.
	 *
	 * @param  args
	 * @param  save_settings
	 * @return App
	 */
	fn get_image_args(
		args: App<'a, 'b>,
		save_settings: Vec<AppSettings>,
	) -> App<'a, 'b> {
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
			).subcommand(Self::get_save_args(FileFormat::Png).settings(&save_settings)),
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
		).subcommand(Self::get_save_args(FileFormat::Jpg).settings(&save_settings)))
		.subcommand(
			SubCommand::with_name("bmp")
				.about("Changes the BMP encoder settings")
				.subcommand(Self::get_save_args(FileFormat::Bmp).settings(&save_settings)),
		)
		.subcommand(
			SubCommand::with_name("tiff")
				.about("Changes the TIFF encoder settings")
				.subcommand(Self::get_save_args(FileFormat::Tiff).settings(&save_settings)),
		)
		.subcommand(
			SubCommand::with_name("ff")
				.about("Changes the farbfeld encoder settings")
				.subcommand(Self::get_save_args(FileFormat::Ff).settings(&save_settings)),
		)
		.subcommand(Self::get_save_args(FileFormat::Any).settings(&save_settings))
	}

	/**
	 * Get save subcommand arguments.
	 *
	 * @param  file_format
	 * @return App
	 */
	fn get_save_args(file_format: FileFormat) -> App<'a, 'b> {
		SubCommand::with_name("save")
			.about("Changes the output file settings")
			.arg(
				Arg::with_name("output")
					.value_name("FILE")
					.default_value(
						Box::leak(
							File::from_format(file_format).path.into_boxed_path(),
						)
						.to_str()
						.unwrap_or_default(),
					)
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
					.help("Shows input prompt for the file name"),
			)
	}
}
