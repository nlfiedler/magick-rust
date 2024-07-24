# Installation

The `README.md` covers the basic requirements for building and using this crate. In theory, if you have all of the requisite image libraries, ImageMagick, and Rust installed, then simply adding this crate as a dependency should be enough. If that did not work as expected, then keep reading.

## In an ideal world

If everything is set up just right, then add the crate and proceed as usual.

```
cargo add magick_rust
cargo build
```

But we don't live in an ideal world, so keep reading.

## ImageMagick

Many Linux distributions will have the older **6.x** versions of the ImageMagick library. With the release of **7.0**, ImageMagick introduced some breaking API changes, and that may be why the Linux distros are still using the older versions. This crate only knows how to work with the **7.x** versions of ImageMagick, which means we will be building ImageMagick from source in the examples below.

## Image Libraries

If you build ImageMagick and at some point try to use `magick_rust` only to get the dreaded `failed to read file` error, this is because typical ImageMagick functions return 1 (good) or 0 (bad), which offers no help at all in debugging problems like this. When this error occurs, it almost certainly means that you are missing an image library that ImageMagick relies upon to process the image in question. See the detailed steps below for some examples of installing the popular image libraries, JPEG and PNG.

## Installing on Linux

Install build tools, Clang, some popular image libraries, and ImageMagick. Note that on most Linux distributions the package for ImageMagick is the older **6.x** which is too old for this crate.

```shell
sudo apt-get install build-essential clang pkg-config libjpeg-dev libpng-dev
wget https://imagemagick.org/archive/ImageMagick.tar.gz
tar axf ImageMagick.tar.gz
cd ImageMagick-*
./configure --with-magick-plus-plus=no --with-perl=no
make
sudo make install
cd ..
```

Install Rust, if it is not already installed:

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
. "$HOME/.cargo/env"
```

## Installing on Windows

Currently, the only way to build on Windows is from source, as the `.lib` files have been removed from the binary releases (see [ImageMagick#7272](https://github.com/ImageMagick/ImageMagick/issues/7272)). You will need to follow the below steps carefully.

1. Ensure you have installed Git and LLVM. The easiest way to do this on Windows is by using `winget`:
```powershell
winget install Git.Git
winget install LLVM.LLVM
```
2. Ensure you have installed Visual Studio 2022, with the "C++ MFC for latest build tools (x86 & x64)".
3. Clone the [ImageMagick-Windows](https://github.com/ImageMagick/ImageMagick-Windows) repository to a well known place. The following instructions assume `C:\IM7`, but the choice does not matter:
```powershell
git clone https://github.com/ImageMagick/ImageMagick-Windows C:\IM7
```
4. Run the `CloneRepositories.IM7.cmd` batch file from the source directory, but take care to include the SHA hash of the latest [ImageMagick](https://github.com/ImageMagick/ImageMagick) release (e.g. d775d2a for [7.1.1-35](https://github.com/ImageMagick/ImageMagick/releases/tag/7.1.1-35)):
```powershell
cd C:\IM7
.\CloneRepositories.IM7.cmd d775d2a
```
5. With Visual Studio 2022, open the `C:\IM7\Configure\Configure.sln` solution.
6. Build and run this application. You can use Ctrl+F5 as a shortcut.
7. Using the wizard, configure for "Dynamic Multi-Threaded DLL Runtimes". You can leave everything else as defaults.
8. Open the generated `C:\IM7\IM7.Dynamic.x64.sln` solution.
9. Change the run configuration from Debug to Release mode.
10. Build the solution using "Build Solution" under the "Build" menu, or press Ctrl+Shift+B.
12. Get a cup of coffee, because this will take a while to finish compiling.
13. Search for "Edit the system environment variables" in the Start menu and click on "Environment Variables..."
14. Add the following as system or user environment variables (replacing `C:\IM7` as appropriate):
```ini
IMAGE_MAGICK_DIR=C:\IM7\Output
IMAGE_MAGICK_INCLUDE_DIRS=C:\IM7\ImageMagick
```
15. Add the following directory to your `PATH` variable:
```
C:\IM7\Output\bin
```
16. Once you have restarted your IDE or terminal to pick up on the changes, you may run `cargo build` in your project that includes `magick_rust` to confirm that ImageMagick is linked successfully.

__NOTE:__ Keep in mind that these instructions will *dynamically* link your Rust application with the ImageMagick DLLs. Thus, when distributing your application, you will either need to provide these DLLs (found as `C:\IM7\Output\bin\*_RL_*_.dll`) in the same directory as your executable, or get your users to install the ImageMagick binary distribution.

A set of instructions to enable static linkage of ImageMagick on Windows has yet to be found.

## Creating an Example

Create the example and copy the code that follows into the `src/main.rs` file.

```shell
cargo new --bin mrexample
cd mrexample
cargo add magick_rust
```

You probably do not have a `snow-covered-cat.jpg` so feel free to find a file with that name or change the name to an image file of your choosing. This code is from the `examples/thumbnail-cat.rs` example with some minor changes.

```rust
use magick_rust::{magick_wand_genesis, MagickError, MagickWand};
use std::fs;
use std::sync::Once;

