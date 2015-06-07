# magick-rust

A "safe" Rust interface to the [ImageMagick](http://www.imagemagick.org/) system, in particular, the MagickWand library. The *safe* is in scarequotes because honestly nearly everything is little more than calls into a C library with `unsafe` wrapped around it.

## TODO

1. ~~Use rust-bindgen to generate Rust bindings.~~
1. ~~Add a license and copyright headers~~
1. Develop Rustic wrappers to the MagickWand library.
    * Old Rust bindings: https://github.com/influenza/wand-of-rust
    * Wand API: http://www.imagemagick.org/script/magick-wand.php
1. Write unit tests
1. Test it on lots of images in batches to stress test it; should not crash

## Building the Bindings

To build the ImageMagick bindings, we use [rust-bindgen](https://github.com/crabtw/rust-bindgen), which reads the C header files and produces a suitable wrapper in Rust.

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

Then copy the `~/bindings.rs` file into the `src` directory of this project, and rebuild everything. Hopefully it still works.
