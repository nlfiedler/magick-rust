# magick-rust

A somewhat safe Rust interface to the [ImageMagick](http://www.imagemagick.org/) system, in particular, the MagickWand library. Many of the functions in the MagickWand API are still missing, but over time more will be added. Pull requests are welcome.

## Documentation

Documentation for upstream is hosted on [github pages](https://nlfiedler.github.io/magick-rust).  To build locally run `cargo doc`.

## Dependencies

* Rust stable
* ImageMagick (version 7.0.10-36 to 7.1.x)
    - Does _not_ work with ImageMagick 6.x due to backward incompatible changes.
    - [FreeBSD](https://www.freebsd.org): `sudo pkg install ImageMagick7`
    - [Homebrew](http://brew.sh): `brew install imagemagick`
    - Linux may require building ImageMagick from source, see the `docker/Dockerfile` for an example
    - Windows: download `*-dll` [installer](https://www.imagemagick.org/script/download.php#windows). When installing, check the *Install development headers and libraries for C and C++* checkbox.
* [Clang](https://clang.llvm.org) (version 3.5 or higher)
    - Or whatever version is dictated by [rust-bindgen](https://github.com/rust-lang/rust-bindgen)
* Windows requires MSVC toolchain
* Optionally `pkg-config`, to facilitate linking with ImageMagick. Or you can set linker parameters via environment variables as described in the next section.

## Build and Test

On FreeBSD, Linux, and macOS the following commands should suffice.

```shell
$ cargo build
$ cargo test
```

If `pkg-config` is not available, or you wish to override its behavior, you can set one or more environment variables before building. The `build.rs` script will pick these up and use them instead of trying to invoke the `pkg-config` utility.

* `IMAGE_MAGICK_DIR` - installation path of ImageMagick
* `IMAGE_MAGICK_LIB_DIRS` - list of `lib` directories split by `:`
* `IMAGE_MAGICK_INCLUDE_DIRS` - list of `include` directories split by `:`
* `IMAGE_MAGICK_LIBS` - list of the libraries with which to link

### Build on Windows

When building on Windows, you will need to set the `IMAGE_MAGICK_DIR` environment variable to point to the ImageMagick installation path. Maybe this is possible with the `set` command, but it may be necessary to set the variable in the system preferences. Without setting `IMAGE_MAGICK_DIR`, the `build.rs` script will try to run `pkg-config` which is a tool generally found on Unix-based systems.

```shell
> set IMAGE_MAGICK_DIR=<path to ImageMagick installation directory>
> cargo build
> cargo test
```

### Build Troubleshooting

#### Error: cannot find value `QuantumRange` in module bindings

When attempting to build the library, you might see an error like this one:

```
error[E0425]: cannot find value `QuantumRange` in module `bindings`
   --> C:\Users\charlie\.cargo\registry\src\github.com-1ecc6299db9ec823\magick_rust-0.9.0\src\wand\magick.rs:337:80
    |
337 |             if bindings::MagickSepiaToneImage(self.wand, threshold * bindings::QuantumRange) == bindings::MagickBooleanType::MagickTrue {
    |
     ^^^^^^^^^^^^ not found in `bindings`

error: aborting due to previous error
```

See [issue 40](https://github.com/nlfiedler/magick-rust/issues/40) on GitHub for some background. The issue seems to be that with HDRI disabled, rust-bindgen will not produce the bindings needed for the "quantum range" feature of ImageMagick (see [issue 316](https://github.com/rust-lang/rust-bindgen/issues/316)). To work-around this issue, you can disable HDRI support in your `Cargo.toml` file, like so:

```
magick_rust = { version = "0.17.0", features = ["disable-hdri"] }
```

## Example Usage

MagickWand has some global state that needs to be initialized prior to using the library, but fortunately Rust makes handling this pretty easy. In the example below, we read in an image from a file and resize it to fit a square of 240 by 240 pixels, then convert the image to JPEG.

```rust
use magick_rust::{MagickWand, magick_wand_genesis};
use std::sync::Once;

// Used to make sure MagickWand is initialized exactly once. Note that we
// do not bother shutting down, we simply exit when we're done.
static START: Once = Once::new();

fn resize() -> Result<Vec<u8>, &'static str> {
    START.call_once(|| {
        magick_wand_genesis();
    });
    let wand = MagickWand::new();
    try!(wand.read_image("kittens.jpg"));
    wand.fit(240, 240);
    wand.write_image_blob("jpeg")
}
```

Writing the image to a file rather than an in-memory blob is done by replacing the call to `write_image_blob()` with `write_image()`, which takes a string for the path to the file.

## Frequent API Changes

Because rust-bindgen changes from time to time, and is very difficult to use for a library as large as ImageMagick, the API of this crate may experience dramatic mood swings. Typically this pain manifests itself in the way the enums are represented. I am deeply sorry for this pain. Hopefully someone smarter than me can fix it some day. Pull requests are welcome.

## Contributing

There are still many missing functions, so if you find there is something you would like to see added to this library, feel free to file an issue. Even better, fork the repo, and write the thin wrapper necessary to expose the MagickWand function. For getters and setters this is often very easy, just add a row to the table in `wand/magick.rs`, and it will work with no additional coding. Tests are optional, as this crate is basically a thin wrapper around code that is assumed to be thoroughly tested already. If you make a change that you want to contribute, please feel free to submit a pull request.

## Docker

[Docker](https://www.docker.com) can be used to build and test the code without affecting your development environment, which may have a different version of ImageMagick installed. The use of `docker compose`, as shown in the example below, is optional, but it makes the process very simple.

```shell
$ cd docker
$ docker compose build --pull
$ docker compose run magick-rust
$ cargo clean
$ cargo build
$ cargo test
```
