use crate::gif::encoder::Encoder;
use crate::gif::settings::GifSettings;
use crate::image::geometry::Geometry;
use crate::image::Image;
use crate::util::state::InputState;
use gifski::{Collector, Writer};
use std::io::{self, Error, Write};
use std::thread;

/* GIF encoder and settings */
pub struct Gif<Output: Write> {
	fps: u32,
	collector: Collector,
	writer: Writer,
	output: Output,
}

impl<Output: Write> Encoder<Output> for Gif<Output> {
	/**
	 * Create a new Gif object.
	 *
	 * @param  geometry
	 * @param  output
	 * @param  fps
	 * @param  settings
	 * @return Result (Gif)
	 */
	fn new(
		geometry: Geometry,
		output: Output,
		fps: u32,
		settings: GifSettings,
	) -> Result<Self, Error> {
		let (collector, writer) = gifski::new(gifski::Settings {
			width: Some(geometry.width),
			height: Some(geometry.height),
			quality: settings.quality,
			once: settings.repeat == 0,
			fast: settings.fast,
		})
		.expect("Failed to initialize the gifski encoder");
		Ok(Self {
			fps,
			collector,
			writer,
			output,
		})
	}

	/**
	 * Encode images as frame and write to the GIF file.
	 *
	 * @param  images
	 * @param  input_state
	 * @return Result
	 */
	fn save(
		self,
		images: Vec<Image>,
		input_state: &'static InputState,
	) -> Result<(), Error> {
		let fps = self.fps;
		let mut collector = self.collector;
		let collector_thread = thread::spawn(move || {
			for i in 0..images.len() {
				let percentage = ((i + 1) as f64 / images.len() as f64) * 100.;
				info!("Encoding... ({:.1}%)\r", percentage);
				debug!(
					"Encoding... ({:.1}%) [{}/{}]\r",
					percentage,
					i + 1,
					images.len()
				);
				io::stdout().flush().expect("Failed to flush stdout");
				if input_state.check_cancel_keys() {
					info!("\n");
					warn!("User interrupt detected.");
					panic!("Failed to write the frames")
				}
				collector
					.add_frame_rgba(
						i,
						images[i].get_img_vec(),
						i as f64 / fps as f64,
					)
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
		Ok(())
	}
}
