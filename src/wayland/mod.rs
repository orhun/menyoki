pub mod display;
pub mod window;

use crate::record::settings::RecordWindow;
use crate::settings::AppSettings;
use crate::wayland::display::Display;
use crate::wayland::window::Window;
use crate::window::Access;
use std::env;

/* Environment variable for overriding the detected window system */
const WINDOW_SYSTEM_VAR: &str = concat!(env!("CARGO_PKG_NAME"), "_WINDOW_SYSTEM");

/**
 * Check if the Wayland backend should be used.
 *
 * @return bool
 */
pub fn is_preferred() -> bool {
	match env::var(WINDOW_SYSTEM_VAR.to_uppercase()) {
		Ok(value) => value.eq_ignore_ascii_case("wayland"),
		Err(_) => env::var_os("WAYLAND_DISPLAY").is_some(),
	}
}

/* Wayland window system */
pub struct WindowSystem<'a> {
	display: &'static Display,
	settings: &'a AppSettings<'a>,
}

impl<'a> Access<'a, Window> for WindowSystem<'a> {
	/**
	 * Initialize the Wayland window system.
	 *
	 * @param  settings
	 * @return WindowSystem (Option)
	 */
	fn init(settings: &'a AppSettings<'a>) -> Option<Self> {
		Display::open(settings.record).map(|display| Self {
			display: Box::leak(Box::new(display)),
			settings,
		})
	}

	/**
	 * Get the output to record.
	 *
	 * @return Window (Option)
	 */
	fn get_window(&mut self) -> Option<Window> {
		debug!("Record window: {:?}", self.settings.record.window);
		let size = match self.settings.record.window {
			RecordWindow::Root(size) => size,
			RecordWindow::Focus(_, _) => {
				error!(
					"Wayland does not allow capturing an individual window, \
					only a whole output."
				);
				error!(
					"Use --root to capture an output, \
					along with --monitor/--size to narrow it down."
				);
				return None;
			}
		};
		if self.settings.record.flag.mouse {
			warn!("Selecting a window with the mouse is not supported on Wayland.");
		}
		if self.settings.args.is_present("record")
			&& self.settings.record.flag.action_keys.is_some()
		{
			warn!(
				"Action keys are not supported on Wayland, \
				use --duration or press Ctrl-C to stop recording."
			);
		}
		let output = match self.settings.record.flag.monitor {
			Some(monitor) => monitor.saturating_sub(1),
			None => {
				if self.display.outputs.len() > 1 {
					warn!(
						"{} outputs found, capturing the first one. \
						Use --monitor to capture another one.",
						self.display.outputs.len()
					);
				}
				0
			}
		};
		let mut geometry = match self.display.outputs.get(output) {
			Some(output) => output.geometry,
			None => {
				error!(
					"Invalid monitor number: {} (found {} outputs)",
					output + 1,
					self.display.outputs.len()
				);
				return None;
			}
		};
		let mut padding = self.settings.record.padding;
		if let Some(size) = size.filter(|size| !size.is_zero()) {
			padding.right = size
				.width
				.checked_add(padding.left)
				.and_then(|width| geometry.width.checked_sub(width))
				.unwrap_or_default();
			padding.bottom = size
				.height
				.checked_add(padding.top)
				.and_then(|height| geometry.height.checked_sub(height))
				.unwrap_or_default();
		}
		let area = geometry.with_padding(padding);
		if area.width == 0 || area.height == 0 {
			error!("The capture area is empty, check the size and padding values.");
			return None;
		}
		let window = Window::new(self.display, output, geometry, area);
		debug!("Selected output: {}", output);
		info!("{}", window);
		Some(window)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use pretty_assertions::assert_eq;
	#[test]
	fn test_wayland_backend_selection() {
		let variable = WINDOW_SYSTEM_VAR.to_uppercase();
		assert_eq!("MENYOKI_WINDOW_SYSTEM", variable);
		env::set_var(&variable, "x11");
		env::set_var("WAYLAND_DISPLAY", "wayland-0");
		assert!(!is_preferred());
		env::set_var(&variable, "wayland");
		assert!(is_preferred());
		env::remove_var(&variable);
		assert!(is_preferred());
		env::remove_var("WAYLAND_DISPLAY");
		assert!(!is_preferred());
	}
}
