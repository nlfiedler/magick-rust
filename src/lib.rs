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

use std::ffi::CString;
use libc::{c_uint, size_t, c_double};
use filters::FilterType;

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

    /// Read the image data from the named file.
    pub fn read_image(&self, path: &str) -> Result<(), &'static str> {
        let c_name = CString::new(path).unwrap();
        let result = unsafe {
            bindings::MagickReadImage(self.wand, c_name.as_ptr())
        };
        match result {
            bindings::MagickTrue => Ok(()),
            _ => Err("failed to read image")
        }
    }

    // TODO: make a Rustic wrapper for reading a blob from bytes
    // pub fn MagickReadImageBlob(arg1: *mut MagickWand,
    //                            arg2: *const ::libc::c_void, arg3: size_t)
    //  -> MagickBooleanType;

    /// Retrieve the width of the image.
    pub fn get_image_width(&self) -> usize {
        unsafe {
            bindings::MagickGetImageWidth(self.wand) as usize
        }
    }

    /// Retrieve the height of the image.
    pub fn get_image_height(&self) -> usize {
        unsafe {
            bindings::MagickGetImageHeight(self.wand) as usize
        }
    }

    /// Resize the image to the specified width and height, using the
    /// specified filter type with the specified blur / sharpness factor.
    ///
    /// blur_factor values greater than 1 create blurriness, while values
    /// less than 1 create sharpness.
    pub fn resize_image(&self, width: usize, height: usize,
                        filter: FilterType, blur_factor: f64) {
        unsafe {
            bindings::MagickResizeImage(
                self.wand, width as size_t, height as size_t,
                filter as c_uint, blur_factor as c_double
            );
        }
    }

    // TODO: get the image from the wand somehow (maybe GetImageFromMagickWand())

    /// Write the current image to the provided path.
    pub fn write_image(&self, path: &str) -> Result<(), &'static str> {
        let c_name = CString::new(path).unwrap();
        let result = unsafe {
            bindings::MagickWriteImage(self.wand, c_name.as_ptr())
        };
        match result {
            bindings::MagickTrue => Ok(()),
            _ => Err("failed to write image")
        }
    }
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

pub mod filters {

    use bindings;

    pub enum FilterType {
        UndefinedFilter = bindings::UndefinedFilter as isize,
        PointFilter = bindings::PointFilter as isize,
        BoxFilter = bindings::BoxFilter as isize,
        TriangleFilter = bindings::TriangleFilter as isize,
        HermiteFilter = bindings::HermiteFilter as isize,
        HanningFilter = bindings::HanningFilter as isize,
        HammingFilter = bindings::HammingFilter as isize,
        BlackmanFilter = bindings::BlackmanFilter as isize,
        GaussianFilter = bindings::GaussianFilter as isize,
        QuadraticFilter = bindings::QuadraticFilter as isize,
        CubicFilter = bindings::CubicFilter as isize,
        CatromFilter = bindings::CatromFilter as isize,
        MitchellFilter = bindings::MitchellFilter as isize,
        JincFilter = bindings::JincFilter as isize,
        SincFilter = bindings::SincFilter as isize,
        SincFastFilter = bindings::SincFastFilter as isize,
        KaiserFilter = bindings::KaiserFilter as isize,
        WelshFilter = bindings::WelshFilter as isize,
        ParzenFilter = bindings::ParzenFilter as isize,
        BohmanFilter = bindings::BohmanFilter as isize,
        BartlettFilter = bindings::BartlettFilter as isize,
        LagrangeFilter = bindings::LagrangeFilter as isize,
        LanczosFilter = bindings::LanczosFilter as isize,
        LanczosSharpFilter = bindings::LanczosSharpFilter as isize,
        Lanczos2Filter = bindings::Lanczos2Filter as isize,
        Lanczos2SharpFilter = bindings::Lanczos2SharpFilter as isize,
        RobidouxFilter = bindings::RobidouxFilter as isize,
        RobidouxSharpFilter = bindings::RobidouxSharpFilter as isize,
        CosineFilter = bindings::CosineFilter as isize,
        SplineFilter = bindings::SplineFilter as isize,
        LanczosRadiusFilter = bindings::LanczosRadiusFilter as isize,
        SentinelFilter = bindings::SentinelFilter as isize
    }
}
