/*
 * Copyright 2016-2017 Nathan Fiedler
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
extern crate bindgen;
extern crate pkg_config;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::process::Command;

const MIN_VERSION: &'static str = "6.9";
const MAX_VERSION: &'static str = "6.10";

static HEADER: &'static str = "#include <wand/MagickWand.h>\n";

fn main() {
    // Assert that the appropriate version of MagickWand is installed,
    // since we are very dependent on the particulars of MagickWand.
    pkg_config::Config::new()
        .atleast_version(MIN_VERSION)
        .probe("MagickWand")
        .unwrap();
    // Check the maximum version separately as pkg-config will ignore that
    // option when combined with (certain) other options. And since the
    // pkg-config crate always adds those other flags, we must run the
    // command directly.
    if !Command::new("pkg-config")
                .arg(format!("--max-version={}", MAX_VERSION))
                .arg("MagickWand")
                .status().unwrap().success() {
        panic!("MagickWand version must be no higher than 6.9");
    }
    // We have to split the version check and the cflags/libs check because
    // you can't do both at the same time on RHEL (apparently).
    let library = pkg_config::Config::new().probe("MagickWand").unwrap();

    // If the generated bindings are missing, generate them now.
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let bindings_path_str = out_dir.join("bindings.rs");
    if !Path::new(&bindings_path_str).exists() {

        // Create the header file that rust-bindgen needs as input.
        let gen_h_path = out_dir.join("gen.h");
        let mut gen_h = File::create(&gen_h_path).expect("could not create file");
        gen_h.write_all(HEADER.as_bytes()).expect("could not write header file");

        // Geneate the bindings.
        let mut builder = bindgen::Builder::default()
            .no_unstable_rust()
            .emit_builtins()
            .ctypes_prefix("libc")
            .raw_line("extern crate libc;")
            .header(gen_h_path.to_str().unwrap());
        for include_path in library.include_paths {
            builder = builder.clang_arg(format!("-I{}", include_path.to_string_lossy()));
        }
        if cfg!(target_os = "freebsd") {
            // pkg_config does not seem to work properly on FreeBSD, so
            // hard-code the builder settings for the time being.
            builder = builder.clang_arg("-I/usr/local/include/ImageMagick-6");
            // Need to hack the linker flags as well.
            println!("cargo:rustc-link-lib=dylib=MagickWand-6");
            println!("cargo:rustc-link-search=native=/usr/local/lib");
        }
        let bindings = builder.generate().unwrap();
        let mut file = File::create(&bindings_path_str).expect("could not create bindings file");
        // Work around the include! issue in rustc (as described in the
        // rust-bindgen README file) by wrapping the generated code in a
        // `pub mod` declaration; see issue #359 in (old) rust-bindgen.
        file.write(b"pub mod bindings {\n").unwrap();
        file.write(bindings.to_string().as_bytes()).unwrap();
        file.write(b"\n}").unwrap();

        std::fs::remove_file(&gen_h_path).expect("could not remove header file");
    }
}
