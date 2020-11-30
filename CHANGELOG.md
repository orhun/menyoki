# Changelog <a href="https://github.com/orhun/menyoki"><img src="https://user-images.githubusercontent.com/24392180/99184076-96c10b00-2751-11eb-99ea-ad962144df76.png" width="25"></a>

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Added
- Enable gifski encoder with --gifski flag
- Support APNG format

### Changed
- Update contribution guidelines about changes in command line arguments
- Update the optimization level of the development profile
- Build with debug profile while generating shell completions

### Removed
- Remove update_release action from CD workflow

## [0.1.3] - 2020-11-26
### Added
* Add release instructions
* Extend tests cases and add new tests
* Add manpage for configuration file
* Add APNG to supported formats as TODO
* Support using `$MENYOKI_CONFIG` for configuration file
* Update the release details after upload in CD workflow

### Changed
* Update badges in README.md
* Update AUR installation section in README.md
* Change default configuration file extension to `.conf`

### Removed
* Remove release notes from CD workflow due to [softprops/action-gh-release#57](https://github.com/softprops/action-gh-release/issues/57)

## [0.1.2] - 2020-11-23
### Fixed
* Fix the file name for release notes in CD workflow

## [0.1.1] - 2020-11-23
### Added
* Set the prepared release note for the releases with CD workflow

### Changed
* Update the homepage of project in Cargo.toml
* Update contribution guidelines about generating shell completions

### Fixed
* Allow record module tests pass without a window system running

## [0.1.0] - 2020-11-21
Initial release.
