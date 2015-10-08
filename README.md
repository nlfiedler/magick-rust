# magick-rust

A "safe" Rust interface to the [ImageMagick](http://www.imagemagick.org/) system, in particular, the MagickWand library. The word *safe* is in scarequotes because, honestly, nearly everything is little more than a call into a C function with `unsafe` wrapped around it.

## TODO

1. ~~Use rust-bindgen to generate Rust bindings.~~
1. ~~Add a license and copyright headers~~
1. Develop Rustic wrappers to the MagickWand library.
    * Old Rust bindings: https://github.com/influenza/wand-of-rust
    * Wand API: http://www.imagemagick.org/script/magick-wand.php
1. ~~Write unit tests~~
1. Test it on lots of images in batches to stress test it; should not crash

## Build and Test

Pretty simple for now.

```
$ cargo build
$ cargo test
```

## Example Usage

MagickWand has some global state that needs to be initialized prior to using the library, but fortunately Rust makes handling this pretty easy. In the example below, we read in an image from a file and resize it to fit a square of 240 by 240 pixels, then convert the image to JPEG.

```
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

## Generating Bindings

To generate the ImageMagick bindings, we use [rust-bindgen](https://github.com/crabtw/rust-bindgen), which reads the C header files and produces a suitable wrapper in Rust.

This example is using the [Homebrew](http://brew.sh) installed version of ImageMagick, and the LLVM compiler suite provided in the Command Line Tools from Apple. The only real difference for Mac OS X is the `DYLD_LIBRARY_PATH` that is needed to work around [issue #89](https://github.com/crabtw/rust-bindgen/issues/89) in rust-bindgen. Otherwise, the same basic steps should work on any Rust-supported system.

```
$ git clone https://github.com/crabtw/rust-bindgen.git
$ cd rust-bindgen
$ cargo build
$ echo '#include <wand/MagickWand.h>' > ~/gen.h
$ DYLD_LIBRARY_PATH=/Library/Developer/CommandLineTools/usr/lib \
    ./target/debug/bindgen \
    `MagickWand-config --cflags` \
    -builtins \
    -o ~/bindings.rs \
    `MagickWand-config --ldflags` \
    ~/gen.h
```

Then copy the `~/bindings.rs` file into the `src` directory of this project, and rebuild everything (`cargo clean` and `cargo test`). Hopefully it still works.
