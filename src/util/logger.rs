use crate::file::format::FileFormat;
use crate::settings::AppSettings;
use chrono::Local;
use colored::Color;
use fern::colors::ColoredLevelConfig;
use fern::{Dispatch, Output};
use log::{LevelFilter, SetLoggerError};

/* Logger with settings */
#[derive(Clone, Copy)]
pub struct Logger<'a> {
	settings: &'a AppSettings<'a>,
	colors: ColoredLevelConfig,
	level_filter: LevelFilter,
}

impl<'a> Logger<'a> {
	/**
	 * Create a new Logger object.
	 *
	 * @param  settings
	 * @return Logger
	 */
	pub fn new(settings: &'a AppSettings<'a>) -> Self {
		Self {
			settings,
			colors: ColoredLevelConfig::new()
				.info(settings.get_main_color().unwrap_or(Color::Magenta))
				.error(Color::Red)
				.warn(Color::Yellow)
				.debug(Color::Blue)
				.trace(Color::BrightBlack),
			level_filter: if settings.args.is_present("quiet") {
				LevelFilter::Off
			} else {
				match settings.args.occurrences_of("verbose") {
					0 => LevelFilter::Info,
					1 => LevelFilter::Debug,
					_ => LevelFilter::Trace,
				}
			},
		}
	}

	/**
	 * Initialize the logger.
	 *
	 * @return Result
	 */
	pub fn init(&self) -> Result<(), SetLoggerError> {
		let colors = self.colors;
		let mut logger = Dispatch::new()
			.format(move |out, message, record| {
				let time = Local::now().format("%FT%T");
				let color = colors.color(record.level());
				let target = record.target();
				let message = message.to_string();
				if message == "\n" {
					out.finish(format_args!("\n"))
				} else if message == "\r" {
					out.finish(format_args!("\r"))
				} else if message.ends_with('#') {
					out.finish(format_args!("{}", &message[..message.len() - 1]))
				} else if message.ends_with('\r') {
					out.finish(format_args!(
						"\r[{} {} {}] {}",
						time,
						color,
						target,
						&message[..message.len() - 1]
					))
				} else {
					out.finish(format_args!(
						"[{} {} {}] {}\n",
						time, color, target, message
					))
				}
			})
			.chain(Output::stdout(""))
			.level(self.level_filter);
		if self.settings.save.file.format == FileFormat::Gif {
			logger = logger.level_for(
				format!("{}::edit", env!("CARGO_PKG_NAME")),
				LevelFilter::Warn,
			)
		}
		logger.apply()
	}
}
