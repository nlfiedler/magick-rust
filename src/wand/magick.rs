/*
 * Copyright 2016 Mattis Marjak
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
use std::fmt;
use std::ptr;
use std::ffi::{CStr, CString};
use libc::{c_double, c_void};

#[cfg(target_os = "freebsd")]
use libc::{size_t, ssize_t};
#[cfg(not(target_os = "freebsd"))]
use ::{size_t, ssize_t};
use ::bindings;
use ::conversions::*;
use super::{DrawingWand, PixelWand};

wand_common!(
    MagickWand,
    NewMagickWand, ClearMagickWand, IsMagickWand, CloneMagickWand, DestroyMagickWand,
    MagickClearException, MagickGetExceptionType, MagickGetException
);

/// MagickWand is a Rustic wrapper to the Rust bindings to ImageMagick.
///
/// Instantiating a `MagickWand` will construct an ImageMagick "wand"
/// on which operations can be performed via the `MagickWand` functions.
/// When the `MagickWand` is dropped, the ImageMagick wand will be
/// destroyed as well.
impl MagickWand {

    pub fn new_image(&self, columns: size_t, rows: size_t, pixel_wand: &PixelWand) -> Result<(), &'static str> {
        match unsafe { bindings::MagickNewImage(self.wand, columns, rows, pixel_wand.wand) } {
            bindings::MagickBooleanType::MagickTrue => Ok(()),
            _ => Err("Could not create image"),
        }
    }

    pub fn set_option(&mut self, key: &str, value: &str) -> Result<(), &'static str> {
        let c_key = CString::new(key).unwrap();
        let c_value = CString::new(value).unwrap();
        let result = unsafe {
            bindings::MagickSetOption(self.wand, c_key.as_ptr(), c_value.as_ptr())
        };
        match result {
            bindings::MagickBooleanType::MagickTrue => Ok(()),
            _ => Err("failed to set option"),
        }
    }

    pub fn annotate_image(&mut self, drawing_wand: &DrawingWand, x: f64, y: f64, angle: f64, text: &str) -> Result<(), &'static str> {
        let c_string = try!(CString::new(text).map_err(|_| "could not convert to cstring"));
        match unsafe { bindings::MagickAnnotateImage(self.wand, drawing_wand.wand, x, y, angle, c_string.as_ptr() as *const _) } {
            bindings::MagickBooleanType::MagickTrue => Ok(()),
            _ => Err("unable to annotate image")
        }
    }

    pub fn append_all(&mut self, stack: bool) -> MagickWand {
        unsafe { bindings::MagickResetIterator(self.wand) };
        MagickWand {
            wand: unsafe { bindings::MagickAppendImages(self.wand, stack.to_magick()) }
        }
    }

    pub fn label_image(&self, label: &str) -> Result<(), &'static str> {
        let c_label = CString::new(label).unwrap();
        let result = unsafe {
            bindings::MagickLabelImage(self.wand, c_label.as_ptr())
        };
        match result {
            bindings::MagickBooleanType::MagickTrue => Ok(()),
            _ => Err("failed to add label")
        }
    }

    pub fn write_images(&self, path: &str, adjoin: bool) -> Result<(), &'static str> {
        let c_name = CString::new(path).unwrap();
        let result = unsafe {
            bindings::MagickWriteImages(self.wand, c_name.as_ptr(), adjoin.to_magick())
        };
        match result {
            bindings::MagickBooleanType::MagickTrue => Ok(()),
            _ => Err("failed to write images")
        }
    }

    /// Read the image data from the named file.
    pub fn read_image(&self, path: &str) -> Result<(), &'static str> {
        let c_name = CString::new(path).unwrap();
        let result = unsafe {
            bindings::MagickReadImage(self.wand, c_name.as_ptr())
        };
        match result {
            bindings::MagickBooleanType::MagickTrue => Ok(()),
            _ => Err("failed to read image")
        }
    }

    /// Read the image data from the vector of bytes.
    pub fn read_image_blob(&self, data: &Vec<u8>) -> Result<(), &'static str> {
        let int_slice = &data[..];
        let size = data.len();
        let result = unsafe {
            bindings::MagickReadImageBlob(
                self.wand, int_slice.as_ptr() as *const c_void, size as size_t)
        };
        match result {
            bindings::MagickBooleanType::MagickTrue => Ok(()),
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

    /// Retrieve the named image property value.
    pub fn get_image_property(&self, name: &str) -> Result<String, &'static str> {
        let c_name = CString::new(name).unwrap();
        let result = unsafe {
            bindings::MagickGetImageProperty(self.wand, c_name.as_ptr())
        };
        let value = if result.is_null() {
            Err("missing property")
        } else {
            // convert (and copy) the C string to a Rust string
            let cstr = unsafe { CStr::from_ptr(result) };
            Ok(cstr.to_string_lossy().into_owned())
        };
        unsafe {
            bindings::MagickRelinquishMemory(result as *mut c_void);
        }
        value
    }

    /// Resize the image to the specified width and height, using the
    /// specified filter type with the specified blur / sharpness factor.
    ///
    /// blur_factor values greater than 1 create blurriness, while values
    /// less than 1 create sharpness.
    pub fn resize_image(&self, width: usize, height: usize,
                        filter: bindings::FilterTypes, blur_factor: f64) {
        unsafe {
            bindings::MagickResizeImage(
                self.wand, width as size_t, height as size_t,
                filter, blur_factor as c_double
            );
        }
    }

    /// Resize the image to find within the given dimensions, maintaining
    /// the current aspect ratio.
    pub fn fit(&self, width: size_t, height: size_t) {
        let mut width_ratio = width as f64;
        width_ratio /= self.get_image_width() as f64;
        let mut height_ratio = height as f64;
        height_ratio /= self.get_image_height() as f64;
        let new_width: size_t;
        let new_height: size_t;
        if width_ratio < height_ratio {
            new_width = width;
            new_height = (self.get_image_height() as f64 * width_ratio) as size_t;
        } else {
            new_width = (self.get_image_width() as f64 * height_ratio) as size_t;
            new_height = height;
        }
        unsafe {
            bindings::MagickResetIterator(self.wand);
            while bindings::MagickNextImage(self.wand) != bindings::MagickBooleanType::MagickFalse {
                bindings::MagickResizeImage(self.wand, new_width, new_height,
                                            bindings::FilterTypes::LanczosFilter, 1.0);
            }
        }
    }

    /// Detect if the loaded image is not in top-left orientation, and
    /// hence should be "auto" oriented so it is suitable for viewing.
    pub fn requires_orientation(&self) -> bool {
        unsafe {
            bindings::MagickGetImageOrientation(self.wand) != bindings::OrientationType::TopLeftOrientation
        }
    }

    /// Automatically adjusts the loaded image so that its orientation is
    /// suitable for viewing (i.e. top-left orientation).
    ///
    /// Returns `true` if successful or `false` if an error occurred.
    pub fn auto_orient(&self) -> bool {
        unsafe {
            bindings::MagickAutoOrientImage(self.wand) == bindings::MagickBooleanType::MagickTrue
        }
    }

    /// Write the current image to the provided path.
    pub fn write_image(&self, path: &str) -> Result<(), &'static str> {
        let c_name = CString::new(path).unwrap();
        let result = unsafe {
            bindings::MagickWriteImage(self.wand, c_name.as_ptr())
        };
        match result {
            bindings::MagickBooleanType::MagickTrue => Ok(()),
            _ => Err("failed to write image")
        }
    }

    /// Write the image in the desired format to a new blob.
    ///
    /// The `format` argument may be any ImageMagick supported image
    /// format (e.g. GIF, JPEG, PNG, etc).
    pub fn write_image_blob(&self, format: &str) -> Result<Vec<u8>, &'static str> {
        let c_format = CString::new(format).unwrap();
        let mut length: size_t = 0;
        let blob = unsafe {
            bindings::MagickSetImageFormat(self.wand, c_format.as_ptr());
            bindings::MagickResetIterator(self.wand);
            bindings::MagickGetImageBlob(self.wand, &mut length)
        };
        let mut bytes = Vec::with_capacity(length as usize);
        unsafe {
            bytes.set_len(length as usize);
            ptr::copy_nonoverlapping(blob, bytes.as_mut_ptr(), length as usize);
            bindings::MagickRelinquishMemory(blob as *mut c_void);
        };
        Ok(bytes)
    }

    /// Write the images in the desired format to a new blob.
    ///
    /// The `format` argument may be any ImageMagick supported image
    /// format (e.g. GIF, JPEG, PNG, etc).
    pub fn write_images_blob(&self, format: &str) -> Result<Vec<u8>, &'static str> {
        let c_format = CString::new(format).unwrap();
        let mut length: size_t = 0;
        let blob = unsafe {
            bindings::MagickSetImageIndex(self.wand, 0);
            bindings::MagickSetImageFormat(self.wand, c_format.as_ptr());
            bindings::MagickGetImagesBlob(self.wand, &mut length)
        };
        let mut bytes = Vec::with_capacity(length as usize);
        unsafe {
            bytes.set_len(length as usize);
            ptr::copy_nonoverlapping(blob, bytes.as_mut_ptr(), length as usize);
            bindings::MagickRelinquishMemory(blob as *mut c_void);
        };
        Ok(bytes)
    }

    string_set_get!(
        get_filename,                    set_filename,                    MagickGetFilename,                 MagickSetFilename
        get_font,                        set_font,                        MagickGetFont,                     MagickSetFont
        get_format,                      set_format,                      MagickGetFormat,                   MagickSetFormat
        get_image_filename,              set_image_filename,              MagickGetImageFilename,            MagickSetImageFilename
        get_image_format,                set_image_format,                MagickGetImageFormat,              MagickSetImageFormat
    );

    set_get!(
        get_colorspace,                  set_colorspace,                  MagickGetColorspace,               MagickSetColorspace,              bindings::ColorspaceType
        get_compression,                 set_compression,                 MagickGetCompression,              MagickSetCompression,             bindings::CompressionType
        get_compression_quality,         set_compression_quality,         MagickGetCompressionQuality,       MagickSetCompressionQuality,      size_t
        get_gravity,                     set_gravity,                     MagickGetGravity,                  MagickSetGravity,                 bindings::GravityType
        get_image_colorspace,            set_image_colorspace,            MagickGetImageColorspace,          MagickSetImageColorspace,         bindings::ColorspaceType
        get_image_compose,               set_image_compose,               MagickGetImageCompose,             MagickSetImageCompose,            bindings::CompositeOperator
        get_image_compression,           set_image_compression,           MagickGetImageCompression,         MagickSetImageCompression,        bindings::CompressionType
        get_image_compression_quality,   set_image_compression_quality,   MagickGetImageCompressionQuality,  MagickSetImageCompressionQuality, size_t
        get_image_delay,                 set_image_delay,                 MagickGetImageDelay,               MagickSetImageDelay,              size_t
        get_image_depth,                 set_image_depth,                 MagickGetImageDepth,               MagickSetImageDepth,              size_t
        get_image_dispose,               set_image_dispose,               MagickGetImageDispose,             MagickSetImageDispose,            bindings::DisposeType
        get_image_endian,                set_image_endian,                MagickGetImageEndian,              MagickSetImageEndian,             bindings::EndianType
        get_image_fuzz,                  set_image_fuzz,                  MagickGetImageFuzz,                MagickSetImageFuzz,               f64
        get_image_gamma,                 set_image_gamma,                 MagickGetImageGamma,               MagickSetImageGamma,              f64
        get_image_gravity,               set_image_gravity,               MagickGetImageGravity,             MagickSetImageGravity,            bindings::GravityType
        get_image_index,                 set_image_index,                 MagickGetImageIndex,               MagickSetImageIndex,              ssize_t
        get_image_interlace_scheme,      set_image_interlace_scheme,      MagickGetImageInterlaceScheme,     MagickSetImageInterlaceScheme,    bindings::InterlaceType
        get_image_interpolate_method,    set_image_interpolate_method,    MagickGetImageInterpolateMethod,   MagickSetImageInterpolateMethod,  bindings::InterpolatePixelMethod
        get_image_iterations,            set_image_iterations,            MagickGetImageIterations,          MagickSetImageIterations,         size_t
        get_image_orientation,           set_image_orientation,           MagickGetImageOrientation,         MagickSetImageOrientation,        bindings::OrientationType
        get_image_rendering_intent,      set_image_rendering_intent,      MagickGetImageRenderingIntent,     MagickSetImageRenderingIntent,    bindings::RenderingIntent
        get_image_scene,                 set_image_scene,                 MagickGetImageScene,               MagickSetImageScene,              size_t
        get_image_type,                  set_image_type,                  MagickGetImageType,                MagickSetImageType,               bindings::ImageType
        get_image_units,                 set_image_units,                 MagickGetImageUnits,               MagickSetImageUnits,              bindings::ResolutionType
        get_interlace_scheme,            set_interlace_scheme,            MagickGetInterlaceScheme,          MagickSetInterlaceScheme,         bindings::InterlaceType
        get_interpolate_method,          set_interpolate_method,          MagickGetInterpolateMethod,        MagickSetInterpolateMethod,       bindings::InterpolatePixelMethod
        get_iterator_index,              set_iterator_index,              MagickGetIteratorIndex,            MagickSetIteratorIndex,           ssize_t
        get_orientation,                 set_orientation,                 MagickGetOrientation,              MagickSetOrientation,             bindings::OrientationType
        get_pointsize,                   set_pointsize,                   MagickGetPointsize,                MagickSetPointsize,               f64
        get_type,                        set_type,                        MagickGetType,                     MagickSetType,                    bindings::ImageType
    );

}

impl fmt::Debug for MagickWand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(writeln!(f, "MagickWand {{"));
        try!(writeln!(f, "    Exception: {:?}", self.get_exception()));
        try!(writeln!(f, "    IsWand: {:?}", self.is_wand()));
        try!(self.fmt_string_settings(f, "    "));
        try!(self.fmt_checked_settings(f, "    "));
        writeln!(f, "}}")
    }
}
