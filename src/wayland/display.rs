use crate::image::geometry::Geometry;
use crate::record::settings::RecordSettings;
use image::Rgba;
use std::env;
use std::fmt;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Seek, SeekFrom};
use std::os::fd::AsFd;
use std::os::unix::fs::OpenOptionsExt;
use std::path::PathBuf;
use std::process;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};
use wayland_client::protocol::wl_buffer::WlBuffer;
use wayland_client::protocol::wl_output::{self, WlOutput};
use wayland_client::protocol::wl_registry::{self, WlRegistry};
use wayland_client::protocol::wl_shm::{self, WlShm};
use wayland_client::protocol::wl_shm_pool::WlShmPool;
use wayland_client::{
	delegate_noop, event_created_child, Connection, Dispatch, EventQueue, Proxy,
	QueueHandle, WEnum,
};
use wayland_protocols_hyprland::toplevel_export::v1::client::hyprland_toplevel_export_frame_v1::{
	self, HyprlandToplevelExportFrameV1,
};
use wayland_protocols_hyprland::toplevel_export::v1::client::hyprland_toplevel_export_manager_v1::HyprlandToplevelExportManagerV1;
use wayland_protocols_wlr::foreign_toplevel::v1::client::zwlr_foreign_toplevel_handle_v1::{
	self, State as ToplevelState, ZwlrForeignToplevelHandleV1,
};
use wayland_protocols_wlr::foreign_toplevel::v1::client::zwlr_foreign_toplevel_manager_v1::{
	self, ZwlrForeignToplevelManagerV1, EVT_TOPLEVEL_OPCODE,
};
use wayland_protocols_wlr::screencopy::v1::client::zwlr_screencopy_frame_v1::{
	self, ZwlrScreencopyFrameV1,
};
use wayland_protocols_wlr::screencopy::v1::client::zwlr_screencopy_manager_v1::ZwlrScreencopyManagerV1;

/* Highest version of the screencopy protocol that is used */
const SCREENCOPY_VERSION: u32 = 3;
/* Highest version of the output interface that is used */
const OUTPUT_VERSION: u32 = 4;
/* Highest version of the foreign toplevel interface that is used */
const TOPLEVEL_VERSION: u32 = 3;
/* Version of the toplevel export protocol that accepts a toplevel handle */
const TOPLEVEL_EXPORT_VERSION: u32 = 2;
/* Number of bytes that a single pixel occupies in a shm buffer */
const PIXEL_SIZE: u32 = 4;

/* Thing to capture the frames of */
#[derive(Clone, Copy, Debug)]
pub enum Target {
	Output(usize),
	Toplevel(usize),
}

/* Result of a frame copy request */
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum FrameStatus {
	#[default]
	Pending,
	Ready,
	Failed,
}

/* Properties of the buffer that a frame is copied into */
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct FrameInfo {
	format: wl_shm::Format,
	width: u32,
	height: u32,
	stride: u32,
}

impl FrameInfo {
	/**
	 * Get the size of the buffer that is described.
	 *
	 * @return usize (Option)
	 */
	fn get_size(&self) -> Option<usize> {
		self.stride
			.checked_mul(self.height)
			.and_then(|size| usize::try_from(size).ok())
			.filter(|size| *size != 0)
	}

	/**
	 * Get the offsets of the (R, G, B) channels in a pixel.
	 *
	 * @return Tuple (Option)
	 */
	fn get_channels(&self) -> Option<(usize, usize, usize)> {
		match self.format {
			wl_shm::Format::Argb8888 | wl_shm::Format::Xrgb8888 => Some((2, 1, 0)),
			wl_shm::Format::Abgr8888 | wl_shm::Format::Xbgr8888 => Some((0, 1, 2)),
			_ => None,
		}
	}

	/**
	 * Check if the format carries a meaningful alpha channel.
	 *
	 * @return bool
	 */
	fn has_alpha(&self) -> bool {
		matches!(
			self.format,
			wl_shm::Format::Argb8888 | wl_shm::Format::Abgr8888
		)
	}
}

/* Name and geometric properties of an output */
#[derive(Clone, Debug)]
pub struct OutputInfo {
	pub name: String,
	pub geometry: Geometry,
}

/* Title and state of a toplevel (window) */
#[derive(Clone, Debug, Default)]
pub struct ToplevelInfo {
	pub title: String,
	pub app_id: String,
	pub activated: bool,
	pub closed: bool,
}

