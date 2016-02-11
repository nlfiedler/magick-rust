# Development Setup

## Mac OS X

1. Install Xcode
1. Install Homebrew
1. Install Git
1. Install Rust and Cargo
1. Install ImageMagick

```
$ xcode-select --install
$ ruby -e "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/master/install)"
$ brew install git
$ brew install rust
$ brew install imagemagick
```

Then build in the usual manner, as shown in the `README.md` file (i.e. `cargo build` and `cargo test`).

## FreeBSD

1. Install Git
1. Install Rust
1. Install Cargo
1. Install ImageMagick
1. Install the Clang libraries

```
$ sudo pkg install -y git
$ sudo pkg install -y rust
$ sudo pkg install -y cargo
$ sudo pkg install -y ImageMagick-nox11
$ sudo pkg install -y clang-devel
```

### Building

The rust-bindgen tool (or one of its dependencies) needs a little help finding the Clang library during the build process, so set `LIBCLANG_PATH` to the path of `libclang.so`. The steps below work for FreeBSD 10.2.

```
$ setenv LIBCLANG_PATH /usr/local/llvm-devel/lib
$ cargo build
$ cargo test
```

## Ubuntu Linux

1. Install Git
1. Install Rust and Cargo
1. Install ImageMagick
1. Install the Clang libraries

These steps are known to work for Ubuntu Linux 14.04 LTS.

```
$ sudo apt-get install git
$ curl -sSf https://static.rust-lang.org/rustup.sh | sh
$ sudo apt-get build-dep imagemagick
$ wget http://www.imagemagick.org/download/ImageMagick.tar.gz
$ tar zxf ImageMagick.tar.gz
$ cd ImageMagick-*
$ ./configure
$ make
$ sudo make install
$ cd ..
$ sudo apt-get install libclang-dev
```

Then build in the usual manner, as shown in the `README.md` file (i.e. `cargo build` and `cargo test`).
