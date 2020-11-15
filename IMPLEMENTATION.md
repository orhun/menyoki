# Implementation <a href="https://github.com/orhun/menyoki"><img src="https://user-images.githubusercontent.com/24392180/99184076-96c10b00-2751-11eb-99ea-ad962144df76.png" width="25"></a>

Details of the implementation of **menyoki**.

## Project Structure

[mod.rs](https://doc.rust-lang.org/rust-by-example/mod/split.html) is used for the hierarchy of modules. Every directory in `src/` is a module and its general methods/types are contained in a `mod.rs` file.

`settings.rs` file is commonly used for handling configuration based operations such as conditionally parsing command line arguments to set an option/flag for a particular module. It constructs a struct named `XyzSettings` where `Xyz` is generally the name of the module.

* [main.rs](https://github.com/orhun/menyoki/blob/master/src/main.rs) -> starts the _application_ (`App::new(...).start()`)
* [app.rs](https://github.com/orhun/menyoki/blob/master/src/app.rs) -> `App` (contains the application methods such as `record`, `capture` and `edit_image`)
* [settings.rs](https://github.com/orhun/menyoki/blob/master/src/settings.rs) -> `AppSettings`
* analyze
  * [mod.rs](https://github.com/orhun/menyoki/blob/master/src/analyze/mod.rs) -> `ImageAnalyzer`
  * [settings.rs](https://github.com/orhun/menyoki/blob/master/src/analyze/settings.rs) -> `AnalyzeSettings`
* args
  * [matches.rs](https://github.com/orhun/menyoki/blob/master/src/args/matches.rs) -> `ArgMatches` (`clap::ArgMatches` wrapper for using configuration file and environment variables)
  * [mod.rs](https://github.com/orhun/menyoki/blob/master/src/args/mod.rs) -> `Args` (command line arguments)
  * [parser.rs](https://github.com/orhun/menyoki/blob/master/src/args/parser.rs) -> `ArgParser` (helper for parsing arguments)
* edit
  * [mod.rs](https://github.com/orhun/menyoki/blob/master/src/edit/mod.rs) -> `ImageOps` (contains image operations related functions such as `crop`, `resize` and `rotate`)
  * [settings.rs](https://github.com/orhun/menyoki/blob/master/src/edit/settings.rs) -> `ImageSettings`, `ColorSettings`, `EditSettings`
* file
  * [format.rs](https://github.com/orhun/menyoki/blob/master/src/file/format.rs) -> `FileFormat` (enum for file formats)
  * [info.rs](https://github.com/orhun/menyoki/blob/master/src/file/info.rs) -> `FileInfo` (enum for adding information to the file name)
  * [mod.rs](https://github.com/orhun/menyoki/blob/master/src/file/mod.rs) -> `File` (path + format)
  * [settings.rs](https://github.com/orhun/menyoki/blob/master/src/file/settings.rs) -> `SaveSettings`
* gif
  * [decoder.rs](https://github.com/orhun/menyoki/blob/master/src/gif/decoder.rs) -> `Decoder` (GIF decoder)
  * [encoder.rs](https://github.com/orhun/menyoki/blob/master/src/gif/encoder.rs) -> `Encoder` (trait that GIF encoders implement)
  * [mod.rs](https://github.com/orhun/menyoki/blob/master/src/gif/mod.rs) -> `Gif` (default GIF encoder)
  * [settings.rs](https://github.com/orhun/menyoki/blob/master/src/gif/settings.rs) -> `GifSettings`
  * [ski.rs](https://github.com/orhun/menyoki/blob/master/src/gif/ski.rs) -> `Gif` (gifski encoder, enabled with `ski` feature)
* image
  * [geometry.rs](https://github.com/orhun/menyoki/blob/master/src/image/geometry.rs) -> `Geometry` (x + y + width + height)
  * [mod.rs](https://github.com/orhun/menyoki/blob/master/src/image/mod.rs) -> `Image` (main image type)
  * [padding.rs](https://github.com/orhun/menyoki/blob/master/src/image/padding.rs) -> `Padding` (top + right + bottom + left)
  * [settings.rs](https://github.com/orhun/menyoki/blob/master/src/image/settings.rs) -> `PngSettings`, `JpgSettings`, `PnmSettings`
* record
  * [fps.rs](https://github.com/orhun/menyoki/blob/master/src/record/fps.rs) -> `FpsClock` (FPS controller)
  * [mod.rs](https://github.com/orhun/menyoki/blob/master/src/record/mod.rs) -> `RecordResult`, `Recorder`
  * [settings.rs](https://github.com/orhun/menyoki/blob/master/src/record/settings.rs) -> `RecordSettings`
* util
  * [command.rs](https://github.com/orhun/menyoki/blob/master/src/util/command.rs) -> `Command` (for executing OS commands)
  * [keys.rs](https://github.com/orhun/menyoki/blob/master/src/util/keys.rs) -> `ActionKeys` (parser and checker)
  * [logger.rs](https://github.com/orhun/menyoki/blob/master/src/util/logger.rs) -> `Logger` (for initializing the logger)
  * [mod.rs](https://github.com/orhun/menyoki/blob/master/src/util/mod.rs) -> only contains the module declarations
  * [state.rs](https://github.com/orhun/menyoki/blob/master/src/util/state.rs) -> `InputState` (checks the pressed keys)
* window
  * [mod.rs](https://github.com/orhun/menyoki/blob/master/src/window/mod.rs) -> `Access`, `Capture` (crucial traits)
  * [test.rs](https://github.com/orhun/menyoki/blob/master/src/window/test.rs) -> `TestWindow` (implements `Capture` trait for testing purposes)
* x11
    * [display.rs](https://github.com/orhun/menyoki/blob/master/src/x11/display.rs) -> `Display` (X11 display wrapper with methods like `get_window` and `select_window`)
    * [mod.rs](https://github.com/orhun/menyoki/blob/master/src/x11/mod.rs) -> `WindowSystem` (implements `Access` trait for X11)
    * [window.rs](https://github.com/orhun/menyoki/blob/master/src/x11/window.rs) -> `Window` (X11 window wrapper with methods like `get_geometry` and `get_name`)

## Implementing For Other Platforms

There are two crucial traits in [src/window/mod.rs](https://github.com/orhun/menyoki/blob/master/src/window/mod.rs) that need to be implemented for **menyoki** to function.

`Access` trait must be implemented for accessing the [window system](https://en.wikipedia.org/wiki/Windowing_system) and getting a _Window_.

```rust
/* Window system functions for accessing a window */
pub trait Access<'a, Window: Capture + Send + Sync + Copy + Debug + 'static> {
	fn init(settings: &'a AppSettings<'a>) -> Option<Self>
	where
		Self: Sized;
	fn get_window(&mut self) -> Option<Window>;
}
```

As seen in the `Access`' definition, the _Window_ that `get_window` provides must also implement `Capture + Send + Sync + Copy + Debug` with the `'static` lifetime for thread safety.

`Capture` trait contains methods for getting an [Image](https://github.com/orhun/menyoki/blob/master/src/image/mod.rs), showing a countdown on the window or the console and, releasing the _captured_ window.

```rust
/* Window methods for capturing an image */
pub trait Capture {
	fn get_image(&self) -> Option<Image>;
	fn show_countdown(&self);
	fn release(&self);
}
```

As a reference, see [src/x11/mod.rs](https://github.com/orhun/menyoki/blob/master/src/x11/mod.rs) (for `Access` implementation) and [src/x11/window.rs](https://github.com/orhun/menyoki/blob/master/src/x11/window.rs) (for `Capture` implementation).

The rest of the modules/functions are not platform-dependent (abstracted) so they are expected to work properly.

# Contributing

If you're considering to contribute, please see the [Contribution Guidelines](https://github.com/orhun/menyoki/blob/master/CONTRIBUTING.md).
