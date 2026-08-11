pub mod display;
pub mod window;

use crate::image::geometry::Geometry;
use crate::record::settings::RecordWindow;
use crate::settings::AppSettings;
use crate::wayland::display::{Display, Target};
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
		let (target, size) =
			match self.settings.record.window {
				RecordWindow::Root(size) => (self.get_output()?, size),
				RecordWindow::Focus(size, parent) => {
					if parent {
						warn!("Capturing the parent window is not supported on Wayland.");
					}
					(self.get_toplevel()?, size)
				}
			};
		let mut geometry = self.get_geometry(target)?;
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
		let window = Window::new(self.display, target, geometry, area);
		debug!("Selected target: {:?}", target);
		info!("{}", window);
		Some(window)
	}
}

impl WindowSystem<'_> {
	/**
	 * Get the output to capture, selected via the monitor flag.
	 *
	 * @return Target (Option)
	 */
	fn get_output(&self) -> Option<Target> {
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
		if output >= self.display.outputs.len() {
			error!(
				"Invalid monitor number: {} (found {} outputs)",
				output + 1,
				self.display.outputs.len()
			);
			return None;
		}
		Some(Target::Output(output))
	}

	/**
	 * Get the focused window to capture.
	 *
	 * @return Target (Option)
	 */
	fn get_toplevel(&self) -> Option<Target> {
		if !self.display.has_toplevel_capture() {
			error!(
				"This compositor does not allow capturing an individual window, \
				only a whole output."
			);
			error!(
				"Use --root to capture an output, \
				along with --monitor/--size to narrow it down."
			);
			return None;
		}
		match self.display.get_active_toplevel() {
			Some(toplevel) => Some(Target::Toplevel(toplevel)),
			None => {
				error!("No focused window found to capture.");
				None
			}
		}
	}

	/**
	 * Get the size of the target to capture.
	 *
	 * @param  target
	 * @return Geometry (Option)
	 */
	fn get_geometry(&self, target: Target) -> Option<Geometry> {
		match target {
			Target::Output(output) => self
				.display
				.outputs
				.get(output)
				.map(|output| output.geometry),
			Target::Toplevel(toplevel) => {
				match self.display.get_toplevel_geometry(toplevel) {
					Ok(geometry) => Some(geometry),
					Err(e) => {
						error!("Failed to get the size of the window: {}", e);
						None
					}
				}
			}
		}
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
