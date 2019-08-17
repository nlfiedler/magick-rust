/*
 * Copyright 2016-2018 Nathan Fiedler
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

use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::process::Command;

const MIN_VERSION: &'static str = "7.0";
const MAX_VERSION: &'static str = "7.1";

static HEADER: &'static str = "#include <MagickWand/MagickWand.h>\n";

fn main() {
    if cfg!(target_os = "freebsd") {
        // pkg_config does not seem to work properly on FreeBSD, so
        // hard-code the builder settings for the time being.
        env_var_set_default(
            "IMAGE_MAGICK_INCLUDE_DIRS",
            "/usr/local/include/ImageMagick-7",
        );
        // Need to hack the linker flags as well.
        env_var_set_default("IMAGE_MAGICK_LIB_DIRS", "/usr/local/lib");
        env_var_set_default("IMAGE_MAGICK_LIBS", "MagickWand-7");
    }

    let lib_dirs = find_image_magick_lib_dirs();
    for d in &lib_dirs {
        if !d.exists() {
            panic!(
                "ImageMagick library directory does not exist: {}",
                d.to_string_lossy()
            );
        }
        println!("cargo:rustc-link-search=native={}", d.to_string_lossy());
    }
    let include_dirs = find_image_magick_include_dirs();
    for d in &include_dirs {
        if !d.exists() {
            panic!(
                "ImageMagick include directory does not exist: {}",
                d.to_string_lossy()
            );
        }
        println!("cargo:include={}", d.to_string_lossy());
    }
    println!("cargo:rerun-if-env-changed=IMAGE_MAGICK_LIBS");

    let target = env::var("TARGET").unwrap();
    let libs_env = env::var("IMAGE_MAGICK_LIBS").ok();
    let libs = match libs_env {
        Some(ref v) => v.split(":").map(|x| x.to_owned()).collect(),
        None => {
            if target.contains("windows") {
                vec!["CORE_RL_MagickWand_".to_string()]
            } else if target.contains("freebsd") {
                vec!["MagickWand-7".to_string()]
            } else {
                run_pkg_config().libs
            }
        }
    };

    let kind = determine_mode(&lib_dirs, libs.as_slice());
    for lib in libs.into_iter() {
        println!("cargo:rustc-link-lib={}={}", kind, lib);
    }

    // If the generated bindings are missing, generate them now.
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let bindings_path_str = out_dir.join("bindings.rs");

    if !Path::new(&bindings_path_str).exists() {
        // Create the header file that rust-bindgen needs as input.
        let gen_h_path = out_dir.join("gen.h");
        let mut gen_h = File::create(&gen_h_path).expect("could not create file");
        gen_h
            .write_all(HEADER.as_bytes())
            .expect("could not write header file");

        // Geneate the bindings.
        let mut builder = bindgen::Builder::default()
            .emit_builtins()
            .ctypes_prefix("libc")
            .raw_line("extern crate libc;")
            .header(gen_h_path.to_str().unwrap())
            // https://github.com/rust-lang-nursery/rust-bindgen/issues/687
            .blacklist_type("FP_NAN")
            .blacklist_type("FP_INFINITE")
            .blacklist_type("FP_ZERO")
            .blacklist_type("FP_SUBNORMAL")
            .blacklist_type("FP_NORMAL")
            .blacklist_type("FP_INT_UPWARD")
            .blacklist_type("FP_INT_DOWNWARD")
            .blacklist_type("FP_INT_TOWARDZERO")
            .blacklist_type("FP_INT_TONEARESTFROMZERO")
            .blacklist_type("FP_INT_TONEAREST");

        for d in include_dirs {
            builder = builder.clang_arg(format!("-I{}", d.to_string_lossy()));
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

fn env_var_set_default(name: &str, value: &str) {
    if env::var(name).is_err() {
        env::set_var(name, value);
    }
}

fn find_image_magick_lib_dirs() -> Vec<PathBuf> {
    println!("cargo:rerun-if-env-changed=IMAGE_MAGICK_LIB_DIRS");
    env::var("IMAGE_MAGICK_LIB_DIRS")
        .map(|x| x.split(":").map(PathBuf::from).collect::<Vec<PathBuf>>())
        .or_else(|_| Ok(vec![find_image_magick_dir()?.join("lib")]))
        .or_else(|_: env::VarError| -> Result<_, env::VarError> { Ok(run_pkg_config().link_paths) })
        .expect("Couldn't find ImageMagick library directory")
}

fn find_image_magick_include_dirs() -> Vec<PathBuf> {
    println!("cargo:rerun-if-env-changed=IMAGE_MAGICK_INCLUDE_DIRS");
    env::var("IMAGE_MAGICK_INCLUDE_DIRS")
        .map(|x| x.split(":").map(PathBuf::from).collect::<Vec<PathBuf>>())
        .or_else(|_| Ok(vec![find_image_magick_dir()?.join("include")]))
        .or_else(|_: env::VarError| -> Result<_, env::VarError> {
            Ok(run_pkg_config().include_paths)
        })
        .expect("Couldn't find ImageMagick include directory")
}

fn find_image_magick_dir() -> Result<PathBuf, env::VarError> {
    println!("cargo:rerun-if-env-changed=IMAGE_MAGICK_DIR");
    env::var("IMAGE_MAGICK_DIR").map(PathBuf::from)
}

fn determine_mode<T: AsRef<str>>(libdirs: &Vec<PathBuf>, libs: &[T]) -> &'static str {
    println!("cargo:rerun-if-env-changed=IMAGE_MAGICK_STATIC");
    let kind = env::var("IMAGE_MAGICK_STATIC").ok();
    match kind.as_ref().map(|s| &s[..]) {
        Some("0") => return "dylib",
        Some(_) => return "static",
        None => {}
    }

    // See what files we actually have to link against, and see what our
    // possibilities even are.
    let files = libdirs
        .into_iter()
        .flat_map(|d| d.read_dir().unwrap())
        .map(|e| e.unwrap())
        .map(|e| e.file_name())
        .filter_map(|e| e.into_string().ok())
        .collect::<HashSet<_>>();
    let can_static = libs.iter().all(|l| {
        files.contains(&format!("lib{}.a", l.as_ref()))
            || files.contains(&format!("{}.lib", l.as_ref()))
    });
    let can_dylib = libs.iter().all(|l| {
        files.contains(&format!("lib{}.so", l.as_ref()))
            || files.contains(&format!("{}.dll", l.as_ref()))
            || files.contains(&format!("lib{}.dylib", l.as_ref()))
    });

    match (can_static, can_dylib) {
        (true, false) => return "static",
        (false, true) => return "dylib",
        (false, false) => {
            panic!(
                "ImageMagick libdirs at `{:?}` do not contain the required files \
                 to either statically or dynamically link ImageMagick",
                libdirs
            );
        }
        (true, true) => {}
    }

    // default
    "dylib"
}

fn run_pkg_config() -> pkg_config::Library {
    // Assert that the appropriate version of MagickWand is installed,
    // since we are very dependent on the particulars of MagickWand.
    pkg_config::Config::new()
        .cargo_metadata(false)
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
        .status()
        .unwrap()
        .success()
    {
        panic!(format!(
            "MagickWand version must be no higher than {}",
            MAX_VERSION
        ));
    }
    // We have to split the version check and the cflags/libs check because
    // you can't do both at the same time on RHEL (apparently).
    pkg_config::Config::new()
        .cargo_metadata(false)
        .probe("MagickWand")
        .unwrap()
}
