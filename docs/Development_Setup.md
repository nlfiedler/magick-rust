# Development Setup

## Mac OS X

[Homebrew](http://brew.sh) is the easiest way to install everything on Mac.

1. Install Xcode
1. Install Homebrew
1. Install Rust and Cargo
1. Install ImageMagick

```
$ xcode-select --install
$ ruby -e "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/master/install)"
$ brew install rust
$ brew install imagemagick@6
$ brew install pkg-config
```

Then build in the usual manner, as shown in the `README.md` file (i.e. `cargo build` and `cargo test`).

## FreeBSD

1. Install Rust
1. Install Cargo
1. Install ImageMagick
1. Install the Clang libraries

See the FreeBSD `fabfile.py` for an example of how to install everything. In particular, note that it may be necessary to set `LIBCLANG_PATH` to the path containing the `libclang.so` library.

Then build in the usual manner, as shown in the `README.md` file (i.e. `cargo build` and `cargo test`).

## Ubuntu Linux

1. Install Rust and Cargo
1. Install ImageMagick
1. Install the Clang libraries

See the Ubuntu `fabfile.py` for an example of how to install everything. In particular, note that it may be necessary to set `LIBCLANG_PATH` to the path containing the `libclang.so` library.

Then build in the usual manner, as shown in the `README.md` file (i.e. `cargo build` and `cargo test`). If running the tests fails because the MagickWand library cannot be found, try rebuilding the ldconfig cache (`sudo ldconfig`).
