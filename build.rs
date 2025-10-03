/*
 * Copyright 2015-2023 Nathan Fiedler
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

use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::process::Command;

const MIN_VERSION: &str = "7.1.1";
const MAX_VERSION: &str = "7.2";

#[cfg(windows)]
static HEADER: &str = r#"
#if !defined(ssize_t) && !defined(__MINGW32__)
#if defined(_WIN64)
typedef __int64 ssize_t;
#else
typedef long ssize_t;
#endif
#endif

#include <MagickWand/MagickWand.h>
"#;
#[cfg(not(windows))]
static HEADER: &str = "#include <MagickWand/MagickWand.h>\n";

//on windows path env always contain : like c:
pub const PATH_SEPARATOR: &str = if cfg!(target_os = "windows") {
    ";"
} else {
    ":"
};

#[derive(Debug)]
struct IgnoreMacros {
    macros_to_ignore: HashSet<String>,
}

impl<S: Into<String>> FromIterator<S> for IgnoreMacros {
    fn from_iter<T: IntoIterator<Item = S>>(macro_names: T) -> Self {
        let mut macros_to_ignore = HashSet::new();
        for macro_name in macro_names {
            macros_to_ignore.insert(macro_name.into());
        }
        Self { macros_to_ignore }
    }
}

impl bindgen::callbacks::ParseCallbacks for IgnoreMacros {
    fn will_parse_macro(&self, name: &str) -> bindgen::callbacks::MacroParsingBehavior {
        if self.macros_to_ignore.contains(name) {
            bindgen::callbacks::MacroParsingBehavior::Ignore
        } else {
            bindgen::callbacks::MacroParsingBehavior::Default
        }
    }
}

#[derive(Debug)]
struct RemoveEnumVariantSuffixes {
    names_to_suffix: HashMap<String, String>,
}

impl<S: Into<String>> FromIterator<(S, S)> for RemoveEnumVariantSuffixes {
    fn from_iter<T: IntoIterator<Item = (S, S)>>(enum_suffix_pairs: T) -> Self {
        let mut names_to_suffix = HashMap::new();
        for (enum_name, variant_suffix) in enum_suffix_pairs {
            names_to_suffix.insert(enum_name.into(), variant_suffix.into());
        }
        Self { names_to_suffix }
    }
}

impl bindgen::callbacks::ParseCallbacks for RemoveEnumVariantSuffixes {
    fn enum_variant_name(
        &self,
        enum_name: Option<&str>,
        original_variant_name: &str,
        _variant_value: bindgen::callbacks::EnumVariantValue,
    ) -> Option<String> {
        let suffix = self.names_to_suffix.get(enum_name?)?;
        Some(original_variant_name.trim_end_matches(suffix).to_string())
    }
}

#[allow(clippy::too_many_lines)]
fn main() {
    let check_cppflags = Command::new("MagickCore-config")
        .arg("--cppflags")
        .output()
        .or_else(|_| {
            Command::new("pkg-config")
                .args(["--cflags", "MagickCore"])
                .output()
        });
    if let Ok(ok_cppflags) = check_cppflags {
        let cppflags = ok_cppflags.stdout;
        let cppflags = String::from_utf8(cppflags).unwrap();
        env_var_set_default("BINDGEN_EXTRA_CLANG_ARGS", &cppflags);
    }

    let lib_dirs = find_image_magick_lib_dirs();
    for d in &lib_dirs {
        assert!(
            d.exists(),
            "ImageMagick library directory does not exist: {}",
            d.to_string_lossy()
        );
        println!("cargo:rustc-link-search=native={}", d.to_string_lossy());
    }

    let include_dirs = find_image_magick_include_dirs();
    for d in &include_dirs {
        assert!(
            d.exists(),
            "ImageMagick include directory does not exist: {}",
            d.to_string_lossy()
        );
        println!("cargo:include={}", d.to_string_lossy());
    }

    println!("cargo:rerun-if-env-changed=IMAGE_MAGICK_LIBS");

    let target = env::var("TARGET").unwrap();
    let libs_env = env::var("IMAGE_MAGICK_LIBS");
    let libs = match libs_env {
        Ok(ref v) => v.split(PATH_SEPARATOR).map(ToOwned::to_owned).collect(),
        Err(_) => {
            if target.contains("windows") {
                vec![
                    "CORE_RL_MagickWand_".to_string(),
                    "CORE_RL_MagickCore_".to_string(),
                ]
            } else if target.contains("freebsd") {
                vec!["MagickWand-7".to_string()]
            } else {
                run_pkg_config().libs
            }
        }
    };

    let kind = determine_mode(&lib_dirs, libs.as_slice());
    for lib in libs {
        println!("cargo:rustc-link-lib={kind}={lib}");
    }

    // If the generated bindings are missing, generate them now.
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let bindings_path_str = out_dir.join("bindings.rs");

    let ignored_macros = IgnoreMacros::from_iter([
        "FP_INFINITE",
        "FP_NAN",
        "FP_NORMAL",
        "FP_SUBNORMAL",
        "FP_ZERO",
        "IPPORT_RESERVED",
        "FP_INT_UPWARD",
        "FP_INT_DOWNWARD",
        "FP_INT_TOWARDZERO",
        "FP_INT_TONEARESTFROMZERO",
        "FP_INT_TONEAREST",
    ]);

    let remove_enum_suffixes = RemoveEnumVariantSuffixes::from_iter([
        ("ClassType", "Class"),
        ("CompositeOperator", "CompositeOp"),
        ("GravityType", "Gravity"),
        ("ImageType", "Type"),
        ("InterlaceType", "Interlace"),
        ("OrientationType", "Orientation"),
        ("ResolutionType", "Resolution"),
        ("TransmitType", "TransmitType"),
        ("MapMode", "Mode"),
        ("ColorspaceType", "Colorspace"),
        ("ChannelType", "Channel"),
        ("PixelChannel", "PixelChannel"),
        ("PixelIntensityMethod", "PixelIntensityMethod"),
        ("PixelInterpolateMethod", "InterpolatePixel"),
        ("PixelMask", "PixelMask"),
        ("PixelTrait", "PixelTrait"),
        ("VirtualPixelMethod", "VirtualPixelMethod"),
        ("ComplianceType", "Compliance"),
        ("IlluminantType", "Illuminant"),
        ("CompressionType", "Compression"),
        ("KernelInfoType", "Kernel"),
        ("MorphologyMethod", "Morphology"),
        ("PreviewType", "Preview"),
        ("DisposeType", "Dispose"),
        ("LayerMethod", "Layer"),
        ("RenderingIntent", "Intent"),
        ("EndianType", "Endian"),
        ("QuantumAlphaType", "QuantumAlpha"),
        ("QuantumFormat", "QuantumFormat"),
        ("QuantumType", "Quantum"),
        ("FilterType", "Filter"),
        ("TimerState", "TimerState"),
        ("StretchType", "Stretch"),
        ("StyleType", "Style"),
        ("AlignType", "Align"),
        ("DecorationType", "Decoration"),
        ("DirectionType", "Direction"),
        ("FillRule", "Rule"),
        ("GradientType", "Gradient"),
        ("LineCap", "Cap"),
        ("LineJoin", "Join"),
        ("PaintMethod", "Method"),
        ("PrimitiveType", "Primitive"),
        ("ReferenceType", "Reference"),
        ("SpreadMethod", "Spread"),
        ("WordBreakType", "WordBreakType"),
        ("CacheType", "Cache"),
        ("AlphaChannelOption", "AlphaChannel"),
        ("MetricType", "ErrorMetric"),
        ("MagickFormatType", "FormatType"),
        ("MagickInfoFlag", "Flag"),
        ("DistortMethod", "Distortion"),
        ("SparseColorMethod", "ColorInterpolate"),
        ("ComplexOperator", "ComplexOperator"),
        ("MontageMode", "Mode"),
        ("MagickCLDeviceType", "DeviceType"),
        ("CommandOption", "Options"),
        ("ValidateType", "Validate"),
        ("CommandOptionFLags", "OptionFlag"),
        ("PolicyDomain", "PolicyDomain"),
        ("PolicyRights", "PolicyRights"),
        ("DitherMethod", "DitherMethod"),
        ("RegistryType", "RegistryType"),
        ("ResourceType", "Resource"),
        ("MagickEvaluateOperator", "EvaluateOperator"),
        ("MagickFunction", "Function"),
        ("StatisticType", "Statistic"),
        ("AutoThresholdMethod", "ThresholdMethod"),
        ("PathType", "Path"),
        ("NoiseType", "Noise"),
    ]);

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
            .raw_line("#![allow(warnings)]")
            .raw_line("#![allow(clippy::all)]")
            .raw_line("use libc;")
            .header(gen_h_path.to_str().unwrap())
            .size_t_is_usize(true)
            .parse_callbacks(Box::new(ignored_macros))
            .parse_callbacks(Box::new(remove_enum_suffixes))
            .blocklist_type("timex")
            .blocklist_function("clock_adjtime")
            .default_enum_style(bindgen::EnumVariation::Rust {
                non_exhaustive: false,
            })
            .derive_eq(true);

        for d in include_dirs {
            builder = builder.clang_arg(format!("-I{}", d.to_string_lossy()));
        }

        let bindings = if cfg!(all(windows, target_pointer_width = "64")) {
            match builder.clone().generate() {
                Ok(bindings) => bindings,
                Err(bindgen::BindgenError::ClangDiagnostic(err_msg)) if err_msg.contains("C++") => {
                    builder.clang_arg("-xc++").generate().unwrap()
                }
                Err(err) => panic!("{err:?}"),
            }
        } else {
            builder.generate().unwrap()
        };
        let mut file = File::create(&bindings_path_str).expect("could not create bindings file");
        // Work around the include! issue in rustc (as described in the
        // rust-bindgen README file) by wrapping the generated code in a
        // `pub mod` declaration; see issue #359 in (old) rust-bindgen.
        file.write_all(b"pub mod bindings {\n").unwrap();
        file.write_all(bindings.to_string().as_bytes()).unwrap();
        file.write_all(b"\n}").unwrap();

        std::fs::remove_file(&gen_h_path).expect("could not remove header file");
    }
}

fn env_var_set_default(name: &str, value: &str) {
    if env::var(name).is_err() {
        unsafe {
            env::set_var(name, value);
        }
    }
}

fn find_image_magick_lib_dirs() -> Vec<PathBuf> {
    println!("cargo:rerun-if-env-changed=IMAGE_MAGICK_LIB_DIRS");
    env::var("IMAGE_MAGICK_LIB_DIRS")
        .map(|x| {
            x.split(PATH_SEPARATOR)
                .map(PathBuf::from)
                .collect::<Vec<PathBuf>>()
        })
        .or_else(|_| Ok(vec![find_image_magick_dir()?.join("lib")]))
        .or_else(|_: env::VarError| -> Result<_, env::VarError> { Ok(run_pkg_config().link_paths) })
        .expect("Couldn't find ImageMagick library directory")
}

fn find_image_magick_include_dirs() -> Vec<PathBuf> {
    println!("cargo:rerun-if-env-changed=IMAGE_MAGICK_INCLUDE_DIRS");
    env::var("IMAGE_MAGICK_INCLUDE_DIRS")
        .map(|x| {
            x.split(PATH_SEPARATOR)
                .map(PathBuf::from)
                .collect::<Vec<PathBuf>>()
        })
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
    let kind = env::var("IMAGE_MAGICK_STATIC");
    match kind.as_ref().map(|s| &s[..]) {
        Ok("0") => return "dylib",
        Ok(_) => return "static",
        Err(_) => {}
    }

    // See what files we actually have to link against, and see what our
    // possibilities even are.
    let files = libdirs
        .iter()
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
            let can_static_verbatim = libs.iter().all(|l| files.contains(l.as_ref()));
            if can_static_verbatim {
                return "static:+verbatim";
            }

            panic!(
                "ImageMagick libdirs at `{libdirs:?}` do not contain the required files \
                 to either statically or dynamically link ImageMagick"
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
        .arg(format!("--max-version={MAX_VERSION}"))
        .arg("MagickWand")
        .status()
        .unwrap()
        .success()
    {
        panic!("MagickWand version must be less than {MAX_VERSION}");
    }
    // We have to split the version check and the cflags/libs check because
    // you can't do both at the same time on RHEL (apparently).
    pkg_config::Config::new()
        .cargo_metadata(false)
        .probe("MagickWand")
        .unwrap()
}
