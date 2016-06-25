# Change Log

All notable changes to this project will be documented in this file.
This project adheres to [Semantic Versioning](http://semver.org/).

## [0.5.1] - 2016-06-25
### Changed
- hjr3: Changed read_image_blob() to borrow data rather than take ownership.

## [0.5.0] - 2016-05-18
### Changed
- marjakm: Added numerous functions and enabled cross-compile support.

## [0.4.0] - 2016-03-29
### Changed
- Add functions for detecting and correcting image orientation.

## [0.3.3] - 2016-03-17
### Changed
- Allow libc version 0.2 or higher

## [0.3.2] - 2016-02-10
### Changed
- Automatically generate `bindings.rs` using `rust-bindgen` via `build.rs` script.

## [0.3.1] - 2016-01-02
### Changed
- Fix bug `get_image_property()` to ensure C string is copied.

## [0.3.0] - 2016-01-02
### Changed
- Add `get_image_property()` function to retrieve, for example, EXIF data.

## [0.2.3] - 2015-12-26
### Changed
- Upgrade to libc 0.2.4 in hopes of fixing downstream build incompatibilities.

## [0.2.2] - 2015-12-23
### Changed
- Change the build to specify the likely path to ImageMagick, for easier setup.

## [0.2.1] - 2015-09-07
### Changed
- Fix the cargo package name (replace dash with underscore).

## [0.2.0] - 2015-06-10
### Changed
- Add a `fit()` function for fitting an image to a given bounds.

## [0.1.0] - 2015-06-09
### Changed
- Initial release
