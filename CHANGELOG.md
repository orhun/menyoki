# Changelog <a href="https://github.com/orhun/menyoki"><img src="https://user-images.githubusercontent.com/24392180/99184076-96c10b00-2751-11eb-99ea-ad962144df76.png" width="25"></a>

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.5.4] - 2021-10-22
### Changed
- Switch to Rust 2021 edition
- Bump the Rust version in Dockerfile
- Improve the Docker build workflow
- Switch to ubuntu-20.04 runner
- Run CI builds periodically
- Add security audit workflow
- Update .gitignore about misc files
- Enable macOS releases
- Bump dependencies

## [1.5.3] - 2021-08-22
### Changed
- Temporarily disable macOS releases

## [1.5.2] - 2021-08-22
### Changed
- Drop `apng` dependency and use `png` for encoding APNG
- Bump dependencies in Cargo.toml
- Bump the Rust version in Dockerfile
- Update workflows for automated Docker builds

## [1.5.1] - 2021-07-04
### Changed
- Update the CD workflow about building for macOS
- Update image viewer tests

## [1.5.0] - 2021-07-04
### Added
- Add **view** subcommand for viewing image files from the terminal

## [1.4.2] - 2021-07-02
### Changed
- Switch from [softprops/action-gh-release](https://github.com/softprops/action-gh-release) to [svenstaro/upload-release-action](https://github.com/svenstaro/upload-release-action)

## [1.4.1] - 2021-07-02
### Changed
- Implement custom error handling ([#21](https://github.com/orhun/menyoki/issues/21))
- Update the PGP keyserver link in documentation

### Fixed
- Fix clippy lints
- Remove cargo-bloat from workflows

## [1.4.0] - 2021-04-25
### Added
- Add `--cancel-keys` option for changing the key bindings that are responsible for cancelling an operation. ([#22](https://github.com/orhun/menyoki/issues/22))

### Changed
- [**breaking**] Improve the action key parser to support multiple key bindings. Representation of the default action key is changed to `LAlt-S,LAlt-Enter` (<s>`LAlt-S/Enter`</s>). New syntax for the key bindings is: `<key1>,<key2>-<key3>-...-<key4>,<key5>`
- [**breaking**] Rename `--keys` option to `--action-keys` including corresponding configuration file entries and environment variables.
- Change default timeout value for area selection to 300 seconds
- Upgrade dependencies in Cargo.toml

## [1.3.0] - 2021-03-20
### Added
- Add examples for copying captures and videos ([#19](https://github.com/orhun/menyoki/pull/19))
- Add `--mouse` flag for selecting windows with a mouse click

### Fixed
- Compare ICO height with geometry height in set_icon_size
- Set the `--quiet` flag implicitly if output is piped to stdout
- Sleep before drawing the borders while selecting a window

### Changed
- Upgrade dependencies in Cargo.toml

## [1.2.1] - 2021-02-03
### Added
- Add [lychee-action](https://github.com/lycheeverse/lychee-action) to CI workflow

### Changed
- Disable musl builds ([#17](https://github.com/orhun/menyoki/issues/17))
- Update README.md about installation instructions
- Update RELEASE.md about releasing for Arch Linux
- Upgrade dependencies

## [1.2.0] - 2020-12-20
### Added
- Add `--parent` flag for recording/capturing the parent window ([#14](https://github.com/orhun/menyoki/issues/14))
- Add `--monitor` argument for selecting the monitor to capture/record ([#15](https://github.com/orhun/menyoki/issues/15))

### Changed
- Use architecture compatible types for Xlib calls
- Update README.md about Linux dependencies ([#7](https://github.com/orhun/menyoki/issues/7))

## [1.1.0] - 2020-12-10
### Added
- Use stdout as output if "-" is given as file ([#10](https://github.com/orhun/menyoki/issues/10))

### Changed
- Allow disabling action keys on slop selection ([#12](https://github.com/orhun/menyoki/issues/12))

### Fixed
- Use the parent window on invalid child geometry ([#6](https://github.com/orhun/menyoki/issues/6))

## [1.0.2] - 2020-12-08
### Changed
- Update README.md about installation requirements

### Fixed
- Update analyze module tests about invalid file metadata

## [1.0.1] - 2020-12-07
### Changed
- Update traced opcodes of X11 implementation
- Update roadmap in README.md about platforms

## [1.0.0] - 2020-12-06
### Add
- Add completions for powershell and elvish
- Update CD workflow about releasing on different platforms
- Update CI workflow about testing on different platforms

### Changed
- Disable default features of gifski dependency
- Add `ski` feature for enabling gifski as default
- Update pull request template about configuration file

### Removed
- Remove verbose flag from codecov action in CI workflow

### Fixed
- Enable gifski encoder if `--fast` flag is set
- Create new GC while drawing text on window
- Limit maximum countdown value to 99

## [0.1.5] - 2020-12-04
### Added
- Support [slop](https://github.com/naelstrof/slop) via `--size $(slop)` for area selection
- Add blank implementation for other platforms

### Changed
- Update CD workflow for building for other platforms

### Fixed
- Prevent abort on cancelling record/capture ([#3](https://github.com/orhun/menyoki/issues/3))
- Check if RGBA color is valid while colorizing the report

## [0.1.4] - 2020-12-02
### Added
- Enable gifski encoder with `--gifski` flag
- Support APNG format
- Add `--font` argument for font selection ([#1](https://github.com/orhun/menyoki/issues/1))
- Add codecov config

### Changed
- Update contribution guidelines about changes in command line arguments
- Update the optimization level of the development profile
- Build with debug profile while generating shell completions
- Use `--border` argument for setting the border width
- Update release instructions about continuous integration workflow

### Removed
- Remove update_release action from CD workflow

### Fixed
- Install musl-tools for musl build in CD workflow

## [0.1.3] - 2020-11-26
### Added
- Add release instructions
- Extend tests cases and add new tests
- Add manpage for configuration file
- Add APNG to supported formats as TODO
- Support using `$MENYOKI_CONFIG` for configuration file
- Update the release details after upload in CD workflow

### Changed
- Update badges in README.md
- Update AUR installation section in README.md
- Change default configuration file extension to `.conf`

### Removed
- Remove release notes from CD workflow due to [softprops/action-gh-release#57](https://github.com/softprops/action-gh-release/issues/57)

## [0.1.2] - 2020-11-23
### Fixed
- Fix the file name for release notes in CD workflow

## [0.1.1] - 2020-11-23
### Added
- Set the prepared release note for the releases with CD workflow

### Changed
- Update the homepage of project in Cargo.toml
- Update contribution guidelines about generating shell completions

### Fixed
- Allow record module tests pass without a window system running

## [0.1.0] - 2020-11-21
Initial release.
