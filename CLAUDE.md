# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this crate is

`magick_rust` is a thin "somewhat safe" Rust wrapper over ImageMagick's C `MagickWand` library. The crate generates its FFI bindings at build time via `bindgen` against the locally installed ImageMagick headers — there is no checked-in `bindings.rs`. ImageMagick **7.1.1 or later** is required; the 6.x series is not compatible. Minimum Rust is 1.85 (edition 2024).

## Build, test, examples

```shell
cargo build
cargo test                                # runs the integration tests in tests/
cargo test --test lib test_resize_image   # run a single integration test by name
cargo run --example thumbnail-cat         # run an example from examples/
cargo doc --no-deps --open                # generate API docs (docs.rs cannot build this crate; see README)
```

Tests live in `tests/lib.rs` (integration tests, not `#[cfg(test)]` modules) and share fixtures via `tests/fixtures.rs` + `tests/fixtures/`. Every test must call `magick_wand_genesis()` once before using a `MagickWand`; the existing tests do this via a shared `static START: Once`.

### Locating ImageMagick

`build.rs` first tries `pkg-config`. To override, export any of:

- `IMAGE_MAGICK_DIR` — install prefix
- `IMAGE_MAGICK_LIB_DIRS` — `:`-separated lib dirs (`;` on Windows)
- `IMAGE_MAGICK_INCLUDE_DIRS` — `:`-separated include dirs
- `IMAGE_MAGICK_LIBS` — libs to link

The `disable-hdri` feature (on by default) works around a `bindgen` issue where `QuantumRange` is not found when ImageMagick is built with `--disable-hdri`. Toggle it if linking against an HDRI build.

### Docker

`docker/` builds an Ubuntu+ImageMagick-from-source image for reproducible testing:

```shell
cd docker
docker compose build --pull
docker compose run magick-rust
cargo test
```

## Architecture

### Three-layer structure

1. **`bindings` (generated)** — `build.rs` runs `bindgen` against `<MagickWand/MagickWand.h>` into `$OUT_DIR/bindings.rs`, then `src/lib.rs` does `include!(...)` to expose it as `crate::bindings`. The build script:
   - Enforces an ImageMagick version range (`MIN_VERSION`/`MAX_VERSION` constants near the top of `build.rs`).
   - Ignores a hand-maintained set of float-related macros that conflict between `<math.h>` and ImageMagick headers (`IgnoreMacros`).
   - On Windows, prepends a `ssize_t` typedef before the include so MSVC builds compile.
   - On macOS calls `xcrun --show-sdk-path` to add the SDK `usr/include` to bindgen's clang args.
   - When updating bindings, bump `MIN_VERSION`/`MAX_VERSION` if needed and check the `IgnoreMacros` list against the new headers.

2. **Type re-exports — `src/types/`** — Newtype wrappers and a small number of Rust-side types (`Image`, `KernelInfo`, `GeometryInfo`, …) that don't belong inside any specific wand.

3. **Safe wrappers — `src/wand/`** — One module per wand: `magick.rs` (`MagickWand`), `drawing.rs` (`DrawingWand`), `pixel.rs` (`PixelWand`/`HSL`). All three are constructed by the **`wand_common!` macro** in `src/wand/macros.rs`, which generates the struct, `new`/`from_ptr`/`as_ptr`, `Drop`, `Clone`, and exception accessors from the C `New…`/`Clear…`/`Is…`/`Clone…`/`Destroy…` symbol names. If you are wrapping a new wand type, invoke `wand_common!` rather than writing those impls by hand.

### Adding new MagickWand functions

`src/wand/macros.rs` provides several declarative macros that generate idiomatic Rust wrappers from tables of C function names. Look near the bottom of `src/wand/magick.rs` for the in-use lists:

- `get!` — single-return getter.
- `set_get!` — paired getter/setter for typed values (`ColorspaceType`, `usize`, `f64`, etc.).
- `string_set_get!` — paired getter/setter for C strings.
- `mutations!` — operations that mutate the wand and return `Result<()>` (covers most of the "do something to the image" verbs).

For getters/setters and simple mutations, **add a row to the appropriate table** in `src/wand/magick.rs` — this is usually a one-line change with no extra unsafe code. Only write a full hand-rolled wrapper (like `new_image`, `get_image`, `contrast_stretch_image`) when the C signature doesn't fit any macro shape (e.g. returns a new wand, takes complex out-params, or needs custom marshalling).

### Result and error conventions

- `src/result.rs` defines `MagickError(String)` and `Result<T> = std::result::Result<T, MagickError>`.
- The macros and helpers in `magick.rs` convert C return conventions into `Result`: `result_from_boolean` for `MagickBooleanType`, `result_from_ptr` / `result_from_ptr_with_error_message` for fallible pointer returns.
- ImageMagick's `MagickBooleanType::{MagickTrue, MagickFalse}` are re-exported through `src/wand/mod.rs` so call sites can use the bare names.

### Global initialization

`magick_wand_genesis()` / `magick_wand_terminus()` map directly to `MagickWandGenesis`/`MagickWandTerminus` and guard against double-init via `IsMagickWandInstantiated`. Callers (including each test) must invoke `magick_wand_genesis()` before any wand operation — typically once, behind a `std::sync::Once`.

## Contribution conventions (from README)

- Tests are optional for thin wrappers — the crate trusts ImageMagick is tested upstream.
- For getters/setters, **add a row to the macro table in `wand/magick.rs`** rather than writing a hand-rolled function.