/* Wayland globals and the state of the ongoing frame copy */
#[derive(Debug, Default)]
struct State {
	shm: Option<WlShm>,
	screencopy: Option<ZwlrScreencopyManagerV1>,
	toplevel_export: Option<HyprlandToplevelExportManagerV1>,
	outputs: Vec<(WlOutput, OutputInfo)>,
	toplevels: Vec<(ZwlrForeignToplevelHandleV1, ToplevelInfo)>,
	frame_info: Option<FrameInfo>,
	frame_status: FrameStatus,
	y_invert: bool,
}

/* Shared memory buffer that frames are copied into */
#[derive(Debug)]
struct ShmBuffer {
	file: File,
	pool: WlShmPool,
	buffer: WlBuffer,
	info: FrameInfo,
	size: usize,
}

impl ShmBuffer {
	/* Destroy the buffer and the pool it belongs to. */
	fn destroy(&self) {
		self.buffer.destroy();
		self.pool.destroy();
	}
}

/* Mutable parts of the connection, guarded for thread-safe usage */
#[derive(Debug)]
struct DisplayInner {
	queue: EventQueue<State>,
	state: State,
	buffer: Option<ShmBuffer>,
}

/* Wayland display connection */
pub struct Display {
	inner: Mutex<DisplayInner>,
	pub outputs: Vec<OutputInfo>,
	pub toplevels: Vec<ToplevelInfo>,
	pub settings: RecordSettings,
}

/* Debug implementation for programmer-facing output */
impl fmt::Debug for Display {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("Display")
			.field("outputs", &self.outputs)
			.field("toplevels", &self.toplevels)
			.finish()
	}
}

impl Display {
	/**
	 * Connect to the Wayland compositor and collect the globals.
	 *
	 * @param  settings
	 * @return Display (Option)
	 */
	pub fn open(settings: RecordSettings) -> Option<Self> {
		let connection = match Connection::connect_to_env() {
			Ok(connection) => connection,
			Err(e) => {
				error!("Cannot connect to the Wayland compositor: {}", e);
				return None;
			}
		};
		let mut queue = connection.new_event_queue::<State>();
		let mut state = State::default();
		connection.display().get_registry(&queue.handle(), ());
		/* The first roundtrip binds the globals, the second one collects
		 * the events that are emitted by the newly bound outputs. */
		for _ in 0..2 {
			if let Err(e) = queue.roundtrip(&mut state) {
				error!("Wayland communication failed: {}", e);
				return None;
			}
		}
		if state.screencopy.is_none() {
			error!(
				"The compositor does not support the {} protocol.",
				ZwlrScreencopyManagerV1::interface().name
			);
			error!("Screen capture on Wayland requires a wlroots-based compositor.");
			return None;
		}
		if state.shm.is_none() {
			error!("The compositor does not support shared memory buffers.");
			return None;
		}
		if state.outputs.is_empty() {
			error!("No outputs found to capture.");
			return None;
		}
		let outputs = state
			.outputs
			.iter()
			.map(|(_, info)| info.clone())
			.collect::<Vec<OutputInfo>>();
		let toplevels = state
			.toplevels
			.iter()
			.map(|(_, info)| info.clone())
			.collect::<Vec<ToplevelInfo>>();
		debug!("Outputs: {:?}", outputs);
		debug!("Toplevels: {:?}", toplevels);
		Some(Self {
			inner: Mutex::new(DisplayInner {
				queue,
				state,
				buffer: None,
			}),
			outputs,
			toplevels,
			settings,
		})
	}

	/**
	 * Check if individual windows can be captured.
	 *
	 * @return bool
	 */
	pub fn has_toplevel_capture(&self) -> bool {
		match self.inner.lock() {
			Ok(inner) => inner.state.toplevel_export.is_some(),
			Err(_) => false,
		}
	}

	/**
	 * Get the index of the toplevel that currently has the focus.
	 *
	 * @return usize (Option)
	 */
	pub fn get_active_toplevel(&self) -> Option<usize> {
		self.toplevels.iter().position(|info| info.activated)
	}

