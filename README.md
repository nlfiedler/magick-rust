# magick-rust

A somewhat safe Rust interface to the [ImageMagick](http://www.imagemagick.org/) system, in particular, the MagickWand library. Many of the functions in the MagickWand API are still missing, but over time more will be added. Pull requests are welcome, as are bug reports, and requests for examples.

## Dependencies

Because this crate is generating bindings for a C/C++ library, there are several dependencies beyond simply having the latest Rust toolchain installed.

* [Rust](https://www.rust-lang.org) stable
* [ImageMagick](https://imagemagick.org) (version 7.1.1-26 or later)
    - Does _not_ work with ImageMagick **6.x** due to backward incompatible changes.
    - [FreeBSD](https://www.freebsd.org): `sudo pkg install ImageMagick7`
    - [Homebrew](http://brew.sh): `brew install imagemagick`
    - Linux may require building ImageMagick from source, see the [INSTALL.md](./INSTALL.md) guide
    - Windows: download `*-dll` [installer](https://www.imagemagick.org/script/download.php#windows). When installing, check the *Install development headers and libraries for C and C++* checkbox.
* [Clang](https://clang.llvm.org) (version 5.0 or higher, as dictated by [rust-bindgen](https://github.com/rust-lang/rust-bindgen))
* Windows requires MSVC toolchain
    - Download the [Microsoft C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) and select the `MSVC ... build tools` (latest version with appropriate architecture) and `Windows 11 SDK` (or `10` if using Windows 10).
* Optionally `pkg-config`, to facilitate linking with ImageMagick. Alternatively, you can set linker parameters via environment variables as described in the next section.

For detailed examples, see the [INSTALL.md](./INSTALL.md) guide, along with some discussion about the various dependencies.

## Build and Test

On FreeBSD, Linux, and macOS the following commands should suffice.

```shell
cargo build
cargo test
```

If `pkg-config` is not available, or you wish to override its behavior, you can set one or more environment variables before building. The `build.rs` script will pick these up and use them instead of trying to invoke the `pkg-config` utility.

* `IMAGE_MAGICK_DIR` - installation path of ImageMagick
* `IMAGE_MAGICK_LIB_DIRS` - list of `lib` directories split by `:`
* `IMAGE_MAGICK_INCLUDE_DIRS` - list of `include` directories split by `:`
* `IMAGE_MAGICK_LIBS` - list of the libraries with which to link

### Build on Windows

When building on Windows, you will need to set the `IMAGE_MAGICK_DIR` environment variable to point to the ImageMagick installation path. Maybe this is possible with the `set` command, but it may be necessary to set the variable in the system preferences. Without setting `IMAGE_MAGICK_DIR`, the `build.rs` script will try to run `pkg-config` which is a tool generally found on Unix-based systems.

```shell
$Env:IMAGE_MAGICK_DIR = '<path\to\imagemagick>'
cargo build
cargo test
```

If you are having trouble building on Windows, you are not alone. See the [INSTALL.md](./INSTALL.md) guide for the current state of affairs.

## Documentation

The API documentation is available at [github pages](https://nlfiedler.github.io/magick-rust) since the docs.rs system has a hard time building anything that requires an external library that is not wrapped in a "sys" style library. See [issue 57](https://github.com/nlfiedler/magick-rust/issues/57) for the "create a sys crate request."

## Examples

MagickWand has some global state that needs to be initialized prior to using the library, but fortunately Rust makes handling this pretty easy by use of the `std::sync::Once` type. See the example code in the `examples` directory for the basic usage of the crate.

## Contributing

There are still many missing functions, so if you find there is something you would like to see added to this library, feel free to file an issue. Even better, fork the repo, and write the thin wrapper necessary to expose the MagickWand function. For getters and setters this is often very easy, just add a row to the table in `wand/magick.rs`, and it will work with no additional coding. Tests are optional, as this crate is basically a thin wrapper around code that is assumed to be thoroughly tested already. If you make a change that you want to contribute, please feel free to submit a pull request.

## Docker

[Docker](https://www.docker.com) can be used to build and test the code without affecting your development environment, which may have a different version of ImageMagick installed. The use of `docker compose`, as shown in the example below, is optional, but it makes the process very simple.

```shell
cd docker
docker compose build --pull
docker compose run magick-rust
cargo clean
cargo build
cargo test
```
