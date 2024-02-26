# Change Log

All notable changes to this project will be documented in this file.
This project adheres to [Semantic Versioning](http://semver.org/).
This file follows the convention described at
[Keep a Changelog](http://keepachangelog.com/en/1.0.0/).

## [0.20.0] - 2024-02-25
### Changed
- **BREAKING CHANGES**
    * 5ohue: added `CompositeOperator` in place of `bindings::CompositeOperator`. Instead of using `bindings::CompositeOperator_LightenIntensityCompositeOp` one must now use `CompositeOperator::LightenIntensity`.
### Added
- 5ohue: added `sigmoidal_contrast_image()` function to the wand API.

## [0.19.1] - 2023-09-23
### Changed
- Feature `disable-hdri` is now enabled by default to work around an apparent
  bug with rust-bindgen that cannot discover the `QuantumRange` constant which
  is conditionally computed during compile-time in the MagickCore library.

## [0.19.0] - 2023-06-19
### Added
- walterbm: Add `coalesce()` for image coalesce.
- FaithBeam: Add `compose_images_gravity()` for image composition.
- Upgrade dependency `bindgen` to 0.66.1

## [0.18.0] - 2023-04-15
### Changed
- **BREAKING CHANGEs**
    * jshrake: Add `map` argument to `import_image_pixels()` and change `pixels` argument to a byte slice rather than a vector reference.
### Added
- jshrake: Add `MagickAutoGammaImage` and `MagickAutoLevelImage`
- 2e0by0: add API documentation and setup automated build of docs.
### Fixed
- BeatButton: prevent segfault if `MagickGetImageBlob` returns `null`

## [0.17.0] - 2022-12-10
### Added
- walterbm: Add support for image deskew.
- Upgrade dependency `bindgen` to 0.63
- DCjanus: Upgrade dependency `bindgen` (0.59 -> 0.60)
- MaksRawski: add `DrawRectangle` and `MagickBrightnessContrastImage` bindings

## [0.16.0] - 2022-04-09
### Added
- walterbm: Add support for thumbnail image resizing.
- DCjanus: add methods `get_image_alpha_channel`, `draw_image`, `set_image_channel_mask`,
  `evaluate_image`, `border_image`, `shadow_image`, `import_image_pixels`, `set_first_iterator`,
  `next_image`
- DCjanus: feat: std error compatible error
- davidwilemski: add binding for `MagickStripImage()`

## [0.15.0] - 2021-08-10
### Added
- glebpom: Support resource limits
- lerouxrgd: Add kmeans
- danielronnkvist: modulate image
- asonix: Add quantum depth and sample method
- Drevoed: Add MagickLiquidRescaleImage and MagickImplodeImage
### Changed
- brownjohnf: Update imagemmagick checks to support 7.1
- liyunde: Fix path_separator on windows can not build
- kz6wk9: Required version bump on bindgen.
- asonix: Set environment variable with magickcore config flags
- captainbland: Add workaround for `QuantumRange` not defined error when hdri is disabled

## [0.14.0] - 2020-05-21
### Added
- npajkovsky: add `negate_image()` operation
- danielronnkvist: add `MagickFxImage` binding
- danielronnkvist: add `MagickLevelImage` binding
- danielronnkvist: add `MagickSetImageAlpha` binding
- danielronnkvist: add `MagickBlurImage` binding
- danielronnkvist: add `MagickHaldClutImage` binding
### Changed
- npajkovsky: remove use of deprecated item `try!`, use `?` instead
- npajkovsky: fix multiple redefined values on linux build
- Updated `bindgen` to `0.53.2` release, added "size_t is usize" flag.

## [0.13.0] - 2020-03-25
### Added
- danielronnkvist: Binding for MagickClutImage
- danielronnkvist: Binding for MagickSetSize
- max-frai: Add gaussian blur function

## [0.12.0] - 2019-09-09
### Added
- magiclen: Binding for set_background_color

## [0.11.0] - 2019-04-17
### Changed
- Updated `bindgen` dependency to 0.31 release and fixed compiler issues.
  Enum definitions changed again, default in bindgen is different now, and
  using `default_enum_style()` caused endless compiler errors.
- Made `get_exception_type()`, `get_exception()`, and `clear_exception()`
  on the various wand implementations.

## [0.10.0] - 2018-08-11
### Added
- Mewp: Add ping_image and ping_image_blob functions.
- Mewp: Add reset_image_page function.
- Mewp: Add set_image_alpha_channel function.
- NQNStudios: Adding binding for MagickAddImage function.
- NQNStudios: Adding doc comment and rotate_image function.
- NQNStudios: Adding binding for adaptive_resize_image function.

## [0.9.0] - 2018-05-05
### Added
- Mewp: Numerous additional MagickWand functions
### Changed
- Mewp: crop_image() now returns a Result
### Fixed
- Mewp: Fixed memory management in `string_get!`
- sindreij: Fix exporting pdf->jpeg for multi-page pdf

## [0.8.0] - 2018-02-16
### Added
- little-bobby-tables: add color-related getters and mutations
- sindreij: Add crop_image() to MagickWand

## [0.7.1] - 2017-12-30
### Changed
- gentoo90: Hide more types from bindgen to fix the build for some systems
- gentoo90: Build now supports Windows

## [0.7.0] - 2017-08-26
### Changed
- Upgrade bindgen to 0.29
- little-bobby-tables: Change to MagickWand 7.0;
  this introduces backward incompatible changes...
- `get_quantum` and `set_quantum` now take `Quantum` instead of `u16`
- `resize_image` no longer takes a `blur_factor` argument
- `InterpolatePixelMethod` was renamed `PixelInterpolateMethod`

## [0.6.6] - 2017-07-08
### Changed
- Downgrade to version 0.25.5 of `bindgen` library to avoid errors on Linux.

## [0.6.5] - 2017-07-07
### Added
- Add `compare_images()` method to `MagickWand` type.
### Changed
- Update to latest release of `bindgen` library.

## [0.6.4] - 2017-04-08
### Changed
- Actually set the version this time.

## [0.6.3] - 2017-04-08
### Changed
- Changed to use `pkg-config` crate to get MagickWand compiler settings.
- Fixed bindings generation on FreeBSD (i.e. no longer hard-coded).
- Changed the bindings generation to use `libc` prefix for C types.
- Changed the bindings generation and interface code to use Rust enums.

## [0.6.2] - 2016-10-20
### Changed
- Presence of `pkg-config` checked in `build.rs` script at build time.

## [0.6.1] - 2016-10-16
### Changed
- MagickWand version enforced in `build.rs` script at build time.

## [0.6.0] - 2016-09-20
### Changed
- Update to 0.19.0 version of rust-bindgen; rebuilds are much faster.
- Hacked bindings for FreeBSD systems due to rust-bindgen bug #385.
- gadomski: add `set_option()` method to wand API.
- gadomski: add `write_images_blob()` to create animated GIFs.

## [0.5.2] - 2016-07-17
### Changed
- Streamline error handling in `build.rs` script.
- Fix the crate version number (previously stuck at 0.4.0).

## [0.5.1] - 2016-06-25
### Changed
- hjr3: Changed `read_image_blob()` to borrow data rather than take ownership.

## [0.5.0] - 2016-05-18
### Added
- marjakm: Added numerous functions and enabled cross-compile support.

## [0.4.0] - 2016-03-29
### Added
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
### Added
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
### Added
- Add a `fit()` function for fitting an image to a given bounds.

## [0.1.0] - 2015-06-09
### Changed
- Initial release
