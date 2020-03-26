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
use libc::c_void;
use std::ffi::{CStr, CString};
use std::{fmt, ptr, slice};

use super::{DrawingWand, PixelWand};
use bindings;
use conversions::*;
#[cfg(target_os = "freebsd")]
use libc::{size_t, ssize_t};
#[cfg(not(target_os = "freebsd"))]
use {size_t, ssize_t};

wand_common!(
    MagickWand,
    NewMagickWand,
    ClearMagickWand,
    IsMagickWand,
    CloneMagickWand,
    DestroyMagickWand,
    MagickClearException,
    MagickGetExceptionType,
    MagickGetException
);

/// MagickWand is a Rustic wrapper to the Rust bindings to ImageMagick.
///
/// Instantiating a `MagickWand` will construct an ImageMagick "wand"
/// on which operations can be performed via the `MagickWand` functions.
/// When the `MagickWand` is dropped, the ImageMagick wand will be
/// destroyed as well.
impl MagickWand {
    pub fn new_image(
        &self,
        columns: size_t,
        rows: size_t,
        pixel_wand: &PixelWand,
    ) -> Result<(), &'static str> {
        match unsafe { bindings::MagickNewImage(self.wand, columns, rows, pixel_wand.wand) } {
            bindings::MagickBooleanType_MagickTrue => Ok(()),
            _ => Err("Could not create image"),
        }
    }

    pub fn set_option(&mut self, key: &str, value: &str) -> Result<(), &'static str> {
        let c_key = CString::new(key).unwrap();
        let c_value = CString::new(value).unwrap();
        let result =
            unsafe { bindings::MagickSetOption(self.wand, c_key.as_ptr(), c_value.as_ptr()) };
        match result {
            bindings::MagickBooleanType_MagickTrue => Ok(()),
            _ => Err("failed to set option"),
        }
    }

    pub fn annotate_image(
        &mut self,
        drawing_wand: &DrawingWand,
        x: f64,
        y: f64,
        angle: f64,
        text: &str,
    ) -> Result<(), &'static str> {
        let c_string = try!(CString::new(text).map_err(|_| "could not convert to cstring"));
        match unsafe {
            bindings::MagickAnnotateImage(
                self.wand,
                drawing_wand.wand,
                x,
                y,
                angle,
                c_string.as_ptr() as *const _,
            )
        } {
            bindings::MagickBooleanType_MagickTrue => Ok(()),
            _ => Err("unable to annotate image"),
        }
    }

    /// Add all images from another wand to this wand at the current index.
    pub fn add_image(&mut self, other_wand: &MagickWand) -> Result<(), &'static str> {
        match unsafe { bindings::MagickAddImage(self.wand, other_wand.wand) } {
            bindings::MagickBooleanType_MagickTrue => Ok(()),
            _ => Err("unable to add images from another wand"),
        }
    }

    pub fn append_all(&mut self, stack: bool) -> MagickWand {
        unsafe { bindings::MagickResetIterator(self.wand) };
        MagickWand {
            wand: unsafe { bindings::MagickAppendImages(self.wand, stack.to_magick()) },
        }
    }

    pub fn label_image(&self, label: &str) -> Result<(), &'static str> {
        let c_label = CString::new(label).unwrap();
        let result = unsafe { bindings::MagickLabelImage(self.wand, c_label.as_ptr()) };
        match result {
            bindings::MagickBooleanType_MagickTrue => Ok(()),
            _ => Err("failed to add label"),
        }
    }

    pub fn write_images(&self, path: &str, adjoin: bool) -> Result<(), &'static str> {
        let c_name = CString::new(path).unwrap();
        let result =
            unsafe { bindings::MagickWriteImages(self.wand, c_name.as_ptr(), adjoin.to_magick()) };
        match result {
            bindings::MagickBooleanType_MagickTrue => Ok(()),
            _ => Err("failed to write images"),
        }
    }

    /// Read the image data from the named file.
    pub fn read_image(&self, path: &str) -> Result<(), &'static str> {
        let c_name = CString::new(path).unwrap();
        let result = unsafe { bindings::MagickReadImage(self.wand, c_name.as_ptr()) };
        match result {
            bindings::MagickBooleanType_MagickTrue => Ok(()),
            _ => Err("failed to read image"),
        }
    }

    /// Read the image data from the vector of bytes.
    pub fn read_image_blob<T: AsRef<[u8]>>(&self, data: T) -> Result<(), &'static str> {
        let int_slice = data.as_ref();
        let size = int_slice.len();
        let result = unsafe {
            bindings::MagickReadImageBlob(
                self.wand,
                int_slice.as_ptr() as *const c_void,
                size as size_t,
            )
        };
        match result {
            bindings::MagickBooleanType_MagickTrue => Ok(()),
            _ => Err("failed to read image"),
        }
    }

    /// Same as read_image, but reads only the width, height, size and format of an image,
    /// without reading data.
    pub fn ping_image(&self, path: &str) -> Result<(), &'static str> {
        let c_name = CString::new(path).unwrap();
        let result = unsafe { bindings::MagickPingImage(self.wand, c_name.as_ptr()) };
        match result {
            bindings::MagickBooleanType_MagickTrue => Ok(()),
            _ => Err("failed to ping image"),
        }
    }

    /// Same as read_image, but reads only the width, height, size and format of an image,
    /// without reading data.
    pub fn ping_image_blob<T: AsRef<[u8]>>(&self, data: T) -> Result<(), &'static str> {
        let int_slice = data.as_ref();
        let size = int_slice.len();
        let result = unsafe {
            bindings::MagickPingImageBlob(
                self.wand,
                int_slice.as_ptr() as *const c_void,
                size as size_t,
            )
        };
        match result {
            bindings::MagickBooleanType_MagickTrue => Ok(()),
            _ => Err("failed to ping image"),
        }
    }

    /// Compare two images and return tuple `(distortion, diffImage)`
    /// `diffImage` is `None` if `distortion == 0`
    pub fn compare_images(
        &self,
        reference: &MagickWand,
        metric: bindings::MetricType,
    ) -> (f64, Option<MagickWand>) {
        let mut distortion: f64 = 0.0;
        let result = unsafe {
            bindings::MagickCompareImages(self.wand, reference.wand, metric, &mut distortion)
        };
        let wand = if result.is_null() {
            None
        } else {
            Some(MagickWand { wand: result })
        };
        (distortion, wand)
    }

    /// Compose another image onto self at (x, y) using composition_operator
    pub fn compose_images(
        &self,
        reference: &MagickWand,
        composition_operator: bindings::CompositeOperator,
        clip_to_self: bool,
        x: isize,
        y: isize,
    ) -> Result<(), &'static str> {
        let native_clip_to_self = if clip_to_self {
            bindings::MagickBooleanType_MagickTrue
        } else {
            bindings::MagickBooleanType_MagickFalse
        };
        let result = unsafe {
            bindings::MagickCompositeImage(
                self.wand,
                reference.wand,
                composition_operator,
                native_clip_to_self,
                x,
                y,
            )
        };
        match result {
            bindings::MagickBooleanType_MagickTrue => Ok(()),
            _ => Err("failed to compose images"),
        }
    }

    // Replaces colors in the image from a color lookup table.
    pub fn clut_image(
        &self,
        clut_wand: &MagickWand,
        method: bindings::PixelInterpolateMethod,
    ) -> Result<(), &'static str> {
        let result = unsafe { bindings::MagickClutImage(self.wand, clut_wand.wand, method) };
        match result {
            bindings::MagickBooleanType_MagickTrue => Ok(()),
            _ => Err("failed to replace colors in the image from color lookup table"),
        }
    }

    pub fn set_size(&self, columns: size_t, rows: size_t) -> Result<(), &'static str> {
        let result = unsafe { bindings::MagickSetSize(self.wand, columns, rows) };
        match result {
            bindings::MagickBooleanType_MagickTrue => Ok(()),
            _ => Err("failed to set size of wand"),
        }
    }

    pub fn level_image(
        &self,
        black_point: f64,
        gamma: f64,
        white_point: f64,
    ) -> Result<(), &'static str> {
        let result = unsafe {
            bindings::MagickLevelImage(
                self.wand,
                black_point * bindings::QuantumRange,
                gamma,
                white_point * bindings::QuantumRange,
            )
        };
        match result {
            bindings::MagickBooleanType_MagickTrue => Ok(()),
            _ => Err("failed to set size of wand"),
        }
    }

    /// Extend the image as defined by the geometry, gravity, and wand background color. Set the
    /// (x,y) offset of the geometry to move the original wand relative to the extended wand.
    pub fn extend_image(
        &self,
        width: usize,
        height: usize,
        x: isize,
        y: isize,
    ) -> Result<(), &'static str> {
        let result = unsafe { bindings::MagickExtentImage(self.wand, width, height, x, y) };
        match result {
            bindings::MagickBooleanType_MagickTrue => Ok(()),
            _ => Err("failed to extend image"),
        }
    }

    pub fn profile_image<'a, T: Into<Option<&'a [u8]>>>(
        &self,
        name: &str,
        profile: T,
    ) -> Result<(), &'static str> {
        let c_name = CString::new(name).unwrap();
        let result = unsafe {
            let profile = profile.into();
            let profile_ptr = match profile {
                Some(data) => data.as_ptr(),
                None => ptr::null(),
            } as *const c_void;
            let profile_len = match profile {
                Some(data) => data.len(),
                None => 0,
            };
            bindings::MagickProfileImage(self.wand, c_name.as_ptr(), profile_ptr, profile_len)
        };
        match result {
            bindings::MagickBooleanType_MagickTrue => Ok(()),
            _ => Err("failed to profile image"),
        }
    }

    pub fn flip_image(&self) -> Result<(), &'static str> {
        let result = unsafe { bindings::MagickFlipImage(self.wand) };
        match result {
            bindings::MagickBooleanType_MagickTrue => Ok(()),
            _ => Err("failed to flip image"),
        }
    }

    pub fn flop_image(&self) -> Result<(), &'static str> {
        let result = unsafe { bindings::MagickFlopImage(self.wand) };
        match result {
            bindings::MagickBooleanType_MagickTrue => Ok(()),
            _ => Err("failed to flip image"),
        }
    }

    pub fn gaussian_blur_image(&self, radius: f64, sigma: f64) -> Result<(), &'static str> {
        let result = unsafe { bindings::MagickGaussianBlurImage(self.wand, radius, sigma) };
        match result {
            bindings::MagickBooleanType_MagickTrue => Ok(()),
            _ => Err("failed to blur image"),
        }
    }

    /// Adaptively resize the currently selected image.
    pub fn adaptive_resize_image(&self, width: usize, height: usize) -> Result<(), &'static str> {
        match unsafe { bindings::MagickAdaptiveResizeImage(self.wand, width, height) } {
            bindings::MagickBooleanType_MagickTrue => Ok(()),
            _ => Err("failed to adaptive-resize image"),
        }
    }

    /// Rotate the currently selected image by the given number of degrees,
    /// filling any empty space with the background color of a given PixelWand
    pub fn rotate_image(&self, background: &PixelWand, degrees: f64) -> Result<(), &'static str> {
        match unsafe { bindings::MagickRotateImage(self.wand, background.wand, degrees) } {
            bindings::MagickBooleanType_MagickTrue => Ok(()),
            _ => Err("failed to rotate image"),
        }
    }

    /// Trim the image removing the backround color from the edges.
    pub fn trim_image(&self, fuzz: f64) -> Result<(), &'static str> {
        let result = unsafe { bindings::MagickTrimImage(self.wand, fuzz) };
        match result {
            bindings::MagickBooleanType_MagickTrue => Ok(()),
            _ => Err("failed to trim image"),
        }
    }

    /// Retrieve the width of the image.
    pub fn get_image_width(&self) -> usize {
        unsafe { bindings::MagickGetImageWidth(self.wand) as usize }
    }

    /// Retrieve the height of the image.
    pub fn get_image_height(&self) -> usize {
        unsafe { bindings::MagickGetImageHeight(self.wand) as usize }
    }

    /// Retrieve the page geometry (width, height, x offset, y offset) of the image.
    pub fn get_image_page(&self) -> (usize, usize, isize, isize) {
        let (mut width, mut height, mut x, mut y) = (0usize, 0usize, 0isize, 0isize);
        unsafe {
            bindings::MagickGetImagePage(self.wand, &mut width, &mut height, &mut x, &mut y);
        }
        (width, height, x, y)
    }

    /// Reset the Wand page canvas and position.
    pub fn reset_image_page(&self, page_geometry: &str) -> Result<(), &'static str> {
        let c_page_geometry = CString::new(page_geometry).unwrap();
        let result = unsafe { bindings::MagickResetImagePage(self.wand, c_page_geometry.as_ptr()) };
        if result == bindings::MagickBooleanType_MagickTrue {
            Ok(())
        } else {
            Err("Resetting page geometry failed.")
        }
    }

    /// Retrieve the named image property value.
    pub fn get_image_property(&self, name: &str) -> Result<String, &'static str> {
        let c_name = CString::new(name).unwrap();
        let result = unsafe { bindings::MagickGetImageProperty(self.wand, c_name.as_ptr()) };
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

    /// Set the named image property.
    pub fn set_image_property(&self, name: &str, value: &str) -> Result<(), &'static str> {
        let c_name = CString::new(name).unwrap();
        let c_value = CString::new(value).unwrap();
        let result = unsafe {
            bindings::MagickSetImageProperty(self.wand, c_name.as_ptr(), c_value.as_ptr())
        };
        if result == bindings::MagickBooleanType_MagickTrue {
            Ok(())
        } else {
            Err("Setting image property failed.")
        }
    }

    /// Returns a `PixelWand` instance for the pixel specified by x and y offests.
    pub fn get_image_pixel_color(&self, x: isize, y: isize) -> Option<PixelWand> {
        let pw = PixelWand::new();

        unsafe {
            if bindings::MagickGetImagePixelColor(self.wand, x, y, pw.wand)
                == bindings::MagickBooleanType_MagickTrue
            {
                Some(pw)
            } else {
                None
            }
        }
    }

    /// Sets the image sampling factors.
    ///
    /// samplingFactors: An array of floats representing the sampling factor for each color component (in RGB order).
    pub fn set_sampling_factors(&self, samplingFactors: &[f64]) -> Result<(), &'static str> {
        match unsafe {
            bindings::MagickSetSamplingFactors(
                self.wand,
                samplingFactors.len(),
                &samplingFactors[0],
            )
        } {
            bindings::MagickBooleanType_MagickTrue => Ok(()),
            _ => Err("SetSamplingFactors returned false"),
        }
    }

    /// Returns the image histogram as a vector of `PixelWand` instances for every unique color.
    pub fn get_image_histogram(&self) -> Option<Vec<PixelWand>> {
        let mut color_count: size_t = 0;

        unsafe {
            bindings::MagickGetImageHistogram(self.wand, &mut color_count)
                .as_mut()
                .map(|ptrs| {
                    slice::from_raw_parts(ptrs, color_count)
                        .iter()
                        .map(|raw_wand| PixelWand { wand: *raw_wand })
                        .collect()
                })
        }
    }

    /// Sharpens an image. We convolve the image with a Gaussian operator of the
    /// given radius and standard deviation (sigma). For reasonable results, the
    /// radius should be larger than sigma. Use a radius of 0 and SharpenImage()
    /// selects a suitable radius for you.
    ///
    /// radius: the radius of the Gaussian, in pixels, not counting the center pixel.
    ///
    /// sigma: the standard deviation of the Gaussian, in pixels.
    ///
    pub fn sharpen_image(&self, radius: f64, sigma: f64) -> Result<(), &'static str> {
        match unsafe { bindings::MagickSharpenImage(self.wand, radius, sigma) } {
            bindings::MagickBooleanType_MagickTrue => Ok(()),

            _ => Err("SharpenImage returned false"),
        }
    }

    /// Set the background color.
    pub fn set_background_color(&self, pixel_wand: &PixelWand) -> Result<(), &'static str> {
        match unsafe { bindings::MagickSetBackgroundColor(self.wand, pixel_wand.wand) } {
            bindings::MagickBooleanType_MagickTrue => Ok(()),

            _ => Err("SetBackgroundColor returned false"),
        }
    }

    /// Set the image background color.
    pub fn set_image_background_color(&self, pixel_wand: &PixelWand) -> Result<(), &'static str> {
        match unsafe { bindings::MagickSetImageBackgroundColor(self.wand, pixel_wand.wand) } {
            bindings::MagickBooleanType_MagickTrue => Ok(()),

            _ => Err("SetImageBackgroundColor returned false"),
        }
    }

    /// Returns the image resolution as a pair (horizontal resolution, vertical resolution)
    pub fn get_image_resolution(&self) -> Result<(f64, f64), &'static str> {
        let mut x_resolution = 0f64;
        let mut y_resolution = 0f64;
        unsafe {
            if bindings::MagickGetImageResolution(self.wand, &mut x_resolution, &mut y_resolution)
                == bindings::MagickBooleanType_MagickTrue
            {
                Ok((x_resolution, y_resolution))
            } else {
                Err("GetImageResolution returned false")
            }
        }
    }

    /// Sets the image resolution
    pub fn set_image_resolution(
        &self,
        x_resolution: f64,
        y_resolution: f64,
    ) -> Result<(), &'static str> {
        unsafe {
            if bindings::MagickSetImageResolution(self.wand, x_resolution, y_resolution)
                == bindings::MagickBooleanType_MagickTrue
            {
                Ok(())
            } else {
                Err("SetImageResolution returned false")
            }
        }
    }

    /// Sets the wand resolution
    pub fn set_resolution(&self, x_resolution: f64, y_resolution: f64) -> Result<(), &'static str> {
        unsafe {
            if bindings::MagickSetResolution(self.wand, x_resolution, y_resolution)
                == bindings::MagickBooleanType_MagickTrue
            {
                Ok(())
            } else {
                Err("SetResolution returned false")
            }
        }
    }

    /// Returns the image resolution as a pair (horizontal resolution, vertical resolution)
    pub fn sepia_tone_image(&self, threshold: f64) -> Result<(), &'static str> {
        unsafe {
            if bindings::MagickSepiaToneImage(self.wand, threshold * bindings::QuantumRange)
                == bindings::MagickBooleanType_MagickTrue
            {
                Ok(())
            } else {
                Err("SepiaToneImage returned false")
            }
        }
    }

    /// Extracts pixel data from the image as a vector of 0..255 values defined by `map`.
    /// See https://www.imagemagick.org/api/magick-image.php#MagickExportImagePixels for more information.
    pub fn export_image_pixels(
        &self,
        x: isize,
        y: isize,
        width: usize,
        height: usize,
        map: &str,
    ) -> Option<Vec<u8>> {
        let c_map = CString::new(map).unwrap();
        let capacity = width * height * map.len();
        let mut pixels = Vec::with_capacity(capacity);

        unsafe {
            pixels.set_len(capacity as usize);
            if bindings::MagickExportImagePixels(
                self.wand,
                x,
                y,
                width,
                height,
                c_map.as_ptr(),
                bindings::StorageType_CharPixel,
                pixels.as_mut_ptr() as *mut c_void,
            ) == bindings::MagickBooleanType_MagickTrue
            {
                Some(pixels)
            } else {
                None
            }
        }
    }

    /// Resize the image to the specified width and height, using the
    /// specified filter type.
    pub fn resize_image(&self, width: usize, height: usize, filter: bindings::FilterType) {
        unsafe {
            bindings::MagickResizeImage(self.wand, width as size_t, height as size_t, filter);
        }
    }

    /// Extract a region of the image. The width and height is used as the size
    /// of the region. X and Y is the offset.
    pub fn crop_image(
        &self,
        width: usize,
        height: usize,
        x: isize,
        y: isize,
    ) -> Result<(), &'static str> {
        let result = unsafe { bindings::MagickCropImage(self.wand, width, height, x, y) };
        match result {
            bindings::MagickBooleanType_MagickTrue => Ok(()),
            _ => Err("failed to crop image"),
        }
    }

    /// Resample the image to the specified horizontal and vertical resolution, using the
    /// specified filter type.
    pub fn resample_image(
        &self,
        x_resolution: f64,
        y_resolution: f64,
        filter: bindings::FilterType,
    ) {
        unsafe {
            bindings::MagickResampleImage(self.wand, x_resolution, y_resolution, filter);
        }
    }

    /// Resize the image to fit within the given dimensions, maintaining
    /// the current aspect ratio.
    pub fn fit(&self, width: size_t, height: size_t) {
        let mut width_ratio = width as f64;
        width_ratio /= self.get_image_width() as f64;
        let mut height_ratio = height as f64;
        height_ratio /= self.get_image_height() as f64;
        let (new_width, new_height) = if width_ratio < height_ratio {
            (
                width,
                (self.get_image_height() as f64 * width_ratio) as size_t,
            )
        } else {
            (
                (self.get_image_width() as f64 * height_ratio) as size_t,
                height,
            )
        };
        unsafe {
            bindings::MagickResetIterator(self.wand);
            while bindings::MagickNextImage(self.wand) != bindings::MagickBooleanType_MagickFalse {
                bindings::MagickResizeImage(
                    self.wand,
                    new_width,
                    new_height,
                    bindings::FilterType_LanczosFilter,
                );
            }
        }
    }

    /// Detect if the loaded image is not in top-left orientation, and
    /// hence should be "auto" oriented so it is suitable for viewing.
    pub fn requires_orientation(&self) -> bool {
        unsafe {
            bindings::MagickGetImageOrientation(self.wand)
                != bindings::OrientationType_TopLeftOrientation
        }
    }

    /// Automatically adjusts the loaded image so that its orientation is
    /// suitable for viewing (i.e. top-left orientation).
    ///
    /// Returns `true` if successful or `false` if an error occurred.
    pub fn auto_orient(&self) -> bool {
        unsafe {
            bindings::MagickAutoOrientImage(self.wand) == bindings::MagickBooleanType_MagickTrue
        }
    }

    /// Write the current image to the provided path.
    pub fn write_image(&self, path: &str) -> Result<(), &'static str> {
        let c_name = CString::new(path).unwrap();
        let result = unsafe { bindings::MagickWriteImage(self.wand, c_name.as_ptr()) };
        match result {
            bindings::MagickBooleanType_MagickTrue => Ok(()),
            _ => Err("failed to write image"),
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
            bindings::MagickResetIterator(self.wand);
            bindings::MagickSetImageFormat(self.wand, c_format.as_ptr());
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
            bindings::MagickSetIteratorIndex(self.wand, 0);
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

    mutations!(
        /// Set the image colorspace, transforming (unlike `set_image_colorspace`) image data in
        /// the process.
        MagickTransformImageColorspace => transform_image_colorspace(
            colorspace: bindings::ColorspaceType)

        /// Set the image alpha channel mode.
        MagickSetImageAlphaChannel => set_image_alpha_channel(
            alpha_channel: bindings::AlphaChannelOption)

        /// Reduce the number of colors in the image.
        MagickQuantizeImage => quantize_image(
            number_of_colors: size_t, colorspace: bindings::ColorspaceType,
            tree_depth: size_t, dither_method: bindings::DitherMethod, measure_error: bindings::MagickBooleanType)

        /// Reduce the number of colors in the image.
        MagickQuantizeImages => quantize_images(
            number_of_colors: size_t, colorspace: bindings::ColorspaceType,
            tree_depth: size_t, dither_method: bindings::DitherMethod, measure_error: bindings::MagickBooleanType)

        /// Discard all but one of any pixel color.
        MagickUniqueImageColors => unique_image_colors()
    );

    get!(get_image_colors, MagickGetImageColors, size_t);

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
        get_image_interlace_scheme,      set_image_interlace_scheme,      MagickGetImageInterlaceScheme,     MagickSetImageInterlaceScheme,    bindings::InterlaceType
        get_image_interpolate_method,    set_image_interpolate_method,    MagickGetImageInterpolateMethod,   MagickSetImageInterpolateMethod,  bindings::PixelInterpolateMethod
        get_image_iterations,            set_image_iterations,            MagickGetImageIterations,          MagickSetImageIterations,         size_t
        get_image_orientation,           set_image_orientation,           MagickGetImageOrientation,         MagickSetImageOrientation,        bindings::OrientationType
        get_image_rendering_intent,      set_image_rendering_intent,      MagickGetImageRenderingIntent,     MagickSetImageRenderingIntent,    bindings::RenderingIntent
        get_image_scene,                 set_image_scene,                 MagickGetImageScene,               MagickSetImageScene,              size_t
        get_image_type,                  set_image_type,                  MagickGetImageType,                MagickSetImageType,               bindings::ImageType
        get_image_units,                 set_image_units,                 MagickGetImageUnits,               MagickSetImageUnits,              bindings::ResolutionType
        get_interlace_scheme,            set_interlace_scheme,            MagickGetInterlaceScheme,          MagickSetInterlaceScheme,         bindings::InterlaceType
        get_interpolate_method,          set_interpolate_method,          MagickGetInterpolateMethod,        MagickSetInterpolateMethod,       bindings::PixelInterpolateMethod
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
