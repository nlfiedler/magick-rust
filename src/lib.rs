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

use std::ffi::CString;
use std::ptr;
use libc::{c_uint, c_double, c_void};
use filters::FilterType;

mod bindings;

/// MagickWand is a Rustic wrapper to the Rust bindings to ImageMagick.
///
/// Instantiating a `MagickWand` will construct an ImageMagick "wand"
/// on which operations can be performed via the `MagickWand` functions.
/// When the `MagickWand` is dropped, the ImageMagick wand will be
/// destroyed as well.
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

    /// Read the image data from the vector of bytes.
    pub fn read_image_blob(&self, data: Vec<u8>) -> Result<(), &'static str> {
        let int_slice = &data[..];
        let size = data.len();
        let result = unsafe {
            bindings::MagickReadImageBlob(
                self.wand, int_slice.as_ptr() as *const c_void, size as u64)
        };
        match result {
            bindings::MagickTrue => Ok(()),
            _ => Err("failed to read image")
        }
    }

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
                self.wand, width as u64, height as u64,
                filter as c_uint, blur_factor as c_double
            );
        }
    }

    /// Resize the image to find within the given dimensions, maintaining
    /// the current aspect ratio.
    pub fn fit(&self, width: usize, height: usize) {
        let mut width_ratio = width as f64;
        width_ratio /= self.get_image_width() as f64;
        let mut height_ratio = height as f64;
        height_ratio /= self.get_image_height() as f64;
        let new_width: usize;
        let new_height: usize;
        if width_ratio < height_ratio {
            new_width = width;
            new_height = (self.get_image_height() as f64 * width_ratio) as usize;
        } else {
            new_width = (self.get_image_width() as f64 * height_ratio) as usize;
            new_height = height;
        }
        unsafe {
            bindings::MagickResetIterator(self.wand);
            while bindings::MagickNextImage(self.wand) != bindings::MagickFalse {
                bindings::MagickResizeImage(self.wand, new_width as u64, new_height as u64,
                                            FilterType::LanczosFilter as c_uint, 1.0);
            }
        }
    }

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

    /// Write the image in the desired format to a new blob.
    ///
    /// The `format` argument may be any ImageMagick supported image
    /// format (e.g. GIF, JPEG, PNG, etc).
    pub fn write_image_blob(&self, format: &str) -> Result<Vec<u8>, &'static str> {
        let c_format = CString::new(format).unwrap();
        let mut length: u64 = 0;
        let blob = unsafe {
            bindings::MagickSetImageFormat(self.wand, c_format.as_ptr());
            bindings::MagickResetIterator(self.wand);
            bindings::MagickGetImageBlob(self.wand, &mut length)
        };
        // would have used Vec::from_raw_buf() but it is unstable
        let mut bytes = Vec::with_capacity(length as usize);
        unsafe {
            bytes.set_len(length as usize);
            ptr::copy_nonoverlapping(blob, bytes.as_mut_ptr(), length as usize);
            bindings::MagickRelinquishMemory(blob as *mut c_void);
        };
        Ok(bytes)
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