	/**
	 * Get the size of a toplevel by asking for a frame and dropping it.
	 *
	 * @param  toplevel
	 * @return Geometry (Result)
	 */
	pub fn get_toplevel_geometry(
		&self,
		toplevel: usize,
	) -> Result<Geometry, String> {
		let mut inner = self
			.inner
			.lock()
			.map_err(|e| format!("Failed to lock the display: {e}"))?;
		let DisplayInner { queue, state, .. } = &mut *inner;
		let handle = queue.handle();
		let frame = Self::export_frame(state, &handle, toplevel)?;
		state.frame_info = None;
		let result = queue
			.roundtrip(state)
			.map_err(|e| format!("Wayland communication failed: {e}"));
		let info = state.frame_info;
		frame.destroy();
		result?;
		let info = info
			.ok_or_else(|| String::from("The compositor did not offer a buffer"))?;
		Ok(Geometry::new(0, 0, info.width, info.height))
	}

	/**
	 * Ask for a frame of the given toplevel.
	 *
	 * @param  state
	 * @param  handle
	 * @param  toplevel
	 * @return HyprlandToplevelExportFrameV1 (Result)
	 */
	fn export_frame(
		state: &State,
		handle: &QueueHandle<State>,
		toplevel: usize,
	) -> Result<HyprlandToplevelExportFrameV1, String> {
		let (toplevel, info) = state
			.toplevels
			.get(toplevel)
			.ok_or_else(|| String::from("Invalid toplevel"))?;
		if info.closed {
			return Err(String::from("The window was closed"));
		}
		let manager = state
			.toplevel_export
			.as_ref()
			.ok_or_else(|| String::from("Missing toplevel export manager"))?;
		Ok(manager.capture_toplevel_with_wlr_toplevel_handle(
			0,
			toplevel,
			handle,
			(),
		))
	}

	/**
	 * Copy the given area of an output into an image.
	 *
	 * @param  output
	 * @param  area
	 * @return Vector of Rgba (Result)
	 */
	pub fn capture(
		&self,
		target: Target,
		area: Geometry,
	) -> Result<Vec<Rgba<u8>>, String> {
		if area.width == 0 || area.height == 0 {
			return Err(String::from("The capture area is empty"));
		}
		let mut inner = self
			.inner
			.lock()
			.map_err(|e| format!("Failed to lock the display: {e}"))?;
		let DisplayInner {
			queue,
			state,
			buffer,
		} = &mut *inner;
		let handle = queue.handle();
		let shm = state
			.shm
			.clone()
			.ok_or_else(|| String::from("Missing shared memory global"))?;
		state.frame_info = None;
		state.frame_status = FrameStatus::Pending;
		state.y_invert = false;
		match target {
			Target::Output(output) => {
				let (output, _) = state
					.outputs
					.get(output)
					.ok_or_else(|| String::from("Invalid output"))?;
				let manager = state
					.screencopy
					.clone()
					.ok_or_else(|| String::from("Missing screencopy manager"))?;
				/* ponytail: the whole output is copied and cropped afterwards,
				 * since the region request of the protocol uses logical
				 * coordinates while the padding of menyoki is given in pixels. */
				let frame = manager.capture_output(0, &output.clone(), &handle, ());
				let result = Self::copy_frame(
					queue,
					state,
					buffer,
					&shm,
					&handle,
					|buffer| frame.copy(buffer),
					area,
				);
				frame.destroy();
				result
			}
			Target::Toplevel(toplevel) => {
				let frame = Self::export_frame(state, &handle, toplevel)?;
				let result = Self::copy_frame(
					queue,
					state,
					buffer,
					&shm,
					&handle,
					/* The whole frame is wanted every time, not just the part
					 * of it that changed since the previous one. */
					|buffer| frame.copy(buffer, 1),
					area,
				);
				frame.destroy();
				result
			}
		}
	}

