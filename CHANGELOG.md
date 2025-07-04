# Changelog

## [1.1.0] - 2025-02-19

### Changed
 - Stretch coded features instead of decoding first for a huge performance improvement.

## [1.0.12] - 2025-01-02

### Added
 - Distribute an official resampler manifest file for OpenUtau users.

### Changed
 - Change D4C default threshold to 0.25 to match old straycat.

## [1.0.11] - 2024-08-01

### Fixed
 - Fixed vocal fry issue on some notes.

## [1.0.10] - 2024-07-25

### Fixed
 - Fixed OpenUtau + issue.

## [1.0.9] - 2024-07-23

### Fixed
 - Fixed error with interpreting pitchbends that causes pitch errors around overlap areas.

## [1.0.8] - 2024-07-23

### Changed
 - Add checksum hashes for each release by @layetri in https://github.com/UtaUtaUtau/straycat-rs/pull/4 (thank you again!)

## [1.0.6] - 2024-07-23

### Changed
 - Add GitHub Actions for CI by @layetri in https://github.com/UtaUtaUtau/straycat-rs/pull/1 (thank you!)

## [1.0.1] - 2024-07-23

### Fixed
 - Fixed an index out of bounds error.

## [1.0.0] - 2024-07-23

### Added
 - Flag parsing for extra user-controlled behavior.

### Changed
 - Some for loops are switched to iterators to improve performance.
 - Interpolators now use references instead of moving values.
 - Improve numerical behavior in some calculations.

## [1.0.0-alpha] - 2024-07-12

 - Initial release 🎉

[1.1.0]: https://github.com/UtaUtaUtau/straycat-rs/compare/v1.0.12...v1.1.0
[1.0.12]: https://github.com/UtaUtaUtau/straycat-rs/compare/v1.0.11...v1.0.12
[1.0.11]: https://github.com/UtaUtaUtau/straycat-rs/compare/v1.0.10...v1.0.11
[1.0.10]: https://github.com/UtaUtaUtau/straycat-rs/compare/v1.0.9...v1.0.10
[1.0.9]: https://github.com/UtaUtaUtau/straycat-rs/compare/v1.0.8...v1.0.9
[1.0.8]: https://github.com/UtaUtaUtau/straycat-rs/compare/v1.0.6...v1.0.8
[1.0.6]: https://github.com/UtaUtaUtau/straycat-rs/compare/v1.0.1...v1.0.6
[1.0.1]: https://github.com/UtaUtaUtau/straycat-rs/compare/v1.0.0...v1.0.1
[1.0.0]: https://github.com/UtaUtaUtau/straycat-rs/compare/v1.0.0-alpha...v1.0.0
[1.0.0-alpha]: https://github.com/UtaUtaUtau/straycat-rs/releases/tag/v1.0.0-alpha