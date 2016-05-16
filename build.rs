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

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process::Command;

static HEADER: &'static str = "#include <wand/MagickWand.h>\n";
static LIBPATH: &'static str = "/Library/Developer/CommandLineTools/usr/lib";

fn main() {
    //
    // If the MagickWand bindings are missing, generate them using
    // rust-bindgen.
    //
    let out_dir = ::std::env::var("OUT_DIR").unwrap();
    let bindings_path_str = out_dir.clone() + "/bindings.rs";
    if !Path::new(&bindings_path_str).exists() {
        let bindgen_path_str = out_dir.clone() + "/rust-bindgen";
        let bindgen_path = Path::new(&bindgen_path_str);
        if !bindgen_path.exists() {
            Command::new("git")
                    .arg("clone")
                    .arg("https://github.com/crabtw/rust-bindgen.git")
                    .arg(bindgen_path)
                    .status().unwrap();
            // Checkout a version of rust-bindgen that is known to work;
            // more recent versions produce code that does not compile (the
            // commit after 8a51860 changes the way enums are generated).
            Command::new("git")
                    .arg("checkout")
                    .arg("8a51860")
                    .current_dir(bindgen_path)
                    .status().unwrap();

        }
        let mut bindgen_bin = bindgen_path.to_path_buf();
        bindgen_bin.push("target/debug/bindgen");
        if !bindgen_bin.exists() {
            let mut cmd = Command::new("cargo");
            cmd.arg("build").current_dir(bindgen_path);
            println!("BINDGEN_BUILD={:?}", cmd);
            cmd.status().unwrap();
        }
        // Get the compiler and linker flags for the MagickWand library.
        let mw_cflags_output = Command::new("pkg-config")
                .arg("--cflags")
                .arg("MagickWand")
                .output().unwrap();
        let mw_cflags = std::str::from_utf8(&mw_cflags_output.stdout).unwrap().trim();
        let mw_cflags_arr: Vec<&str> = mw_cflags.split_whitespace().collect();
        println!("CFLAGS={:?}", mw_cflags_arr);
        let mw_ldflags_output = Command::new("pkg-config")
                .arg("--ldflags")
                .arg("MagickWand")
                .output().unwrap();
        let mw_ldflags = std::str::from_utf8(&mw_ldflags_output.stdout).unwrap().trim();
        let mw_ldflags_arr: Vec<&str> = mw_ldflags.split_whitespace().collect();
        println!("LDFLAGS={:?}", mw_ldflags_arr);

        let gen_h_path = out_dir.clone() + "/gen.h";
        // Create the header file that rust-bindgen needs as input.
        let mut gen_h = match File::create(&gen_h_path) {
            Err(why) => panic!("could not create {} file: {}", gen_h_path, Error::description(&why)),
            Ok(file) => file
        };
        match gen_h.write_all(HEADER.as_bytes()) {
            Err(why) => panic!("could not write to {}: {}", gen_h_path, Error::description(&why)),
            Ok(_)    => ()
        };

        // Combine all of that in the invocation of rust-bindgen.
        let mut cmd = &mut Command::new(bindgen_bin);
        if cfg!(target_os = "macos") {
            // Mac requires that the xcode tools are installed so that
            // rustc can find the clang.dylib file. See also issue
            // https://github.com/crabtw/rust-bindgen/issues/89
            let lib_path = Path::new(LIBPATH);
            if !lib_path.exists() {
                panic!("missing {}, run xcode-select --install", LIBPATH);
            }
            cmd.env("DYLD_LIBRARY_PATH", LIBPATH);

            // For the sake of easily building and testing on Mac, include the path
            // to MagickWand. Chances are MagickWand is in /usr/local/lib, or
            // somewhere else that rustc can find it.
            println!("cargo:rustc-link-search=native=/usr/local/lib");
        }
        cmd.args(&mw_cflags_arr[..])
           .arg("-builtins")
           .arg("-o")
           .arg(bindings_path_str)
           .args(&mw_ldflags_arr[..])
           .arg(&gen_h_path);
        println!("BINDING_GENERATION={:?}", cmd);
        cmd.status().unwrap();
        // how to get the output of the command...
        // let output = Commad::new(...).output().unwrap();
        // let out = std::str::from_utf8(&output.stdout).unwrap();
        // println!("cargo:output={}", out);
        // let err = std::str::from_utf8(&output.stderr).unwrap();
        // println!("cargo:error={}", err);
        match std::fs::remove_file(&gen_h_path) {
            Err(why) => panic!("could not remove {}: {}", gen_h_path, Error::description(&why)),
            Ok(_)    => ()
        }
    }
}
