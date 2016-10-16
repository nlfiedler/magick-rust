/*
 * Copyright 2016 Nathan Fiedler
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use std::env;
use std::fs::{copy, File};
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::process::Command;

static HEADER: &'static str = "#include <wand/MagickWand.h>\n";
static LIBPATH: &'static str = "/Library/Developer/CommandLineTools/usr/lib";

fn run_bindgen(out_dir: String, bindings_path_str: &str) {
    // Install rust-bindgen so we can generate the bindings. While the
    // bindgen crate is nice, it does not appear to support setting
    // environment variables like DYLD_LIBRARY_PATH.
    let home_dir = env::home_dir().expect("home directory");
    let mut cargo_bin = PathBuf::from(home_dir);
    cargo_bin.push(".cargo");
    cargo_bin.push("bin");
    cargo_bin.push("bindgen");
    println!("BINDGEN_PATH={:?}", cargo_bin);
    if !cargo_bin.exists() {
        Command::new("cargo")
                .arg("install")
                .arg("bindgen")
                .status().expect("cargo install bindgen");
    }

    // Check that MagickWand is installed before proceeding.
    if !Command::new("pkg-config")
                .arg("--exists")
                .arg("MagickWand")
                .status().unwrap().success() {
        panic!("MagickWand library must be installed");
    }

    // Get the compiler flags for the MagickWand library.
    let mw_cflags_output = Command::new("pkg-config")
            .arg("--cflags")
            .arg("MagickWand")
            .output().expect("pkg-config --cflags MagickWand");
    let mw_cflags = std::str::from_utf8(&mw_cflags_output.stdout).unwrap().trim();
    let mw_cflags_arr: Vec<&str> = mw_cflags.split_whitespace().collect();
    println!("CFLAGS={:?}", mw_cflags_arr);

    // Extract the library name for use with the --link option below.
    // This just happens to be the first argument from the output of
    // pkg-config when given the --libs-only-l option. We lop off the
    // leading "-l" since we only need the name (e.g. "MagickWand-6").
    let mw_ldflags_output = Command::new("pkg-config")
            .arg("--libs-only-l")
            .arg("MagickWand")
            .output().expect("pkg-config --libs-only-l MagickWand");
    let mw_ldflags = std::str::from_utf8(&mw_ldflags_output.stdout).unwrap().trim();
    let mw_ldflags_arr: Vec<&str> = mw_ldflags.split_whitespace().collect();
    let link_arg = &mw_ldflags_arr[0][2..];

    let gen_h_path = out_dir.clone() + "/gen.h";
    // Create the header file that rust-bindgen needs as input.
    let mut gen_h = File::create(&gen_h_path).expect("could not create file");
    gen_h.write_all(HEADER.as_bytes()).expect("could not write header file");

    // Combine all of that in the invocation of rust-bindgen.
    let mut cmd = &mut Command::new(cargo_bin);
    if cfg!(target_os = "macos") {
        // Mac requires that the xcode tools are installed so that
        // rustc can find the clang.dylib file. See also issue
        // https://github.com/crabtw/rust-bindgen/issues/89
        let lib_path = Path::new(LIBPATH);
        if !lib_path.exists() {
            panic!("missing {}, run xcode-select --install", LIBPATH);
        }
        cmd.env("DYLD_LIBRARY_PATH", LIBPATH);
    }
    let mut output_arg = String::from("--output");
    output_arg.push('=');
    output_arg.push_str(&bindings_path_str);
    cmd.arg("--builtins")
       // For the time being, avoid switching to proper Rust enums, as
       // that would change quite a bit of the existing code. We can
       // change that at a later time, and then remove this option.
       .arg("--no-rust-enums")
       // There are a few places where our code is still using `libc::`
       // types, rather than the new default `std::os::raw` types. We
       // can switch at a later time and then remove this option.
       .arg("--ctypes-prefix=libc")
       // Inform bindgen of the library to which we are linking,
       // otherwise it may compile but the tests will fail to link
       // properly. The -L and -l arguments provided by pkg-config
       // apparently go unused with (newer versions of?) clang.
       .arg(format!("--link={}", link_arg))
       .arg(output_arg)
       .arg(&gen_h_path)
       .arg("--");
    if cfg!(target_os = "macos") {
        // Work around issue #361 in rust-bindgen for the time being.
        cmd.arg("-U__BLOCKS__");
    }
    cmd.args(&mw_cflags_arr[..]);
    println!("BINDING_GENERATION={:?}", cmd);
    cmd.status().expect("rust-bindgen invocation");
    std::fs::remove_file(&gen_h_path).expect("could not remove header file");
}

fn assert_mw_version() {
    //
    // So far there is only 6.9 and then 7.x, so comparing to 6.10 should
    // work for now. Such a release may very well work, but we will have to
    // look at that when the time comes.
    //
    if !Command::new("pkg-config")
                .arg("--atleast-version=6.9")
                .arg("MagickWand")
                .status().unwrap().success() {
        panic!("MagickWand version must be at least 6.9");
    }
    if !Command::new("pkg-config")
                .arg("--max-version=6.10")
                .arg("MagickWand")
                .status().unwrap().success() {
        panic!("MagickWand version must be no higher than 6.9");
    }
}

fn main() {
    //
    // Assert that the appropriate version of MagickWand is installed,
    // since we are very dependent on the particulars of MagickWand.
    //
    assert_mw_version();
    //
    // If the MagickWand bindings are missing, generate them using
    // rust-bindgen.
    //
    let out_dir = ::std::env::var("OUT_DIR").unwrap();
    let bindings_path_str = out_dir.clone() + "/bindings.rs";
    if !Path::new(&bindings_path_str).exists() {
        // Either generate the bindings.rs or copy an already generated
        // version for FreeBSD. See rust-bindgen issue #385 for details. We
        // also need to hack the library path to `cargo test` can find the
        // MagickWand libraries.
        if cfg!(target_os = "freebsd") {
            copy("bindings.rs", &bindings_path_str).expect("cp bindings.rs to out_dir");
            println!("cargo:rustc-link-search=native=/usr/local/lib");
        } else {
            run_bindgen(out_dir, &bindings_path_str);
        }
        // Work around the include! issue in rustc (as described in the
        // rust-bindgen README file) by wrapping the generated code in a
        // `pub mod` declaration; see issue #359 in rust-bindgen.
        let mut bind_f = File::open(&bindings_path_str).expect("could not open bindings file");
        let mut bind_text = String::new();
        bind_f.read_to_string(&mut bind_text).expect("could not read bindings file");
        let mut file = File::create(&bindings_path_str).expect("could not create bindings file");
        file.write(b"pub mod bindings {\n").unwrap();
        file.write(bind_text.as_bytes()).unwrap();
        file.write(b"\n}").unwrap();
    }
}