	/**
	 * Wait for the frame to be copied and read the pixels of the given area.
	 *
	 * @param  queue
	 * @param  state
	 * @param  buffer
	 * @param  shm
	 * @param  handle
	 * @param  copy
	 * @param  area
	 * @return Vector of Rgba (Result)
	 */
	#[allow(clippy::too_many_arguments)]
	fn copy_frame<Copy: FnOnce(&WlBuffer)>(
		queue: &mut EventQueue<State>,
		state: &mut State,
		buffer: &mut Option<ShmBuffer>,
		shm: &WlShm,
		handle: &QueueHandle<State>,
		copy: Copy,
		area: Geometry,
	) -> Result<Vec<Rgba<u8>>, String> {
		queue
			.roundtrip(state)
			.map_err(|e| format!("Wayland communication failed: {e}"))?;
		let info = state
			.frame_info
			.ok_or_else(|| String::from("The compositor did not offer a buffer"))?;
		if info.get_channels().is_none() {
			return Err(format!("Unsupported buffer format: {:?}", info.format));
		}
		let size = info
			.get_size()
			.ok_or_else(|| String::from("Invalid buffer size"))?;
		if info.width == 0
			|| info.height == 0
			|| info.stride < info.width.saturating_mul(PIXEL_SIZE)
		{
			return Err(String::from("Invalid buffer geometry"));
		}
		let shm_buffer = match buffer.take() {
			Some(shm_buffer) if shm_buffer.info == info => shm_buffer,
			previous => {
				if let Some(previous) = previous {
					previous.destroy();
				}
				Self::create_buffer(shm, handle, info, size)?
			}
		};
		copy(&shm_buffer.buffer);
		/* The buffer is kept around for the next frame, so it is put back
		 * before anything else can fail. */
		*buffer = Some(shm_buffer);
		while state.frame_status == FrameStatus::Pending {
			queue
				.blocking_dispatch(state)
				.map_err(|e| format!("Wayland communication failed: {e}"))?;
		}
		if state.frame_status == FrameStatus::Failed {
			return Err(String::from("The compositor rejected the copy request"));
		}
		let shm_buffer = buffer
			.as_mut()
			.ok_or_else(|| String::from("Missing buffer"))?;
		let mut data = vec![0; shm_buffer.size];
		shm_buffer
			.file
			.seek(SeekFrom::Start(0))
			.and_then(|_| shm_buffer.file.read_exact(&mut data))
			.map_err(|e| format!("Failed to read the frame: {e}"))?;
		Ok(Self::get_pixels(&data, info, area, state.y_invert))
	}

	/**
	 * Create a shared memory buffer for the given frame properties.
	 *
	 * @param  shm
	 * @param  handle
	 * @param  info
	 * @param  size
	 * @return ShmBuffer (Result)
	 */
	fn create_buffer(
		shm: &WlShm,
		handle: &QueueHandle<State>,
		info: FrameInfo,
		size: usize,
	) -> Result<ShmBuffer, String> {
		let file = Self::create_file(size)
			.map_err(|e| format!("Failed to create the frame buffer: {e}"))?;
		let pool_size = i32::try_from(size)
			.map_err(|_| String::from("The frame is too large to capture"))?;
		let pool = shm.create_pool(file.as_fd(), pool_size, handle, ());
		let buffer = pool.create_buffer(
			0,
			info.width.try_into().unwrap_or_default(),
			info.height.try_into().unwrap_or_default(),
			info.stride.try_into().unwrap_or_default(),
			info.format,
			handle,
			(),
		);
		Ok(ShmBuffer {
			file,
			pool,
			buffer,
			info,
			size,
		})
	}

	/**
	 * Create an unlinked file to share with the compositor.
	 *
	 * @param  size
	 * @return File (Result)
	 */
	fn create_file(size: usize) -> Result<File, std::io::Error> {
		let path = env::var_os("XDG_RUNTIME_DIR")
			.map(PathBuf::from)
			.unwrap_or_else(env::temp_dir)
			.join(format!(
				"{}-{}-{}",
				env!("CARGO_PKG_NAME"),
				process::id(),
				SystemTime::now()
					.duration_since(UNIX_EPOCH)
					.map(|duration| duration.as_nanos())
					.unwrap_or_default()
			));
		let file = OpenOptions::new()
			.read(true)
			.write(true)
			.create_new(true)
			/* The frame contents are private, and the fallback directory
			 * might be shared with the other users of the system. */
			.mode(0o600)
			.open(&path)?;
		/* The file is only shared via its descriptor, so it does not need
		 * to stay visible in the file system. */
		fs::remove_file(&path)?;
		file.set_len(size as u64)?;
		Ok(file)
	}

