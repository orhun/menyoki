pub mod window;

use crate::settings::AppSettings;
use crate::window::Access;
use crate::ws::window::Window;

/* Window system implementation */
pub struct WindowSystem {}

impl<'a> Access<'a, Window> for WindowSystem {
	/**
	 * Initialize the window system.
	 *
	 * @param  settings
	 * @return WindowSystem (Option)
	 */
	fn init(_settings: &'a AppSettings<'a>) -> Option<Self> {
		unimplemented!()
	}

	/**
	 * Get the window to record/capture.
	 *
	 * @return Window (Option)
	 */
	fn get_window(&mut self) -> Option<Window> {
		unimplemented!()
	}
}
