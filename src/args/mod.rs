pub mod matches;
pub mod parser;
use crate::file::format::FileFormat;
use clap::{App, AppSettings, Arg, ArgMatches, Shell, SubCommand};
use std::io;
use std::str::FromStr;

/* Help template for the main arguments */
const HELP_TEMPLATE: &str = "
  -/+o+:.  -- ./+oo+:`    {bin} {version}
`ossssssso-o+ossssssss`   {about}
 .::::::-.-s`..-::::-.    {author}
          .s`
           :o`
            o-
       `:+osd+:`
     :hMMMNNNMMMh:
    yMMmhssyysydMMy`
   sMMhshNMMMNdsyMMs
   mMNssMMMMMMMhsdMN
   hMMysmMMMMMNysNMd
   .NMNhsydddhsymMN-
    `yMMNmdhhdNMMy.
      `/ydmmmdy/`\n
{usage}\n
{all-args}";

/* Gif related subcommands */
#[derive(Debug, PartialEq)]
enum GifMode {
	Record,
	Edit,
	Make,
}

/* Command line arguments */
pub struct Args<'a, 'b> {
	record: App<'a, 'b>,
	split: App<'a, 'b>,
	make: App<'a, 'b>,
	capture: App<'a, 'b>,
	edit: App<'a, 'b>,
	analyze: App<'a, 'b>,
	misc: App<'a, 'b>,
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
		Self::get_app().get_matches()
	}

	/**
	 * Generate completions for the specified shell.
	 *
	 * @param shell
	 */
	pub fn gen_completions(shell: &'a str) {
		if let Ok(shell) = Shell::from_str(shell) {
			Self::get_app().gen_completions_to(
				env!("CARGO_PKG_NAME"),
				shell,
				&mut io::stdout(),
			);
		}
	}

	/**
	 * Initialize the arguments for parsing.
	 *
	 * @return Args
	 */
	fn init() -> Self {
		Self {
			record: Self::get_record_args(false),
			split: Self::get_split_args(),
			make: Self::get_gif_args(GifMode::Make),
			capture: Self::get_record_args(true),
			edit: Self::get_edit_args(),
			analyze: Self::get_analyze_args(),
			misc: Self::get_misc_args(),
		}
	}

	/**
	 * Get the main clap application.
	 *
	 * @return App
	 */
	fn get_app() -> App<'a, 'b> {
		let args = Self::init();
		App::new(env!("CARGO_PKG_NAME"))
			.version(env!("CARGO_PKG_VERSION"))
			.author(env!("CARGO_PKG_AUTHORS"))
			.about(env!("CARGO_PKG_DESCRIPTION"))
			.template(HELP_TEMPLATE)
			.help_message("Print help information")
			.version_message("Print version information")
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
					.help("Increase logging verbosity")
					.multiple(true)
					.display_order(1000),
			)
			.arg(
				Arg::with_name("quiet")
					.short("q")
					.long("quiet")
					.help("Do not show output")
					.display_order(1001),
			)
			.arg(
				Arg::with_name("config")
					.short("c")
					.long("config")
					.value_name("FILE")
					.help("Set the configuration file")
					.takes_value(true),
			)
			.arg(
				Arg::with_name("color")
					.long("color")
					.value_name("HEX")
					.default_value("3AA431")
					.help("Set the main color")
					.takes_value(true),
			)
			.subcommand(
				args.record
					.subcommand(
						Self::get_gif_args(GifMode::Record)
							.subcommand(Self::get_save_args(FileFormat::Gif)),
					)
					.subcommand(Self::get_save_args(FileFormat::Gif)),
			)
			.subcommand(Self::get_image_args(args.split, false))
			.subcommand(args.make.subcommand(Self::get_save_args(FileFormat::Gif)))
			.subcommand(Self::get_image_args(args.capture, true))
			.subcommand(Self::get_image_args(
				args.edit.subcommand(
					Self::get_gif_args(GifMode::Edit)
						.subcommand(Self::get_save_args(FileFormat::Gif)),
				),
				true,
			))
			.subcommand(
				args.analyze
					.subcommand(Self::get_save_args(FileFormat::Txt)),
			)
			.subcommand(args.misc)
	}

	/**
	 * Get record/capture subcommand arguments.
	 *
	 * @param  capture
	 * @return App
	 */
	fn get_record_args(capture: bool) -> App<'a, 'b> {
		SubCommand::with_name(if capture { "capture" } else { "record" })
			.about(if capture {
				"Capture an image"
			} else {
				"Record a GIF"
			})
			.aliases(if capture { &["screenshot", "ss"] } else { &[] })
			.help_message("Print help information")
			.arg(
				Arg::with_name("command")
					.value_name("COMMAND")
					.help("Set the command to run"),
			)
			.arg(
				Arg::with_name("root")
					.short("r")
					.long("root")
					.help(if capture {
						"Capture the root window"
					} else {
						"Record the root window"
					}),
			)
			.arg(
				Arg::with_name("focus")
					.short("f")
					.long("focus")
					.conflicts_with("root")
					.help(if capture {
						"Capture the focused window"
					} else {
						"Record the focused window"
					}),
			)
			.arg(Arg::with_name("select").long("select").help(if capture {
				"Select the window to capture"
			} else {
				"Select the window to record"
			}))
			.arg(
				Arg::with_name("with-alpha")
					.long("with-alpha")
					.help(if capture {
						"Capture with the alpha channel"
					} else {
						"Record with the alpha channel"
					}),
			)
			.arg(
				Arg::with_name("no-borders")
					.long("no-borders")
					.help("Do not show borders on the focused window"),
			)
			.arg(
				Arg::with_name("no-keys")
					.long("no-keys")
					.help("Disable the action keys while recording")
					.hidden(capture),
			)
			.arg(
				Arg::with_name("keys")
					.short("k")
					.long("keys")
					.value_name("KEYS")
					.default_value("LAlt-S/Enter")
					.help("Set the action keys")
					.takes_value(true),
			)
			.arg(
				Arg::with_name("border")
					.short("b")
					.long("border")
					.value_name("BORDER")
					.default_value("1")
					.help("Set the border padding")
					.takes_value(true),
			)
			.arg(
				Arg::with_name("padding")
					.short("p")
					.long("padding")
					.value_name("T:R:B:L")
					.help(if capture {
						"Set the capture area padding"
					} else {
						"Set the record area padding"
					})
					.takes_value(true),
			)
			.arg(
				Arg::with_name("size")
					.short("s")
					.long("size")
					.value_name("WxH")
					.help(if capture {
						"Set the capture area size"
					} else {
						"Set the record area size"
					})
					.empty_values(true)
					.takes_value(true),
			)
			.arg(
				Arg::with_name("duration")
					.short("d")
					.long("duration")
					.value_name("S")
					.default_value("\u{221E}")
					.help("Set the duration for recording")
					.takes_value(true)
					.hidden(capture),
			)
			.arg(
				Arg::with_name("countdown")
					.short("c")
					.long("countdown")
					.value_name("S")
					.default_value_if("command", None, "3")
					.default_value(if capture { "0" } else { "3" })
					.help(if capture {
						"Set the countdown before capturing"
					} else {
						"Set the countdown before recording"
					})
					.takes_value(true),
			)
			.arg(
				Arg::with_name("timeout")
					.short("t")
					.long("timeout")
					.value_name("S")
					.default_value("60")
					.help("Set the timeout for window selection")
					.takes_value(true),
			)
			.arg(
				Arg::with_name("interval")
					.short("i")
					.long("interval")
					.value_name("MS")
					.default_value("10")
					.help("Set the refresh interval for window selection")
					.takes_value(true),
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
				"Make a GIF from frames"
			} else {
				"Use the GIF encoder"
			})
			.help_message("Print help information")
			.aliases(if mode == GifMode::Make {
				&["combine"]
			} else {
				&[]
			})
			.arg(
				Arg::with_name("fps")
					.short("f")
					.long("fps")
					.value_name("FPS")
					.default_value("20")
					.help("Set the FPS")
					.hidden(mode == GifMode::Edit)
					.takes_value(true),
			)
			.arg(
				Arg::with_name("quality")
					.short("q")
					.long("quality")
					.value_name("QUALITY")
					.default_value("75")
					.help("Set the frame quality (1-100)")
					.takes_value(true),
			)
			.arg(
				Arg::with_name("repeat")
					.short("r")
					.long("repeat")
					.value_name("REPEAT")
					.default_value("\u{221E}")
					.help("Set the number of repetitions")
					.takes_value(true),
			)
			.arg(
				Arg::with_name("fast")
					.long("fast")
					.help("Encode 3 times faster")
					.hidden(!cfg!(feature = "ski")),
			)
			.arg(
				Arg::with_name("speed")
					.short("s")
					.long("speed")
					.value_name("SPEED")
					.default_value("1.0")
					.help("Set the GIF speed")
					.hidden(mode != GifMode::Edit)
					.takes_value(true),
			)
			.arg(
				Arg::with_name("cut-beginning")
					.long("cut-beginning")
					.value_name("S")
					.default_value("0.0")
					.help("Cut the beginning of the GIF")
					.hidden(mode != GifMode::Edit)
					.takes_value(true),
			)
			.arg(
				Arg::with_name("cut-end")
					.long("cut-end")
					.value_name("S")
					.default_value("0.0")
					.help("Cut the end of the GIF")
					.hidden(mode != GifMode::Edit)
					.takes_value(true),
			)
			.arg(
				Arg::with_name("frames")
					.value_name("FRAMES")
					.help("Set the GIF frames")
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
					.help("Use frames in the order given")
					.hidden(mode != GifMode::Make),
			)
			.arg(
				Arg::with_name("dir")
					.short("d")
					.long("dir")
					.conflicts_with("frames")
					.value_name("DIRECTORY")
					.help("Set the directory to read frames")
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
			.help_message("Print help information")
			.about("Edit an image")
			.arg(
				Arg::with_name("file")
					.value_name("FILE")
					.help("Set the input file")
					.required(true),
			)
			.arg(
				Arg::with_name("convert")
					.long("convert")
					.help("Convert image using the encoder given"),
			)
			.arg(
				Arg::with_name("grayscale")
					.long("grayscale")
					.help("Convert image to grayscale"),
			)
			.arg(
				Arg::with_name("invert")
					.long("invert")
					.help("Invert the image colors"),
			)
			.arg(
				Arg::with_name("crop")
					.long("crop")
					.value_name("T:R:B:L")
					.help("Apply padding to crop the image")
					.takes_value(true),
			)
			.arg(
				Arg::with_name("resize")
					.long("resize")
					.value_name("WxH")
					.help("Resize the image without keeping the aspect ratio")
					.takes_value(true),
			)
			.arg(
				Arg::with_name("ratio")
					.long("ratio")
					.value_name("RATIO")
					.default_value("1.0")
					.help("Resize the image by changing the aspect ratio")
					.takes_value(true),
			)
			.arg(
				Arg::with_name("rotate")
					.long("rotate")
					.value_name("ROTATE")
					.help("Rotate the image (clockwise)")
					.possible_values(&["90", "180", "270"])
					.takes_value(true),
			)
			.arg(
				Arg::with_name("flip")
					.long("flip")
					.value_name("FLIP")
					.help("Flip the image")
					.possible_values(&["horizontal", "vertical"])
					.takes_value(true),
			)
			.arg(
				Arg::with_name("blur")
					.long("blur")
					.value_name("SIGMA")
					.default_value("0.0")
					.help("Blur the image")
					.takes_value(true),
			)
			.arg(
				Arg::with_name("hue")
					.long("hue")
					.value_name("HUE")
					.default_value("\u{00B1}0")
					.help("Adjust the hue of the image")
					.allow_hyphen_values(true)
					.takes_value(true),
			)
			.arg(
				Arg::with_name("contrast")
					.long("contrast")
					.value_name("CONTRAST")
					.default_value("\u{00B1}0.0")
					.help("Adjust the contrast of the image")
					.allow_hyphen_values(true)
					.takes_value(true),
			)
			.arg(
				Arg::with_name("brightness")
					.long("brightness")
					.value_name("BRIGHTNESS")
					.default_value("\u{00B1}0")
					.help("Adjust the brightness of the image")
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
					.help("Set the sampling filter for scaling")
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
			.about("Split a GIF into frames")
			.help_message("Print help information")
			.alias("extract")
			.arg(
				Arg::with_name("file")
					.value_name("FILE")
					.help("Set the GIF file")
					.required(true),
			)
			.arg(
				Arg::with_name("dir")
					.short("d")
					.long("dir")
					.value_name("DIRECTORY")
					.help("Set the output directory")
					.takes_value(true),
			)
	}

	/**
	 * Get the image analysis arguments.
	 *
	 * @return App
	 */
	fn get_analyze_args() -> App<'a, 'b> {
		SubCommand::with_name("analyze")
			.about("Analyze an image")
			.help_message("Print help information")
			.alias("inspect")
			.arg(
				Arg::with_name("file")
					.value_name("FILE")
					.help("Set the image file")
					.required(true),
			)
			.arg(
				Arg::with_name("time-zone")
					.short("t")
					.long("time-zone")
					.value_name("TIMEZONE")
					.possible_values(&["utc", "local"])
					.default_value("utc")
					.help("Set the time zone of the report")
					.takes_value(true),
			)
			.arg(
				Arg::with_name("timestamp")
					.long("timestamp")
					.help("Use Unix timestamp for report dates"),
			)
	}

	/**
	 * Add image related subcommands to the given arguments.
	 *
	 * @param  args
	 * @param  save
	 * @return App
	 */
	fn get_image_args(args: App<'a, 'b>, save: bool) -> App<'a, 'b> {
		let mut save_settings = Vec::new();
		if !save {
			save_settings.push(AppSettings::Hidden);
		}
		args.subcommand(
			SubCommand::with_name("png")
				.about("Use the PNG encoder")
				.help_message("Print help information")
				.arg(
					Arg::with_name("compression")
						.short("c")
						.long("compression")
						.value_name("COMPRESSION")
						.possible_values(&[
							"default", "fast", "best", "huffman", "rle",
						])
						.default_value("fast")
						.help("Set the compression level")
						.takes_value(true),
				)
				.arg(
					Arg::with_name("filter")
						.short("f")
						.long("filter")
						.value_name("FILTER")
						.possible_values(&["none", "sub", "up", "avg", "paeth"])
						.default_value("sub")
						.help("Set the filter algorithm")
						.takes_value(true),
				)
				.subcommand(
					Self::get_save_args(FileFormat::Png).settings(&save_settings),
				),
		)
		.subcommand(
			SubCommand::with_name("jpg")
				.about("Use the JPG encoder")
				.help_message("Print help information")
				.arg(
					Arg::with_name("quality")
						.short("q")
						.long("quality")
						.value_name("QUALITY")
						.default_value("90")
						.help("Set the image quality (1-100)")
						.takes_value(true),
				)
				.subcommand(
					Self::get_save_args(FileFormat::Jpg).settings(&save_settings),
				),
		)
		.subcommand(
			SubCommand::with_name("bmp")
				.about("Use the BMP encoder")
				.help_message("Print help information")
				.subcommand(
					Self::get_save_args(FileFormat::Bmp).settings(&save_settings),
				),
		)
		.subcommand(
			SubCommand::with_name("ico")
				.about("Use the ICO encoder")
				.help_message("Print help information")
				.subcommand(
					Self::get_save_args(FileFormat::Ico).settings(&save_settings),
				),
		)
		.subcommand(
			SubCommand::with_name("tiff")
				.about("Use the TIFF encoder")
				.help_message("Print help information")
				.subcommand(
					Self::get_save_args(FileFormat::Tiff).settings(&save_settings),
				),
		)
		.subcommand(
			SubCommand::with_name("tga")
				.about("Use the TGA encoder")
				.help_message("Print help information")
				.subcommand(
					Self::get_save_args(FileFormat::Tga).settings(&save_settings),
				),
		)
		.subcommand(
			SubCommand::with_name("pnm")
				.about("Use the PNM encoder")
				.help_message("Print help information")
				.arg(
					Arg::with_name("format")
						.short("f")
						.long("format")
						.value_name("FORMAT")
						.help("Set the PNM format")
						.possible_values(&[
							"bitmap",
							"graymap",
							"pixmap",
							"arbitrary",
						])
						.default_value("pixmap")
						.takes_value(true),
				)
				.arg(
					Arg::with_name("encoding")
						.short("e")
						.long("encoding")
						.value_name("ENCODING")
						.help("Set the encoding for storing the samples")
						.possible_values(&["binary", "ascii"])
						.default_value("binary")
						.takes_value(true),
				)
				.subcommand(
					Self::get_save_args(FileFormat::Pnm(String::from("ppm")))
						.settings(&save_settings),
				),
		)
		.subcommand(
			SubCommand::with_name("ff")
				.about("Use the farbfeld encoder")
				.help_message("Print help information")
				.subcommand(
					Self::get_save_args(FileFormat::Ff).settings(&save_settings),
				),
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
			.about("Save the output file(s)")
			.help_message("Print help information")
			.alias("out")
			.arg(
				Arg::with_name("file")
					.value_name("FILE")
					.default_value(
						Box::leak(file_format.into_file().path.into_boxed_path())
							.to_str()
							.unwrap_or_default(),
					)
					.help("Set the output file"),
			)
			.arg(
				Arg::with_name("with-extension")
					.short("e")
					.long("with-extension")
					.help("Always save the file with an extension"),
			)
			.arg(
				Arg::with_name("timestamp")
					.short("t")
					.long("timestamp")
					.help("Add Unix timestamp to the file name"),
			)
			.arg(
				Arg::with_name("date")
					.short("d")
					.long("date")
					.value_name("FORMAT")
					.default_value("%Y%m%dT%H%M%S")
					.help("Add formatted date/time to the file name")
					.takes_value(true),
			)
	}

	/**
	 * Get misc subcommand arguments.
	 *
	 * @return App
	 */
	fn get_misc_args() -> App<'a, 'b> {
		SubCommand::with_name("misc")
			.about("Perfom miscellaneous operations")
			.help_message("Print help information")
			.setting(AppSettings::Hidden)
			.arg(
				Arg::with_name("gen-completions")
					.short("g")
					.long("gen-completions")
					.value_name("SHELL")
					.help("Generate completions for the specified shell")
					.possible_values(&[
						"bash",
						"fish",
						"zsh",
						"powershell",
						"elvish",
					])
					.takes_value(true),
			)
	}
}
