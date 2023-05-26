# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased] - 2023-05-26

### Added
- Methods for reading tempature as a integer (`u16`).
  Users are advised to ONLY use these methods if running into to trouble using 
  the standard tempature reading methods returning `f32` values as it's a more accurate read.


## [0.2.0] - 2021-05-22

### Added
- Add support for device sleep and wake.

### Changed
- Removed delays after final EEPROM writes before exiting a method.
  Users are advised to wait enough time before interacting with the device again.
  Thanks to @David-OConnor for the suggestion.

## [0.1.0] - 2020-07-29

Initial release to crates.io.

[Unreleased]: https://github.com/eldruin/mlx9061x-rs/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/eldruin/mlx9061x-rs/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/eldruin/mlx9061x-rs/releases/tag/v0.1.0
