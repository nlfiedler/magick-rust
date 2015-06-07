/*
 * Copyright 2015 Nathan Fiedler
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

// Make the Rust bindings compile cleanly, despite being very un-Rust-like
// wrappers around C code.
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

extern crate libc;

mod bindings;

/// MagickWand is a Rustic wrapper to the Rust bindings to ImageMagick.
pub struct MagickWand {
    wand: *mut bindings::MagickWand
}

impl MagickWand {

    /// Create a new MagickWand instance. This instance will be properly
    /// cleaned up once it falls out of scope.
    pub fn new() -> MagickWand {
        MagickWand {
            wand: unsafe { bindings::NewMagickWand() }
        }
    }

// wand-of-rust wrapper around MagickResizeImage
// pub fn resize_image(&self, width: uint, height: uint,
//                     filter: FilterType, blur_factor: f64) {
//     unsafe {
//         bindings::MagickResizeImage(
//             self.wand, width as size_t, height as size_t,
//             filter as c_uint, blur_factor as c_double
//         );
//     }
// }
}

// Automate safe cleanup for MagickWand instances.
impl Drop for MagickWand {

    /// Clear any exceptions and destroy the magic wand.
    fn drop(&mut self) {
        unsafe {
            bindings::MagickClearException(self.wand);
            bindings::DestroyMagickWand(self.wand);
        }
    }
}

/// This function must be called before any other ImageMagick operations
/// are attempted.
pub fn magick_wand_genesis() {
    unsafe {
        bindings::MagickWandGenesis();
    }
}

/// This function should be called when ImageMagick is no longer needed.
pub fn magick_wand_terminus() {
    unsafe {
        bindings::MagickWandTerminus();
    }
}

#[cfg(test)]
mod test {

    use super::{MagickWand};

    #[test]
    fn test_new_drop() {
        MagickWand::new();
    }
}
