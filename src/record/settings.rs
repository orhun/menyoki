#[derive(Debug)]
pub struct RecordSettings {
	pub fps: u32,
	pub timeout: u128,
	pub interval: u64,
	pub countdown: u64,
	pub color: u64,
}

impl RecordSettings {
	pub fn new(
		fps: u32,
		timeout: u128,
		interval: u64,
		countdown: u64,
		color: u64,
	) -> Self {
		Self {
			fps,
			timeout,
			interval,
			countdown,
			color,
		}
	}
}
