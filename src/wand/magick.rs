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
use std::ffi::{CStr, CString};
use std::{fmt, ptr, slice};

use libc::c_void;
#[cfg(target_os = "freebsd")]
use libc::size_t;

use bindings;
use conversions::*;
use result::MagickError;
#[cfg(not(target_os = "freebsd"))]
use size_t;

use crate::result::Result;
use super::{MagickTrue, MagickFalse};

use super::{DrawingWand, PixelWand};
use crate::{
    AlphaChannelOption,
    ColorspaceType,
    CompositeOperator,
    CompressionType,
    DisposeType,
    DitherMethod,
    EndianType,
    FilterType,
    GravityType,
    InterlaceType,
    PixelInterpolateMethod,
    MetricType,
    ResourceType
};

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
    pub fn new_image(&self, columns: usize, rows: usize, pixel_wand: &PixelWand) -> Result<()> {
        match unsafe { bindings::MagickNewImage(self.wand, columns.into(), rows.into(), pixel_wand.wand) } {
            MagickTrue => Ok(()),
            _ => Err(MagickError("Could not create image")),
        }
    }

    // opt-in platforms that have resource limits support
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    pub fn set_resource_limit(resource: ResourceType, limit: u64) -> Result<()> {
        let result = unsafe {
            bindings::SetMagickResourceLimit(
                resource.into(),
                limit as bindings::MagickSizeType,
            )
        };
        match result {
            MagickTrue => Ok(()),
            _ => Err(MagickError("failed to set resource limit")),
        }
    }

    pub fn set_option(&mut self, key: &str, value: &str) -> Result<()> {
        let c_key = CString::new(key).unwrap();
        let c_value = CString::new(value).unwrap();
        let result =
            unsafe { bindings::MagickSetOption(self.wand, c_key.as_ptr(), c_value.as_ptr()) };
        match result {
            MagickTrue => Ok(()),
            _ => Err(MagickError("failed to set option")),
        }
    }

    pub fn annotate_image(
        &mut self,
        drawing_wand: &DrawingWand,
        x: f64,
        y: f64,
        angle: f64,
        text: &str,
    ) -> Result<()> {
        let c_string = CString::new(text).map_err(|_| "could not convert to cstring")?;
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
            MagickTrue => Ok(()),
            _ => Err(MagickError("unable to annotate image")),
        }
    }

    /// Add all images from another wand to this wand at the current index.
    pub fn add_image(&mut self, other_wand: &MagickWand) -> Result<()> {
        match unsafe { bindings::MagickAddImage(self.wand, other_wand.wand) } {
            MagickTrue => Ok(()),
            _ => Err(MagickError("unable to add images from another wand")),
        }
    }

    pub fn append_all(&mut self, stack: bool) -> MagickWand {
        unsafe { bindings::MagickResetIterator(self.wand) };
        MagickWand {
            wand: unsafe { bindings::MagickAppendImages(self.wand, stack.to_magick()) },
        }
    }

    pub fn label_image(&self, label: &str) -> Result<()> {
        let c_label = CString::new(label).unwrap();
        let result = unsafe { bindings::MagickLabelImage(self.wand, c_label.as_ptr()) };
        match result {
            MagickTrue => Ok(()),
            _ => Err(MagickError("failed to add label")),
        }
    }

    pub fn write_images(&self, path: &str, adjoin: bool) -> Result<()> {
        let c_name = CString::new(path).unwrap();
        let result =
            unsafe { bindings::MagickWriteImages(self.wand, c_name.as_ptr(), adjoin.to_magick()) };
        match result {
            MagickTrue => Ok(()),
            _ => Err(MagickError("failed to write images")),
        }
    }

    /// Read the image data from the named file.
    pub fn read_image(&self, path: &str) -> Result<()> {
        let c_name = CString::new(path).unwrap();
        let result = unsafe { bindings::MagickReadImage(self.wand, c_name.as_ptr()) };
        match result {
            MagickTrue => Ok(()),
            _ => Err(MagickError("failed to read image")),
        }
    }

    /// Read the image data from the vector of bytes.
    pub fn read_image_blob<T: AsRef<[u8]>>(&self, data: T) -> Result<()> {
        let int_slice = data.as_ref();
        let size = int_slice.len();
        let result = unsafe {
            bindings::MagickReadImageBlob(
                self.wand,
                int_slice.as_ptr() as *const c_void,
                size.into(),
            )
        };
        match result {
            MagickTrue => Ok(()),
            _ => Err(MagickError("failed to read image")),
        }
    }

    /// Same as read_image, but reads only the width, height, size and format of an image,
    /// without reading data.
    pub fn ping_image(&self, path: &str) -> Result<()> {
        let c_name = CString::new(path).unwrap();
        let result = unsafe { bindings::MagickPingImage(self.wand, c_name.as_ptr()) };
        match result {
            MagickTrue => Ok(()),
            _ => Err(MagickError("failed to ping image")),
        }
    }

    /// Same as read_image, but reads only the width, height, size and format of an image,
    /// without reading data.
    pub fn ping_image_blob<T: AsRef<[u8]>>(&self, data: T) -> Result<()> {
        let int_slice = data.as_ref();
        let size = int_slice.len();
        let result = unsafe {
            bindings::MagickPingImageBlob(
                self.wand,
                int_slice.as_ptr() as *const c_void,
                size.into(),
            )
        };
        match result {
            MagickTrue => Ok(()),
            _ => Err(MagickError("failed to ping image")),
        }
    }

    /// Compare two images and return tuple `(distortion, diffImage)`
    /// `diffImage` is `None` if `distortion == 0`
    pub fn compare_images(
        &self,
        reference: &MagickWand,
        metric: MetricType,
    ) -> (f64, Option<MagickWand>) {
        let mut distortion: f64 = 0.0;
        let result = unsafe {
            bindings::MagickCompareImages(self.wand, reference.wand, metric.into(), &mut distortion)
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
        composition_operator: CompositeOperator,
        clip_to_self: bool,
        x: isize,
        y: isize,
    ) -> Result<()> {
        let native_clip_to_self = if clip_to_self {
            MagickTrue
        } else {
            MagickFalse
        };
        let result = unsafe {
            bindings::MagickCompositeImage(
                self.wand,
                reference.wand,
                composition_operator.into(),
                native_clip_to_self,
                x,
                y,
            )
        };
        match result {
            MagickTrue => Ok(()),
            _ => Err(MagickError("failed to compose images")),
        }
    }

    /// Compose another image onto self with gravity using composition_operator
    pub fn compose_images_gravity(
        &self,
        reference: &MagickWand,
        composition_operator: CompositeOperator,
        gravity_type: GravityType,
    ) -> Result<()> {
        let result = unsafe {
            bindings::MagickCompositeImageGravity(
                self.wand,
                reference.wand,
                composition_operator.into(),
                gravity_type.into(),
            )
        };
        match result {
            MagickTrue => Ok(()),
            _ => Err(MagickError("failed to compose images")),
        }
    }

    /// Rebuilds image sequence with each frame size the same as first frame, and composites each frame atop of previous.
    /// Only affects GIF, and other formats with multiple pages/layers.
    pub fn coalesce(&mut self) -> Result<()> {
        let result = unsafe { bindings::MagickCoalesceImages(self.wand) };
        if result.is_null() {
            Err(MagickError("failed to coalesce images"))
        } else {
            self.wand = result;
            Ok(())
        }
    }

    // Replaces colors in the image from a color lookup table.
    pub fn clut_image(
        &self,
        clut_wand: &MagickWand,
        method: PixelInterpolateMethod,
    ) -> Result<()> {
        let result = unsafe { bindings::MagickClutImage(self.wand, clut_wand.wand, method.into()) };
        match result {
            MagickTrue => Ok(()),
            _ => Err(MagickError(
                "failed to replace colors in the image from color lookup table",
            )),
        }
    }

    pub fn hald_clut_image(&self, clut_wand: &MagickWand) -> Result<()> {
        let result = unsafe { bindings::MagickHaldClutImage(self.wand, clut_wand.wand) };
        match result {
            MagickTrue => Ok(()),
            _ => Err(MagickError(
                "failed to replace colors in the image from color lookup table",
            )),
        }
    }

    pub fn fx(&mut self, expression: &str) -> MagickWand {
        let c_expression = CString::new(expression).unwrap();
        let wand = unsafe { bindings::MagickFxImage(self.wand, c_expression.as_ptr()) };
        MagickWand::new_from_wand(wand)
    }

    pub fn set_size(&self, columns: usize, rows: usize) -> Result<()> {
        let result = unsafe { bindings::MagickSetSize(self.wand, columns.into(), rows.into()) };
        match result {
            MagickTrue => Ok(()),
            _ => Err(MagickError("failed to set size of wand")),
        }
    }

    // Define two 'quantum_range' functions because the bindings::QuantumRange symbol
    // is not available if hdri is disabled in the compiled ImageMagick libs
    #[cfg(not(feature = "disable-hdri"))]
    fn quantum_range(&self) -> Result<f64> {
        Ok(bindings::QuantumRange)
    }

    // with disable-hdri enabled we define our own quantum_range
    // values lifted directly from magick-type.h
    #[cfg(feature = "disable-hdri")]
    fn quantum_range(&self) -> Result<f64> {
        match bindings::MAGICKCORE_QUANTUM_DEPTH {
            8 => Ok(255.0f64),
            16 => Ok(65535.0f64),
            32 => Ok(4294967295.0f64),
            64 => Ok(18446744073709551615.0f64),
            _ => Err(MagickError("Quantum depth must be one of 8, 16, 32 or 64")),
        }
    }

    // Level an image. Black and white points are multiplied with QuantumRange to
    // decrease dependencies on the end user.
    pub fn level_image(&self, black_point: f64, gamma: f64, white_point: f64) -> Result<()> {
        let quantum_range = self.quantum_range()?;

        let result = unsafe {
            bindings::MagickLevelImage(
                self.wand,
                black_point * quantum_range,
                gamma,
                white_point * quantum_range,
            )
        };
        match result {
            MagickTrue => Ok(()),
            _ => Err(MagickError("Failed to level the image")),
        }
    }

    //MagickNormalizeImage enhances the contrast of a color image by adjusting the pixels color
    //to span the entire range of colors available
    pub fn normalize_image(
        &self,
    ) -> Result<()> {
        let result = unsafe {
            bindings::MagickNormalizeImage(
                self.wand,
            )
        };
        match result {
            MagickTrue => Ok(()),
            _ => Err(MagickError("Failed to apply contrast stretch to image")),
        }
    }

    //MagickOrderedDitherImage performs an ordered dither based on a number of pre-defined
    //dithering threshold maps, but over multiple intensity levels, which can be different for
    //different channels, according to the input arguments.
    pub fn ordered_dither_image(
        &self,
        threshold_map: &str,
    ) -> Result<()> {
        let c_threshold_map = CString::new(threshold_map).unwrap();

        let result = unsafe {
            bindings::MagickOrderedDitherImage(
                self.wand,
                c_threshold_map.as_ptr(),
            )
        };
        match result {
            MagickTrue => Ok(()),
            _ => Err(MagickError("Failed to apply ordered dither to image")),
        }
    }

    /// Apply sigmoidal contrast to the image
    /// Midpoint is a number in range [0, 1]
    pub fn sigmoidal_contrast_image(
        &self,
        sharpen: bool,
        contrast: f64,
        midpoint: f64,
    ) -> Result<()> {
        let quantum_range = self.quantum_range()?;

        let result = unsafe {
            bindings::MagickSigmoidalContrastImage(
                self.wand,
                sharpen.to_magick(),
                contrast,
                midpoint * quantum_range,
            )
        };
        match result {
            MagickTrue => Ok(()),
            _ => Err(MagickError("Failed to apply sigmoidal contrast to image")),
        }
    }

    /// Extend the image as defined by the geometry, gravity, and wand background color. Set the
    /// (x,y) offset of the geometry to move the original wand relative to the extended wand.
    pub fn extend_image(&self, width: usize, height: usize, x: isize, y: isize) -> Result<()> {
        let result = unsafe { bindings::MagickExtentImage(self.wand, width, height, x, y) };
        match result {
            MagickTrue => Ok(()),
            _ => Err(MagickError("failed to extend image")),
        }
    }

    pub fn profile_image<'a, T: Into<Option<&'a [u8]>>>(
        &self,
        name: &str,
        profile: T,
    ) -> Result<()> {
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
            MagickTrue => Ok(()),
            _ => Err(MagickError("failed to profile image")),
        }
    }

    pub fn strip_image(&self) -> Result<()> {
        let result = unsafe { bindings::MagickStripImage(self.wand) };
        match result {
            MagickTrue => Ok(()),
            _ => Err(MagickError("failed to strip image")),
        }
    }

    pub fn flip_image(&self) -> Result<()> {
        let result = unsafe { bindings::MagickFlipImage(self.wand) };
        match result {
            MagickTrue => Ok(()),
            _ => Err(MagickError("failed to flip image")),
        }
    }

    pub fn negate_image(&self) -> Result<()> {
        let result = unsafe {
            bindings::MagickNegateImage(self.wand, MagickTrue)
        };
        match result {
            MagickTrue => Ok(()),
            _ => Err(MagickError("failed to flip image")),
        }
    }

    pub fn flop_image(&self) -> Result<()> {
        let result = unsafe { bindings::MagickFlopImage(self.wand) };
        match result {
            MagickTrue => Ok(()),
            _ => Err(MagickError("failed to flip image")),
        }
    }

    pub fn blur_image(&self, radius: f64, sigma: f64) -> Result<()> {
        let result = unsafe { bindings::MagickBlurImage(self.wand, radius, sigma) };
        match result {
            MagickTrue => Ok(()),
            _ => Err(MagickError("failed to blur image")),
        }
    }

    pub fn gaussian_blur_image(&self, radius: f64, sigma: f64) -> Result<()> {
        let result = unsafe { bindings::MagickGaussianBlurImage(self.wand, radius, sigma) };
        match result {
            MagickTrue => Ok(()),
            _ => Err(MagickError("failed to gaussian blur image")),
        }
    }

    /// Adaptively resize the currently selected image.
    pub fn adaptive_resize_image(&self, width: usize, height: usize) -> Result<()> {
        match unsafe { bindings::MagickAdaptiveResizeImage(self.wand, width, height) } {
            MagickTrue => Ok(()),
            _ => Err(MagickError("failed to adaptive-resize image")),
        }
    }

    /// Rotate the currently selected image by the given number of degrees,
    /// filling any empty space with the background color of a given PixelWand
    pub fn rotate_image(&self, background: &PixelWand, degrees: f64) -> Result<()> {
        match unsafe { bindings::MagickRotateImage(self.wand, background.wand, degrees) } {
            MagickTrue => Ok(()),
            _ => Err(MagickError("failed to rotate image")),
        }
    }

    /// Trim the image removing the backround color from the edges.
    pub fn trim_image(&self, fuzz: f64) -> Result<()> {
        let result = unsafe { bindings::MagickTrimImage(self.wand, fuzz) };
        match result {
            MagickTrue => Ok(()),
            _ => Err(MagickError("failed to trim image")),
        }
    }

    /// Retrieve the width of the image.
    pub fn get_image_width(&self) -> usize {
        unsafe { bindings::MagickGetImageWidth(self.wand) }
    }

    /// Retrieve the height of the image.
    pub fn get_image_height(&self) -> usize {
        unsafe { bindings::MagickGetImageHeight(self.wand) }
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
    pub fn reset_image_page(&self, page_geometry: &str) -> Result<()> {
        let c_page_geometry = CString::new(page_geometry).unwrap();
        let result = unsafe { bindings::MagickResetImagePage(self.wand, c_page_geometry.as_ptr()) };
        if result == MagickTrue {
            Ok(())
        } else {
            Err(MagickError("Resetting page geometry failed."))
        }
    }

    /// Retrieve the named image property value.
    pub fn get_image_property(&self, name: &str) -> Result<String> {
        let c_name = CString::new(name).unwrap();
        let result = unsafe { bindings::MagickGetImageProperty(self.wand, c_name.as_ptr()) };
        let value = if result.is_null() {
            Err(MagickError("missing property"))
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
    pub fn set_image_property(&self, name: &str, value: &str) -> Result<()> {
        let c_name = CString::new(name).unwrap();
        let c_value = CString::new(value).unwrap();
        let result = unsafe {
            bindings::MagickSetImageProperty(self.wand, c_name.as_ptr(), c_value.as_ptr())
        };
        if result == MagickTrue {
            Ok(())
        } else {
            Err(MagickError("Setting image property failed."))
        }
    }

    /// Returns a `PixelWand` instance for the pixel specified by x and y offests.
    pub fn get_image_pixel_color(&self, x: isize, y: isize) -> Option<PixelWand> {
        let pw = PixelWand::new();

        unsafe {
            if bindings::MagickGetImagePixelColor(self.wand, x, y, pw.wand)
                == MagickTrue
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
    pub fn set_sampling_factors(&self, samplingFactors: &[f64]) -> Result<()> {
        match unsafe {
            bindings::MagickSetSamplingFactors(
                self.wand,
                samplingFactors.len(),
                &samplingFactors[0],
            )
        } {
            MagickTrue => Ok(()),
            _ => Err(MagickError("SetSamplingFactors returned false")),
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
    pub fn sharpen_image(&self, radius: f64, sigma: f64) -> Result<()> {
        match unsafe { bindings::MagickSharpenImage(self.wand, radius, sigma) } {
            MagickTrue => Ok(()),

            _ => Err(MagickError("SharpenImage returned false")),
        }
    }

    /// Set the background color.
    pub fn set_background_color(&self, pixel_wand: &PixelWand) -> Result<()> {
        match unsafe { bindings::MagickSetBackgroundColor(self.wand, pixel_wand.wand) } {
            MagickTrue => Ok(()),

            _ => Err(MagickError("SetBackgroundColor returned false")),
        }
    }

    /// Set the image background color.
    pub fn set_image_background_color(&self, pixel_wand: &PixelWand) -> Result<()> {
        match unsafe { bindings::MagickSetImageBackgroundColor(self.wand, pixel_wand.wand) } {
            MagickTrue => Ok(()),

            _ => Err(MagickError("SetImageBackgroundColor returned false")),
        }
    }

    /// Returns the image resolution as a pair (horizontal resolution, vertical resolution)
    pub fn get_image_resolution(&self) -> Result<(f64, f64)> {
        let mut x_resolution = 0f64;
        let mut y_resolution = 0f64;
        unsafe {
            if bindings::MagickGetImageResolution(self.wand, &mut x_resolution, &mut y_resolution)
                == MagickTrue
            {
                Ok((x_resolution, y_resolution))
            } else {
                Err(MagickError("GetImageResolution returned false"))
            }
        }
    }

    /// Sets the image resolution
    pub fn set_image_resolution(&self, x_resolution: f64, y_resolution: f64) -> Result<()> {
        unsafe {
            if bindings::MagickSetImageResolution(self.wand, x_resolution, y_resolution)
                == MagickTrue
            {
                Ok(())
            } else {
                Err(MagickError("SetImageResolution returned false"))
            }
        }
    }

    /// Sets the wand resolution
    pub fn set_resolution(&self, x_resolution: f64, y_resolution: f64) -> Result<()> {
        unsafe {
            if bindings::MagickSetResolution(self.wand, x_resolution, y_resolution)
                == MagickTrue
            {
                Ok(())
            } else {
                Err(MagickError("SetResolution returned false"))
            }
        }
    }

    /// Returns the image resolution as a pair (horizontal resolution, vertical resolution)
    pub fn sepia_tone_image(&self, threshold: f64) -> Result<()> {
        unsafe {
            if bindings::MagickSepiaToneImage(self.wand, threshold * self.quantum_range()?)
                == MagickTrue
            {
                Ok(())
            } else {
                Err(MagickError("SepiaToneImage returned false"))
            }
        }
    }

    /// Extracts pixel data from the image as a vector of 0..255 values defined by `map`.
    /// See <https://imagemagick.org/api/magick-image.php#MagickExportImagePixels> for more information.
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
        pixels.resize(capacity, 0);

        unsafe {
            if bindings::MagickExportImagePixels(
                self.wand,
                x,
                y,
                width,
                height,
                c_map.as_ptr(),
                bindings::StorageType_CharPixel,
                pixels.as_mut_ptr() as *mut c_void,
            ) == MagickTrue
            {
                Some(pixels)
            } else {
                None
            }
        }
    }

    /// Resize the image to the specified width and height, using the
    /// specified filter type.
    pub fn resize_image(&self, width: usize, height: usize, filter: FilterType) {
        unsafe {
            bindings::MagickResizeImage(self.wand, width.into(), height.into(), filter.into());
        }
    }

    /// Resize the image to the specified width and height, using the
    /// 'thumbnail' optimizations which remove a lot of image meta-data with the goal
    /// of producing small low cost images suited for display on the web.
    pub fn thumbnail_image(&self, width: usize, height: usize) {
        unsafe {
            bindings::MagickThumbnailImage(self.wand, width.into(), height.into());
        }
    }

    /// Extract a region of the image. The width and height is used as the size
    /// of the region. X and Y is the offset.
    pub fn crop_image(&self, width: usize, height: usize, x: isize, y: isize) -> Result<()> {
        let result = unsafe { bindings::MagickCropImage(self.wand, width, height, x, y) };
        match result {
            MagickTrue => Ok(()),
            _ => Err(MagickError("failed to crop image")),
        }
    }

    /// Sample the image to the target resolution
    ///
    /// This is incredibly fast, as it does 1-1 pixel mapping for downscales, and box filtering for
    /// upscales
    pub fn sample_image(&self, width: usize, height: usize) -> Result<()> {
        let result = unsafe { bindings::MagickSampleImage(self.wand, width, height) };
        match result {
            MagickTrue => Ok(()),
            _ => Err(MagickError("failed to sample image")),
        }
    }

    /// Resample the image to the specified horizontal and vertical resolution, using the
    /// specified filter type.
    pub fn resample_image(
        &self,
        x_resolution: f64,
        y_resolution: f64,
        filter: FilterType,
    ) {
        unsafe {
            bindings::MagickResampleImage(self.wand, x_resolution, y_resolution, filter.into());
        }
    }

    /// Rescale the image using seam carving algorithm
    pub fn liquid_rescale_image(
        &self,
        width: usize,
        height: usize,
        delta_x: f64,
        rigidity: f64,
    ) -> Result<()> {
        match unsafe {
            bindings::MagickLiquidRescaleImage(self.wand, width, height, delta_x, rigidity)
        } {
            MagickTrue => Ok(()),
            _ => Err(MagickError("failed to liquid-rescale image")),
        }
    }

    /// Implodes the image towards the center by the specified percentage
    pub fn implode(&self, amount: f64, method: bindings::PixelInterpolateMethod) -> Result<()> {
        match unsafe { bindings::MagickImplodeImage(self.wand, amount, method) } {
            MagickTrue => Ok(()),
            _ => Err(MagickError("failed to implode image")),
        }
    }

    /// Resize the image to fit within the given dimensions, maintaining
    /// the current aspect ratio.
    pub fn fit(&self, width: usize, height: usize) {
        let mut width_ratio = width as f64;
        width_ratio /= self.get_image_width() as f64;
        let mut height_ratio = height as f64;
        height_ratio /= self.get_image_height() as f64;
        let (new_width, new_height) = if width_ratio < height_ratio {
            (
                width,
                (self.get_image_height() as f64 * width_ratio) as usize,
            )
        } else {
            (
                (self.get_image_width() as f64 * height_ratio) as usize,
                height,
            )
        };
        unsafe {
            bindings::MagickResetIterator(self.wand);
            while bindings::MagickNextImage(self.wand) != MagickFalse {
                bindings::MagickResizeImage(
                    self.wand,
                    new_width.into(),
                    new_height.into(),
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
            bindings::MagickAutoOrientImage(self.wand) == MagickTrue
        }
    }

    /// Write the current image to the provided path.
    pub fn write_image(&self, path: &str) -> Result<()> {
        let c_name = CString::new(path).unwrap();
        let result = unsafe { bindings::MagickWriteImage(self.wand, c_name.as_ptr()) };
        match result {
            MagickTrue => Ok(()),
            _ => Err(MagickError("failed to write image")),
        }
    }

    /// Write the image in the desired format to a new blob.
    ///
    /// The `format` argument may be any ImageMagick supported image
    /// format (e.g. GIF, JPEG, PNG, etc).
    pub fn write_image_blob(&self, format: &str) -> Result<Vec<u8>> {
        let c_format = CString::new(format).unwrap();
        let mut length: size_t = 0;
        let blob = unsafe {
            bindings::MagickResetIterator(self.wand);
            bindings::MagickSetImageFormat(self.wand, c_format.as_ptr());
            bindings::MagickGetImageBlob(self.wand, &mut length)
        };
        if blob.is_null() {
            Err(MagickError("failed to write image blob"))
        } else {
            let mut bytes = Vec::with_capacity(length as usize);
            bytes.resize(length, 0);
            unsafe {
                ptr::copy_nonoverlapping(blob, bytes.as_mut_ptr(), length as usize);
                bindings::MagickRelinquishMemory(blob as *mut c_void);
            };
            Ok(bytes)
        }
    }

    /// Write the images in the desired format to a new blob.
    ///
    /// The `format` argument may be any ImageMagick supported image
    /// format (e.g. GIF, JPEG, PNG, etc).
    pub fn write_images_blob(&self, format: &str) -> Result<Vec<u8>> {
        let c_format = CString::new(format).unwrap();
        let mut length: size_t = 0;
        let blob = unsafe {
            bindings::MagickSetIteratorIndex(self.wand, 0);
            bindings::MagickSetImageFormat(self.wand, c_format.as_ptr());
            bindings::MagickGetImagesBlob(self.wand, &mut length)
        };
        let mut bytes = Vec::with_capacity(length as usize);
        bytes.resize(length, 0);
        unsafe {
            ptr::copy_nonoverlapping(blob, bytes.as_mut_ptr(), length as usize);
            bindings::MagickRelinquishMemory(blob as *mut c_void);
        };
        Ok(bytes)
    }

    /// Return false if the image alpha channel is not activated.
    /// That is, the image is RGB rather than RGBA or CMYK rather than CMYKA
    pub fn get_image_alpha_channel(&self) -> bool {
        let res = unsafe { bindings::MagickGetImageAlphaChannel(self.wand) };
        res == MagickTrue
    }

    /// Renders the drawing wand on the current image
    pub fn draw_image(&mut self, drawing_wand: &DrawingWand) -> Result<()> {
        match unsafe { bindings::MagickDrawImage(self.wand, drawing_wand.wand) } {
            MagickTrue => Ok(()),
            _ => Err(MagickError("unable to draw image")),
        }
    }

    /// Removes skew from the image. Skew is an artifact that
    /// occurs in scanned images because of the camera being misaligned,
    /// imperfections in the scanning or surface, or simply because the paper was
    /// not placed completely flat when scanned
    pub fn deskew_image(&mut self, threshold: f64) -> Result<()> {
        match unsafe { bindings::MagickDeskewImage(self.wand, threshold) } {
            MagickTrue => Ok(()),
            _ => Err(MagickError("unable to deskew image")),
        }
    }

    /// Set image channel mask
    pub fn set_image_channel_mask(
        &mut self,
        option: bindings::ChannelType,
    ) -> bindings::ChannelType {
        unsafe { bindings::MagickSetImageChannelMask(self.wand, option) }
    }

    /// Apply an arithmetic, relational, or logical
    /// expression to an image.  Use these operators to lighten or darken an image,
    /// to increase or decrease contrast in an image, or to produce the "negative"
    /// of an image.
    pub fn evaluate_image(&mut self, op: bindings::MagickEvaluateOperator, val: f64) -> Result<()> {
        let res = unsafe { bindings::MagickEvaluateImage(self.wand, op, val) };
        match res {
            MagickTrue => Ok(()),
            _ => Err(MagickError("failed to evaluate image")),
        }
    }

    /// Surround the image with a border of the color defined
    /// by the `pixel_wand`.
    pub fn border_image(
        &self,
        pixel_wand: &PixelWand,
        width: usize,
        height: usize,
        compose: CompositeOperator,
    ) -> Result<()> {
        match unsafe {
            bindings::MagickBorderImage(self.wand, pixel_wand.wand, width, height, compose.into())
        } {
            MagickTrue => Ok(()),

            _ => Err(MagickError("border image returned false")),
        }
    }

    /// Simulate an image shadow
    pub fn shadow_image(&self, alpha: f64, sigma: f64, x: isize, y: isize) -> Result<()> {
        unsafe {
            if bindings::MagickShadowImage(self.wand, alpha, sigma, x, y)
                == MagickTrue
            {
                Ok(())
            } else {
                Err(MagickError("ShadowImage returned false"))
            }
        }
    }

    /// Accepts pixel data and stores it in the image at the location you specify.
    /// See <https://imagemagick.org/api/magick-image.php#MagickImportImagePixels> for more information.
    pub fn import_image_pixels(
        &mut self,
        x: isize,
        y: isize,
        columns: usize,
        rows: usize,
        pixels: &[u8],
        map: &str,
    ) -> Result<()> {
        let pixel_map = CString::new(map).unwrap();
        match unsafe {
            bindings::MagickImportImagePixels(
                self.wand,
                x,
                y,
                columns,
                rows,
                pixel_map.as_ptr(),
                bindings::StorageType_CharPixel,
                pixels.as_ptr() as *const libc::c_void,
            )
        } {
            MagickTrue => Ok(()),
            _ => Err(MagickError("unable to import pixels")),
        }
    }

    /// Set the wand iterator to the first image.
    /// See <https://imagemagick.org/api/magick-wand.php#MagickSetFirstIterator> for more information.
    pub fn set_first_iterator(&self) {
        unsafe {
            bindings::MagickSetFirstIterator(self.wand);
        }
    }

    /// Set the next image in the wand as the current image.
    /// See <https://imagemagick.org/api/magick-image.php#MagickNextImage> for more information.
    pub fn next_image(&self) -> bool {
        let res = unsafe { bindings::MagickNextImage(self.wand) };
        res == MagickTrue
    }

    /// Automatically performs threshold method to reduce grayscale data
    /// down to a binary black & white image. Included algorithms are
    /// Kapur, Otsu, and Triangle methods.
    /// See <https://imagemagick.org/api/magick-image.php#MagickAutoThresholdImage> for more information.
    pub fn auto_threshold(&self, method: bindings::AutoThresholdMethod) -> Result<()> {
        match unsafe { bindings::MagickAutoThresholdImage(self.wand, method) } {
            MagickTrue => Ok(()),
            _ => Err(MagickError("unable to auto threshold image")),
        }
    }

    /// Set the image colorspace, transforming (unlike `set_image_colorspace`) image data in
    /// the process.
    pub fn transform_image_colorspace(&self, colorspace: ColorspaceType) -> Result<()> {
        match unsafe { bindings::MagickTransformImageColorspace(self.wand, colorspace.into()) } {
            MagickTrue => Ok(()),
            _ => Err(MagickError("failed to transform image colorspace")),
        }
    }

    /// Reduce the number of colors in the image.
    pub fn quantize_image(
        &self,
        number_of_colors: usize,
        colorspace: ColorspaceType,
        tree_depth: usize,
        dither_method: DitherMethod,
        measure_error: bool) -> Result<()> {
        match unsafe { bindings::MagickQuantizeImage(
                self.wand,
                number_of_colors.into(),
                colorspace.into(),
                tree_depth.into(),
                dither_method.into(),
                measure_error.to_magick()) } {
            MagickTrue => Ok(()),
            _ => Err(MagickError("failed to quantize image")),
        }
    }

    /// Reduce the number of colors in the images.
    pub fn quantize_images(
        &self,
        number_of_colors: usize,
        colorspace: ColorspaceType,
        tree_depth: usize,
        dither_method: DitherMethod,
        measure_error: bool) -> Result<()> {
        match unsafe { bindings::MagickQuantizeImages(
                self.wand,
                number_of_colors.into(),
                colorspace.into(),
                tree_depth.into(),
                dither_method.into(),
                measure_error.to_magick()) } {
            MagickTrue => Ok(()),
            _ => Err(MagickError("failed to quantize images")),
        }
    }

    mutations!(
        /// Sets the image to the specified alpha level.
        MagickSetImageAlpha => set_image_alpha(alpha: f64)

        /// Control the brightness, saturation, and hue of an image
        MagickModulateImage => modulate_image(brightness: f64, saturation: f64, hue: f64)

        /// Control the brightness and contrast
        MagickBrightnessContrastImage => brightness_contrast_image(brightness: f64, contrast: f64)

        /// Set the image alpha channel mode.
        MagickSetImageAlphaChannel => set_image_alpha_channel(alpha_channel: AlphaChannelOption)

        /// Discard all but one of any pixel color.
        MagickUniqueImageColors => unique_image_colors()

        /// Applies k-means color reduction to the image.
        MagickKmeansImage => kmeans(number_colors: usize, max_iterations: usize, tolerance: f64)

        /// Extracts the 'mean' from the image and adjust the image to try make set its gamma appropriately.
        MagickAutoGammaImage => auto_gamma()

        /// Adjusts the levels of a particular image channel by scaling the minimum and maximum values to the full quantum range.
        MagickAutoLevelImage => auto_level()
    );

    get!(get_image_colors, MagickGetImageColors, usize);

    string_set_get!(
        get_filename,                    set_filename,                    MagickGetFilename,                 MagickSetFilename
        get_font,                        set_font,                        MagickGetFont,                     MagickSetFont
        get_format,                      set_format,                      MagickGetFormat,                   MagickSetFormat
        get_image_filename,              set_image_filename,              MagickGetImageFilename,            MagickSetImageFilename
        get_image_format,                set_image_format,                MagickGetImageFormat,              MagickSetImageFormat
    );

    set_get!(
        get_colorspace,                  set_colorspace,                  MagickGetColorspace,               MagickSetColorspace,              ColorspaceType
        get_image_compose,               set_image_compose,               MagickGetImageCompose,             MagickSetImageCompose,            CompositeOperator
        get_compression,                 set_compression,                 MagickGetCompression,              MagickSetCompression,             CompressionType
        get_compression_quality,         set_compression_quality,         MagickGetCompressionQuality,       MagickSetCompressionQuality,      usize
        get_gravity,                     set_gravity,                     MagickGetGravity,                  MagickSetGravity,                 GravityType
        get_image_colorspace,            set_image_colorspace,            MagickGetImageColorspace,          MagickSetImageColorspace,         ColorspaceType
        get_image_compression,           set_image_compression,           MagickGetImageCompression,         MagickSetImageCompression,        CompressionType
        get_image_compression_quality,   set_image_compression_quality,   MagickGetImageCompressionQuality,  MagickSetImageCompressionQuality, usize
        get_image_delay,                 set_image_delay,                 MagickGetImageDelay,               MagickSetImageDelay,              usize
        get_image_depth,                 set_image_depth,                 MagickGetImageDepth,               MagickSetImageDepth,              usize
        get_image_dispose,               set_image_dispose,               MagickGetImageDispose,             MagickSetImageDispose,            DisposeType
        get_image_endian,                set_image_endian,                MagickGetImageEndian,              MagickSetImageEndian,             EndianType
        get_image_fuzz,                  set_image_fuzz,                  MagickGetImageFuzz,                MagickSetImageFuzz,               f64
        get_image_gamma,                 set_image_gamma,                 MagickGetImageGamma,               MagickSetImageGamma,              f64
        get_image_gravity,               set_image_gravity,               MagickGetImageGravity,             MagickSetImageGravity,            GravityType
        get_image_interlace_scheme,      set_image_interlace_scheme,      MagickGetImageInterlaceScheme,     MagickSetImageInterlaceScheme,    InterlaceType
        get_image_interpolate_method,    set_image_interpolate_method,    MagickGetImageInterpolateMethod,   MagickSetImageInterpolateMethod,  PixelInterpolateMethod
        get_image_iterations,            set_image_iterations,            MagickGetImageIterations,          MagickSetImageIterations,         usize
        get_image_orientation,           set_image_orientation,           MagickGetImageOrientation,         MagickSetImageOrientation,        bindings::OrientationType
        get_image_rendering_intent,      set_image_rendering_intent,      MagickGetImageRenderingIntent,     MagickSetImageRenderingIntent,    bindings::RenderingIntent
        get_image_scene,                 set_image_scene,                 MagickGetImageScene,               MagickSetImageScene,              usize
        get_image_type,                  set_image_type,                  MagickGetImageType,                MagickSetImageType,               bindings::ImageType
        get_image_units,                 set_image_units,                 MagickGetImageUnits,               MagickSetImageUnits,              bindings::ResolutionType
        get_interlace_scheme,            set_interlace_scheme,            MagickGetInterlaceScheme,          MagickSetInterlaceScheme,         InterlaceType
        get_interpolate_method,          set_interpolate_method,          MagickGetInterpolateMethod,        MagickSetInterpolateMethod,       PixelInterpolateMethod
        get_iterator_index,              set_iterator_index,              MagickGetIteratorIndex,            MagickSetIteratorIndex,           isize
        get_orientation,                 set_orientation,                 MagickGetOrientation,              MagickSetOrientation,             bindings::OrientationType
        get_pointsize,                   set_pointsize,                   MagickGetPointsize,                MagickSetPointsize,               f64
        get_type,                        set_type,                        MagickGetType,                     MagickSetType,                    bindings::ImageType
    );
}

impl fmt::Debug for MagickWand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "MagickWand {{")?;
        writeln!(f, "    Exception: {:?}", self.get_exception())?;
        writeln!(f, "    IsWand: {:?}", self.is_wand())?;
        self.fmt_string_settings(f, "    ")?;
        self.fmt_checked_settings(f, "    ")?;
        writeln!(f, "}}")
    }
}
