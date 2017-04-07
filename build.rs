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
// extern crate bindgen;

use std::env;
use std::fs::{copy, File};
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
// use bindgen::Builder;

// static HEADER: &'static str = "#include <wand/MagickWand.h>\n";
// static LIBPATH: &'static str = "/Library/Developer/CommandLineTools/usr/lib";

// fn run_bindgen(out_dir: &PathBuf, bindings_path_str: &PathBuf) {
//     // Get the compiler flags for the MagickWand library.
//     let mw_cflags_output = Command::new("pkg-config")
//             .arg("--cflags")
//             .arg("MagickWand")
//             .output().expect("pkg-config --cflags MagickWand");
//     let mw_cflags = std::str::from_utf8(&mw_cflags_output.stdout).unwrap().trim();
//     println!("CFLAGS={:?}", mw_cflags);

//     // Extract the library name for use with the --link option below.
//     // This just happens to be the first argument from the output of
//     // pkg-config when given the --libs-only-l option. We lop off the
//     // leading "-l" since we only need the name (e.g. "MagickWand-6").
//     let mw_ldflags_output = Command::new("pkg-config")
//             .arg("--libs-only-l")
//             .arg("MagickWand")
//             .output().expect("pkg-config --libs-only-l MagickWand");
//     let mw_ldflags = std::str::from_utf8(&mw_ldflags_output.stdout).unwrap().trim();
//     println!("LDFLAGS={:?}", mw_ldflags);
//     let mw_ldflags_arr: Vec<&str> = mw_ldflags.split_whitespace().collect();
//     let link_arg = &mw_ldflags_arr[0][2..];

//     let gen_h_path = out_dir.join("gen.h");
//     // Create the header file that rust-bindgen needs as input.
//     let mut gen_h = File::create(&gen_h_path).expect("could not create file");
//     gen_h.write_all(HEADER.as_bytes()).expect("could not write header file");

//     // TODO: is this still needed for MacOS?
//     //       cmd.env("DYLD_LIBRARY_PATH", LIBPATH);
//     // TODO: need to fix the Rust enums (i.e. no more "--no-rust-enums" option)
//     // TODO: is this still needed for MacOS?
//     //     // Work around issue #361 in rust-bindgen for the time being.
//     //     cmd.arg("-U__BLOCKS__");
//     let bindings = Builder::default()
//         .emit_builtins()
//         .no_unstable_rust()
//         .ctypes_prefix("libc")
//         .header(gen_h_path.to_str().unwrap())
//         .clang_arg(mw_cflags)
//         .link(link_arg)
//         .generate()
//         .expect("Unable to generate bindings");
//     bindings
//         .write_to_file(bindings_path_str)
//         .expect("could not write bindings");

//     std::fs::remove_file(&gen_h_path).expect("could not remove header file");
// }

fn assert_mw_version() {
    // This build script depeneds heavily on the pkg-config utility.
    if !Command::new("which")
                .stdout(Stdio::null())
                .arg("pkg-config")
                .status().unwrap().success() {
        panic!("Cannot find pkg-config, see the README");
    }
    // Check that MagickWand is installed before proceeding.
    if !Command::new("pkg-config")
                .arg("--exists")
                .arg("MagickWand")
                .status().unwrap().success() {
        panic!("MagickWand library must be installed");
    }
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
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let bindings_path_str = out_dir.join("bindings.rs");
    if !Path::new(&bindings_path_str).exists() {
        //
        // TODO: binding generation is broken
        //
        // Using rust-bindgen has become much too difficult for generating
        // MagickWand bindings. There are numerous problems, not the least
        // of which is I cannot get it to find the MagickWand headers.
        //
        if cfg!(target_os = "freebsd") {
            copy("bindings_freebsd.rs", &bindings_path_str).expect("cp bindings.rs to out_dir");
            // We need to hack the library path to `cargo test` can find
            // the MagickWand libraries.
            println!("cargo:rustc-link-search=native=/usr/local/lib");
        } else if cfg!(target_os = "macos") {
            copy("bindings_macos.rs", &bindings_path_str).expect("cp bindings.rs to out_dir");
            // We need to hack the library path to `cargo test` can find
            // the MagickWand libraries.
            println!("cargo:rustc-link-search=native=/usr/local/lib");
        } else {
            // run_bindgen(&out_dir, &bindings_path_str);
            panic!("Sorry, no bindings for you.");
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