	/**
	 * Convert the given area of a frame buffer into pixels.
	 *
	 * @param  data
	 * @param  info
	 * @param  area
	 * @param  y_invert
	 * @return Vector of Rgba
	 */
	fn get_pixels(
		data: &[u8],
		info: FrameInfo,
		area: Geometry,
		y_invert: bool,
	) -> Vec<Rgba<u8>> {
		let (r, g, b) = info.get_channels().unwrap_or((2, 1, 0));
		let has_alpha = info.has_alpha();
		let (x_offset, y_offset) = (
			u32::try_from(area.x).unwrap_or_default(),
			u32::try_from(area.y).unwrap_or_default(),
		);
		let mut pixels = Vec::with_capacity(
			usize::try_from(area.width.saturating_mul(area.height))
				.unwrap_or_default(),
		);
		for y in 0..area.height {
			let row = y_offset.saturating_add(y).min(info.height - 1);
			let row = if y_invert { info.height - 1 - row } else { row };
			for x in 0..area.width {
				let column = x_offset.saturating_add(x).min(info.width - 1);
				let offset =
					usize::try_from(row * info.stride + column * PIXEL_SIZE)
						.unwrap_or_default();
				pixels.push(match data.get(offset..offset + 4) {
					Some(pixel) => Rgba::from([
						pixel[r],
						pixel[g],
						pixel[b],
						if has_alpha { pixel[3] } else { 255 },
					]),
					None => Rgba::from([0, 0, 0, 255]),
				});
			}
		}
		pixels
	}
}

/* Bind the globals that are needed for capturing. */
impl Dispatch<WlRegistry, ()> for State {
	fn event(
		state: &mut Self,
		registry: &WlRegistry,
		event: wl_registry::Event,
		_: &(),
		_: &Connection,
		handle: &QueueHandle<Self>,
	) {
		if let wl_registry::Event::Global {
			name,
			interface,
			version,
		} = event
		{
			if interface == WlShm::interface().name {
				state.shm = Some(registry.bind(name, 1, handle, ()));
			} else if interface == ZwlrScreencopyManagerV1::interface().name {
				state.screencopy = Some(registry.bind(
					name,
					version.min(SCREENCOPY_VERSION),
					handle,
					(),
				));
			} else if interface == HyprlandToplevelExportManagerV1::interface().name
				&& version >= TOPLEVEL_EXPORT_VERSION
			{
				state.toplevel_export =
					Some(registry.bind(name, TOPLEVEL_EXPORT_VERSION, handle, ()));
			} else if interface == ZwlrForeignToplevelManagerV1::interface().name {
				registry.bind::<ZwlrForeignToplevelManagerV1, _, _>(
					name,
					version.min(TOPLEVEL_VERSION),
					handle,
					(),
				);
			} else if interface == WlOutput::interface().name {
				let index = state.outputs.len();
				let output =
					registry.bind(name, version.min(OUTPUT_VERSION), handle, index);
				state.outputs.push((
					output,
					OutputInfo {
						name: format!("output-{index}"),
						geometry: Geometry::default(),
					},
				));
			}
		}
	}
}

/* Collect the name and the resolution of the outputs. */
impl Dispatch<WlOutput, usize> for State {
	fn event(
		state: &mut Self,
		_: &WlOutput,
		event: wl_output::Event,
		index: &usize,
		_: &Connection,
		_: &QueueHandle<Self>,
	) {
		let info = match state.outputs.get_mut(*index) {
			Some((_, info)) => info,
			None => return,
		};
		match event {
			wl_output::Event::Mode {
				flags,
				width,
				height,
				..
			} => {
				if flags
					.into_result()
					.map(|flags| flags.contains(wl_output::Mode::Current))
					.unwrap_or_default()
				{
					info.geometry = Geometry::new(
						0,
						0,
						width.try_into().unwrap_or_default(),
						height.try_into().unwrap_or_default(),
					);
				}
			}
			wl_output::Event::Name { name } => info.name = name,
			_ => {}
		}
	}
}

/* Track the state of the frame that is being copied. */
impl Dispatch<ZwlrScreencopyFrameV1, ()> for State {
	fn event(
		state: &mut Self,
		_: &ZwlrScreencopyFrameV1,
		event: zwlr_screencopy_frame_v1::Event,
		_: &(),
		_: &Connection,
		_: &QueueHandle<Self>,
	) {
		match event {
			zwlr_screencopy_frame_v1::Event::Buffer {
				format,
				width,
				height,
				stride,
			} => {
				state.frame_info = match format {
					WEnum::Value(format) => Some(FrameInfo {
						format,
						width,
						height,
						stride,
					}),
					WEnum::Unknown(value) => {
						warn!("Unknown buffer format: {}", value);
						None
					}
				};
			}
			zwlr_screencopy_frame_v1::Event::Flags { flags } => {
				state.y_invert = flags
					.into_result()
					.map(|flags| {
						flags.contains(zwlr_screencopy_frame_v1::Flags::YInvert)
					})
					.unwrap_or_default();
			}
			zwlr_screencopy_frame_v1::Event::Ready { .. } => {
				state.frame_status = FrameStatus::Ready;
			}
			zwlr_screencopy_frame_v1::Event::Failed => {
				state.frame_status = FrameStatus::Failed;
			}
			_ => {}
		}
	}
}