static START: Once = Once::new();

fn resize(filepath: &str) -> Result<Vec<u8>, MagickError> {
    START.call_once(|| {
        magick_wand_genesis();
    });
    let wand = MagickWand::new();
    wand.read_image(filepath)?;
    wand.fit(240, 240);
    wand.write_image_blob("jpeg")
}

fn main() {
    match resize("snow-covered-cat.jpg") {
        Ok(bytes) => {
            fs::write("thumbnail-cat.jpg", bytes).expect("write failed");
        }
        Err(err) => println!("error: {}", err),
    }
}
```

Now we can finally build the example and test it out. Note that an alternative to setting `LD_LIBRARY_PATH` over and over again is to create a file in `/etc/ld.so.conf.d` that has the path `/usr/local/lib` in it.

```shell
export LD_LIBRARY_PATH=/usr/local/lib
cargo build
cargo run
```

Hopefully that produced a `thumbnail-cat.jpg` file.

## Debugging

### Linux builds

Maybe that failed with the "failed to read file" error, in which case you can double-check that the image libraries were found and linked into the final binary. Use the `ldd` tool as shown below to make sure there are no libraries that were "not found". If there are any, make sure to install the requisite library, and then try `ldd` again.

```shell
$ ldd target/debug/mrexample
	linux-vdso.so.1 (0x00007ffee63bd000)
	libMagickWand-7.Q16HDRI.so.10 => /usr/local/lib/libMagickWand-7.Q16HDRI.so.10 (0x00007fd348e52000)
	libMagickCore-7.Q16HDRI.so.10 => /usr/local/lib/libMagickCore-7.Q16HDRI.so.10 (0x00007fd348a54000)
	libgcc_s.so.1 => /lib/x86_64-linux-gnu/libgcc_s.so.1 (0x00007fd348a2d000)
	libc.so.6 => /lib/x86_64-linux-gnu/libc.so.6 (0x00007fd348804000)
	/lib64/ld-linux-x86-64.so.2 (0x00007fd348fc8000)
	libgomp.so.1 => /lib/x86_64-linux-gnu/libgomp.so.1 (0x00007fd3487ba000)
	libm.so.6 => /lib/x86_64-linux-gnu/libm.so.6 (0x00007fd3486d1000)
	libjpeg.so.8 => /lib/x86_64-linux-gnu/libjpeg.so.8 (0x00007fd348650000)
	libpng16.so.16 => /lib/x86_64-linux-gnu/libpng16.so.16 (0x00007fd348615000)
	libxml2.so.2 => /lib/x86_64-linux-gnu/libxml2.so.2 (0x00007fd348433000)
	libz.so.1 => /lib/x86_64-linux-gnu/libz.so.1 (0x00007fd348417000)
	libicuuc.so.70 => /lib/x86_64-linux-gnu/libicuuc.so.70 (0x00007fd34821a000)
	liblzma.so.5 => /lib/x86_64-linux-gnu/liblzma.so.5 (0x00007fd3481ef000)
	libicudata.so.70 => /lib/x86_64-linux-gnu/libicudata.so.70 (0x00007fd3465d1000)
	libstdc++.so.6 => /lib/x86_64-linux-gnu/libstdc++.so.6 (0x00007fd3463a5000)
```
