/*
 * Copyright 2015-2016 Nathan Fiedler
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

extern crate libc;

mod wand;
mod bindings { include!(concat!(env!("OUT_DIR"), "/bindings.rs")); }
pub mod filters;

pub use wand::*;


/// This function must be called before any other ImageMagick operations
/// are attempted. This function is safe to be called repeatedly.
pub fn magick_wand_genesis() {
    unsafe {
        match bindings::IsMagickWandInstantiated() {
            bindings::MagickTrue => (),
            _ => bindings::MagickWandGenesis()
        }
    }
}

/// This function should be called when ImageMagick is no longer needed.
/// This function is safe to be called repeatedly.
pub fn magick_wand_terminus() {
    unsafe {
        match bindings::IsMagickWandInstantiated() {
            bindings::MagickTrue => bindings::MagickWandTerminus(),
            _ => ()
        }
    }
}
