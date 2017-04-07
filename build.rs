extern crate bindgen;
extern crate pkg_config;

use std::env;
use std::path::PathBuf;

const MIN_VERSION: &'static str = "6.9";
const MAX_VERSION: &'static str = "6.10";

fn main() {
    let library = pkg_config::Config::new()
        .atleast_version(MIN_VERSION)
        .arg(format!("--max-version={}", MAX_VERSION))
        .probe("MagickWand")
        .unwrap();
    let mut builder = bindgen::Builder::default()
        .no_unstable_rust()
        .emit_builtins()
        .ctypes_prefix("libc")
        .raw_line("extern crate libc;")
        .header("wrapper.h");
    for include_path in library.include_paths {
        builder = builder.clang_arg(format!("-I{}", include_path.to_string_lossy()));
    }
    let bindings = builder.generate().unwrap();
    let outfile = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings.write_to_file(outfile.join("bindings.rs")).unwrap();
}
