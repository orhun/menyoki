use crate::args::matches::ArgMatches;
use image::pnm::PnmSubtype;
use std::fmt;
use std::str::FromStr;

/* Format of the output file */
#[derive(Clone, Debug, PartialEq)]
pub enum FileFormat {
	Any,
	Gif,
	Png,
	Jpg,
	Bmp,
	Ico,
	Tiff,
	Tga,
	Pnm(String),
	Ff,
	Txt,
}

/* Display implementation for user-facing output */
impl fmt::Display for FileFormat {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{:?}", self)
	}
}

/* Implementation for parsing FileFormat from a string */
impl FromStr for FileFormat {
	type Err = &'static str;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"gif" => Ok(Self::Gif),
			"png" => Ok(Self::Png),
			"jpg" => Ok(Self::Jpg),
			"bmp" => Ok(Self::Bmp),
			"ico" => Ok(Self::Ico),
			"tiff" => Ok(Self::Tiff),
			"tga" => Ok(Self::Tga),
			"pnm" => Ok(Self::Pnm(String::from("ppm"))),
			"ff" => Ok(Self::Ff),
			"txt" => Ok(Self::Txt),
			_ => Err("Unrecognized file format"),
		}
	}
}

impl FileFormat {
	/**
	 * Create a FileFormat enum fron parsed arguments.
	 *
	 * @param  args
	 * @param  pnm_subtype
	 * @return FileFormat
	 */
	pub fn from_args<'a>(
		args: &'a ArgMatches<'a>,
		pnm_subtype: Option<PnmSubtype>,
	) -> Self {
		match args.subcommand_matches(if args.is_present("edit") {
			"edit"
		} else if args.is_present("split") {
			"split"
		} else if args.is_present("analyze") {
			"analyze"
		} else {
			"capture"
		}) {
			Some(matches) => {
				if args.is_present("analyze") {
					if matches.is_present("save") {
						Self::Txt
					} else {
						Self::Any
					}
				} else if matches.is_present("gif") {
					Self::Gif
				} else if matches.is_present("ff") {
					Self::Ff
				} else if matches.is_present("tiff") {
					Self::Tiff
				} else if matches.is_present("tga") {
					Self::Tga
				} else if matches.is_present("pnm") {
					Self::Pnm(String::from(match pnm_subtype {
						Some(PnmSubtype::Bitmap(_)) => "pbm",
						Some(PnmSubtype::Graymap(_)) => "pgm",
						Some(PnmSubtype::Pixmap(_)) => "ppm",
						Some(PnmSubtype::ArbitraryMap) => "pam",
						None => "pnm",
					}))
				} else if matches.is_present("bmp") {
					Self::Bmp
				} else if matches.is_present("ico") {
					Self::Ico
				} else if matches.is_present("jpg") {
					Self::Jpg
				} else {
					Self::Png
				}
			}
			None => Self::Gif,
		}
	}

	/**
	 * Get extension from format.
	 *
	 * @return String
	 */
	pub fn to_extension(&self) -> String {
		match self {
			Self::Any => String::from("*"),
			Self::Pnm(v) => v.to_string(),
			_ => self.to_string(),
		}
		.to_lowercase()
	}
}
