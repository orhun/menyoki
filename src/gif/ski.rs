use crate::app::AppResult;
use crate::gif::encoder::{Encoder, EncoderConfig};
use crate::image::Image;
use crate::util::state::InputState;
use gifski::{Collector, Repeat, Writer};
use std::convert::TryInto;
use std::io::{self, Write};
use std::thread;

/* GIF encoder and settings */
pub struct GifskiEncoder<Output: Write> {
	fps: u32,
	collector: Collector,
	writer: Writer,
	output: Output,
}

impl<'a, Output: Write> Encoder<'a, Output> for GifskiEncoder<Output> {
	/**
	 * Create a new GifskiEncoder object.
	 *
	 * @param  config
	 * @return GifskiEncoder (Result)
	 */
	fn new(config: EncoderConfig<'a, Output>) -> AppResult<Self> {
		let (collector, writer) = gifski::new(gifski::Settings {
			width: Some(config.geometry.width),
			height: Some(config.geometry.height),
			quality: config.settings.quality,
			fast: config.settings.gifski.1,
			repeat: match config.settings.repeat {
				n if n >= 0 => Repeat::Finite(n.try_into().unwrap_or_default()),
				_ => Repeat::Infinite,
			},
		})?;
		Ok(Self {
			fps: config.fps,
			collector,
			writer,
			output: config.output,
		})
	}

	/**
	 * Encode images as frame and write to the GIF file.
	 *
	 * @param  images
	 * @param  input_state (Option)
	 * @param  Result
	 */
	fn save(
		self,
		images: Vec<Image>,
		input_state: Option<&'static InputState>,
	) -> AppResult<()> {
		let fps = self.fps;
		let mut collector = self.collector;
		let collector_thread = thread::spawn(move || {
			for (i, image) in images.iter().enumerate() {
				let percentage = ((i + 1) as f64 / images.len() as f64) * 100.;
				info!("Saving... ({:.1}%)\r", percentage);
				debug!(
					"Encoding... ({:.1}%) [{}/{}]\r",
					percentage,
					i + 1,
					images.len()
				);
				io::stdout().flush().expect("Failed to flush stdout");
				if let Some(state) = input_state {
					if state.check_cancel_keys() {
						info!("\n");
						warn!("User interrupt detected.");
						panic!("Failed to write the frames")
					}
				}
				collector
					.add_frame_rgba(i, image.get_img_vec(), i as f64 / fps as f64)
					.expect("Failed to collect a frame");
			}
			info!("\n");
		});
		self.writer
			.write(self.output, &mut gifski::progress::NoProgress {})?;
		collector_thread
			.join()
			.expect("Failed to collect the frames");
		Ok(())
	}
}
