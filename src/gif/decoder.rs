use crate::gif::settings::EditSettings;
use gif::{
	Decoder as GifDecoder, DecodingError, Encoder, Frame, Reader, Repeat,
	SetParameter,
};
use std::convert::TryInto;
use std::io::{Read, Write};

/* GIF decoder and settings */
pub struct Decoder<'a, Input: Read, Output: Write> {
	reader: Reader<Input>,
	output: Output,
	settings: EditSettings<'a>,
}

impl<'a, Input: Read, Output: Write> Decoder<'a, Input, Output> {
	/**
	 * Create a new Decoder object.
	 *
	 * @param  input
	 * @param  output
	 * @param  settings
	 * @return Result
	 */
	pub fn new(
		input: Input,
		output: Output,
		settings: EditSettings<'a>,
	) -> Result<Self, DecodingError> {
		Ok(Self {
			reader: GifDecoder::new(input).read_info()?,
			output,
			settings,
		})
	}

	/**
	 * Get a GIF encoder from a sample frame.
	 *
	 * @param  sample_frame
	 * @return Result
	 */
	fn get_encoder(
		self,
		sample_frame: &Frame<'_>,
	) -> Result<Encoder<Output>, DecodingError> {
		let mut encoder =
			Encoder::new(self.output, sample_frame.width, sample_frame.height, &[])?;
		encoder.set(match self.settings.repeat {
			n if n >= 0 => Repeat::Finite(n.try_into().unwrap_or_default()),
			_ => Repeat::Infinite,
		})?;
		Ok(encoder)
	}

	/**
	 * Update the frames and save the file.
	 *
	 * @return Result
	 */
	pub fn edit(mut self) -> Result<(), DecodingError> {
		let mut frames = Vec::new();
		while let Some(frame) = self.reader.read_next_frame()? {
			let mut frame = frame.clone();
			info!(
				"{:?} {} {} {} {}",
				frame.delay, frame.top, frame.left, frame.width, frame.height,
			);
			frames.push(frame);
		}
		let mut encoder =
			self.get_encoder(frames.first().expect("No frames found to edit"))?;
		for frame in frames {
			encoder.write_frame(&frame)?;
		}
		Ok(())
	}
}
