# magick-rust

A somewhat safe Rust interface to the [ImageMagick](http://www.imagemagick.org/) system, in particular, the MagickWand library. Many of the functions in the MagickWand API are still missing, and those that are needed will be gradually added.

## Dependencies

* Rust (~latest release)
* Cargo (~latest release)
* ImageMagick (version 7.0)
    - [FreeBSD](https://www.freebsd.org) provides this version
    - [Homebrew](http://brew.sh) requires special steps:
        + `brew install imagemagick`
    - Linux may require building ImageMagick from source
* Clang (version 3.5 or higher)
    - Or whatever version is dictated by [rust-bindgen](https://github.com/servo/rust-bindgen)
* Must have `pkg-config` in order to link with MagickWand.

## Build and Test

Pretty simple for now.

```
$ cargo build
$ cargo test
```

## Example Usage

MagickWand has some global state that needs to be initialized prior to using the library, but fortunately Rust makes handling this pretty easy. In the example below, we read in an image from a file and resize it to fit a square of 240 by 240 pixels, then convert the image to JPEG.

```rust
use magick_rust::{MagickWand, magick_wand_genesis};
use std::sync::{Once, ONCE_INIT};

// Used to make sure MagickWand is initialized exactly once. Note that we
// do not bother shutting down, we simply exit when we're done.
static START: Once = ONCE_INIT;

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

## Docker

[Docker](https://www.docker.com) can be used to build and test the code without
affecting your development environment, which may have a different version of
ImageMagick installed. The use of `docker-compose`, as shown in the example
below, is optional, but it makes the process very simple.

```
$ cd docker
$ docker-compose build
$ docker-compose start
$ docker-compose run magick-rust
$ cargo build
$ cargo test
```