/* Collect the toplevels that the compositor exposes. */
impl Dispatch<ZwlrForeignToplevelManagerV1, ()> for State {
	fn event(
		state: &mut Self,
		_: &ZwlrForeignToplevelManagerV1,
		event: zwlr_foreign_toplevel_manager_v1::Event,
		_: &(),
		_: &Connection,
		_: &QueueHandle<Self>,
	) {
		if let zwlr_foreign_toplevel_manager_v1::Event::Toplevel { toplevel } = event
		{
			state.toplevels.push((toplevel, ToplevelInfo::default()));
		}
	}

	event_created_child!(State, ZwlrForeignToplevelManagerV1, [
		EVT_TOPLEVEL_OPCODE => (ZwlrForeignToplevelHandleV1, ()),
	]);
}

/* Collect the title and the state of the toplevels. */
impl Dispatch<ZwlrForeignToplevelHandleV1, ()> for State {
	fn event(
		state: &mut Self,
		toplevel: &ZwlrForeignToplevelHandleV1,
		event: zwlr_foreign_toplevel_handle_v1::Event,
		_: &(),
		_: &Connection,
		_: &QueueHandle<Self>,
	) {
		let info = match state
			.toplevels
			.iter_mut()
			.find(|(handle, _)| handle == toplevel)
		{
			Some((_, info)) => info,
			None => return,
		};
		match event {
			/* The entry is kept so that the indices of the toplevels that
			 * are captured stay valid for the rest of the session. */
			zwlr_foreign_toplevel_handle_v1::Event::Closed => {
				info.closed = true;
				info.activated = false;
				toplevel.destroy();
			}
			zwlr_foreign_toplevel_handle_v1::Event::Title { title } => {
				info.title = title
			}
			zwlr_foreign_toplevel_handle_v1::Event::AppId { app_id } => {
				info.app_id = app_id
			}
			zwlr_foreign_toplevel_handle_v1::Event::State { state } => {
				info.activated = state
					.chunks_exact(4)
					.filter_map(|value| value.try_into().ok())
					.map(u32::from_ne_bytes)
					.any(|value| value == ToplevelState::Activated as u32);
			}
			_ => {}
		}
	}
}

/* Track the state of the toplevel frame that is being copied. */
impl Dispatch<HyprlandToplevelExportFrameV1, ()> for State {
	fn event(
		state: &mut Self,
		_: &HyprlandToplevelExportFrameV1,
		event: hyprland_toplevel_export_frame_v1::Event,
		_: &(),
		_: &Connection,
		_: &QueueHandle<Self>,
	) {
		match event {
			hyprland_toplevel_export_frame_v1::Event::Buffer {
				format,
				width,
				height,
				stride,
			} => {
				state.frame_info = match format {
					WEnum::Value(format) => Some(FrameInfo {
						format,
						width,
						height,
						stride,
					}),
					WEnum::Unknown(value) => {
						warn!("Unknown buffer format: {}", value);
						None
					}
				};
			}
			hyprland_toplevel_export_frame_v1::Event::Flags { flags } => {
				state.y_invert = flags
					.into_result()
					.map(|flags| {
						flags.contains(
							hyprland_toplevel_export_frame_v1::Flags::YInvert,
						)
					})
					.unwrap_or_default();
			}
			hyprland_toplevel_export_frame_v1::Event::Ready { .. } => {
				state.frame_status = FrameStatus::Ready;
			}
			hyprland_toplevel_export_frame_v1::Event::Failed => {
				state.frame_status = FrameStatus::Failed;
			}
			_ => {}
		}
	}
}

delegate_noop!(State: ignore WlShm);
delegate_noop!(State: ignore HyprlandToplevelExportManagerV1);
delegate_noop!(State: ignore WlShmPool);
delegate_noop!(State: ignore WlBuffer);
delegate_noop!(State: ignore ZwlrScreencopyManagerV1);
