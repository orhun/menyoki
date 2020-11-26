use crate::gif::encoder::{Encoder, EncoderConfig};
use crate::image::Image;
use crate::util::state::InputState;
use gifski::{Collector, Writer};
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
	 * @return GifskiEncoder
	 */
	fn new(config: EncoderConfig<'a, Output>) -> Self {
		let (collector, writer) = gifski::new(gifski::Settings {
			width: Some(config.geometry.width),
			height: Some(config.geometry.height),
			quality: config.settings.quality,
			once: config.settings.repeat == 0,
			fast: config.settings.fast,
		})
		.expect("Failed to initialize the gifski encoder");
		Self {
			fps: config.fps,
			collector,
			writer,
			output: config.output,
		}
	}

	/**
	 * Encode images as frame and write to the GIF file.
	 *
	 * @param images
	 * @param input_state (Option)
	 */
	fn save(self, images: Vec<Image>, input_state: Option<&'static InputState>) {
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
			.write(self.output, &mut gifski::progress::NoProgress {})
			.expect("Failed to write the frames");
		collector_thread
			.join()
			.expect("Failed to collect the frames");
	}
}
