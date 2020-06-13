use crate::args::parser::ArgParser;
use image::png::CompressionType;
use image::png::FilterType;

/* PNG compression and filter settings */
#[derive(Clone, Copy, Debug)]
pub struct PngSettings {
	pub compression: CompressionType,
	pub filter: FilterType,
}

/* Default initialization values for PngSettings */
impl Default for PngSettings {
	fn default() -> Self {
		Self {
			compression: CompressionType::Fast,
			filter: FilterType::Sub,
		}
	}
}

impl PngSettings {
	/**
	 * Create a new PngSettings object.
	 *
	 * @param  compression
	 * @param  filter
	 * @return PngSettings
	 */
	pub fn new(compression: CompressionType, filter: FilterType) -> Self {
		Self {
			compression,
			filter,
		}
	}

	/**
	 * Create a PngSettings object from parsed arguments.
	 *
	 * @param  parser
	 * @return PngSettings
	 */
	pub fn from_args(parser: ArgParser<'_>) -> Self {
		match parser.args {
			Some(matches) => Self::new(
				match matches.value_of("compression") {
					Some("default") => CompressionType::Default,
					Some("fast") => CompressionType::Fast,
					Some("best") => CompressionType::Best,
					Some("huffman") => CompressionType::Huffman,
					Some("rle") => CompressionType::Rle,
					_ => CompressionType::Fast,
				},
				match matches.value_of("filter") {
					Some("none") => FilterType::NoFilter,
					Some("sub") => FilterType::Sub,
					Some("up") => FilterType::Up,
					Some("avg") => FilterType::Avg,
					Some("paeth") => FilterType::Paeth,
					_ => FilterType::Sub,
				},
			),
			None => PngSettings::default(),
		}
	}
}
