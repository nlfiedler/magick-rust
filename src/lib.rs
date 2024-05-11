/*
 * Copyright 2015-2017 Nathan Fiedler
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
//!
//! "Safe" wrapper around the low-level bindings to ImageMagick.
//!
//! Prior to using the ImageMagick system, the application should invoke
//! `magick_wand_genesis()`, which maps directly to `MagickWandGenesis`.
//! Likewise, when an application is done using ImageMagick, invoke the
//! `magick_wand_terminus()` function, which maps to `MagickWandTerminus`.
//!

// Make the Rust bindings compile cleanly, despite being very un-Rust-like
// wrappers around C code.
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)] // the siginfo_t in waitid() definition in bindings.rs

extern crate libc;

use libc::size_t;

pub use conversions::ToMagick;
pub use result::MagickError;
use result::Result;
pub use wand::*;
pub use types::*;

mod conversions;
mod result;
mod wand;
mod types;
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

/// This function must be called before any other ImageMagick operations
/// are attempted. This function is safe to be called repeatedly.
pub fn magick_wand_genesis() {
    unsafe {
        match bindings::IsMagickWandInstantiated() {
            bindings::MagickBooleanType_MagickTrue => (),
            _ => bindings::MagickWandGenesis(),
        }
    }
}

/// This function should be called when ImageMagick is no longer needed.
/// This function is safe to be called repeatedly.
pub fn magick_wand_terminus() {
    unsafe {
        if let bindings::MagickBooleanType_MagickTrue = bindings::IsMagickWandInstantiated() {
            bindings::MagickWandTerminus();
        }
    }
}

pub fn magick_query_fonts(pattern: &str) -> Result<Vec<String>> {
    let mut number_fonts: size_t = 0;
    let c_string = ::std::ffi::CString::new(pattern).map_err(|_| "could not convert to cstring")?;
    let ptr =
        unsafe { bindings::MagickQueryFonts(c_string.as_ptr(), &mut number_fonts as *mut size_t) };
    if ptr.is_null() {
        Err(MagickError("null ptr returned by magick_query_fonts"))
    } else {
        let mut v = Vec::new();
        let c_str_ptr_slice = unsafe { ::std::slice::from_raw_parts(ptr, number_fonts as usize) };
        for c_str_ptr in c_str_ptr_slice {
            let c_str = unsafe { ::std::ffi::CStr::from_ptr(*c_str_ptr) };
            v.push(c_str.to_string_lossy().into_owned())
        }
        Ok(v)
    }
}
