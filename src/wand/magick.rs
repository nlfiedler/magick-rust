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

#[cfg(target_os = "freebsd")]
use libc::size_t;
use libc::{c_char, c_uchar, c_void};

use crate::bindings;
use crate::result::MagickError;
#[cfg(not(target_os = "freebsd"))]
use crate::size_t;

use super::{MagickFalse, MagickTrue};
use crate::result::Result;

use super::{DrawingWand, PixelWand};
#[cfg(any(target_os = "linux", target_os = "macos"))]
use crate::ResourceType;
use crate::bindings::MagickBooleanType;
use crate::{
    AlphaChannelOption, AutoThresholdMethod, ChannelType, ColorspaceType, CompositeOperator,
    CompressionType, DisposeType, DitherMethod, EndianType, FilterType, GravityType, Image,
    ImageType, InterlaceType, KernelInfo, LayerMethod, MagickEvaluateOperator, MagickFunction,
    MetricType, MorphologyMethod, OrientationType, PixelInterpolateMethod, PixelMask,
    RenderingIntent, ResolutionType, StatisticType,
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
    /// Creates new wand by cloning the image.
    ///
    /// * `img`: the image.
    pub fn new_from_image(img: &Image<'_>) -> Result<MagickWand> {
        let wand_ptr = unsafe { bindings::NewMagickWandFromImage(img.get_ptr()) };
        Self::result_from_ptr_with_error_message(
            wand_ptr,
            MagickWand::from_ptr,
            "failed to create magick wand from image",
        )
    }

    pub fn new_image(&self, columns: usize, rows: usize, background: &PixelWand) -> Result<()> {
        self.result_from_boolean(unsafe {
            bindings::MagickNewImage(self.wand, columns, rows, background.as_ptr())
        })
    }

    /// opt-in platforms that have resource limits support
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    pub fn set_resource_limit(resource: ResourceType, limit: u64) -> Result<()> {
        Self::result_from_boolean_with_error_message(
            unsafe {
                bindings::MagickSetResourceLimit(resource, limit as bindings::MagickSizeType)
            },
            "failed to set resource limit",
        )
    }

    pub fn set_option(&mut self, key: &str, value: &str) -> Result<()> {
        let c_key = CString::new(key).map_err(|_| "key string contains null byte")?;
        let c_value = CString::new(value).map_err(|_| "value string contains null byte")?;
        self.result_from_boolean(unsafe {
            bindings::MagickSetOption(self.wand, c_key.as_ptr(), c_value.as_ptr())
        })
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
        self.result_from_boolean(unsafe {
            bindings::MagickAnnotateImage(
                self.wand,
                drawing_wand.as_ptr(),
                x,
                y,
                angle,
                c_string.as_ptr() as *const _,
            )
        })
    }

    /// Add all images from another wand to this wand at the current index.
    pub fn add_image(&mut self, other_wand: &MagickWand) -> Result<()> {
        self.result_from_boolean(unsafe { bindings::MagickAddImage(self.wand, other_wand.wand) })
    }

    pub fn append_all(&mut self, stack: bool) -> Result<MagickWand> {
        unsafe { bindings::MagickResetIterator(self.wand) };
        let wand_ptr = unsafe { bindings::MagickAppendImages(self.wand, stack.into()) };
        Self::result_from_ptr_with_error_message(
            wand_ptr,
            MagickWand::from_ptr,
            "failed to append image",
        )
    }

    pub fn label_image(&self, label: &str) -> Result<()> {
        let c_label = CString::new(label).map_err(|_| "label string contains null byte")?;
        self.result_from_boolean(unsafe { bindings::MagickLabelImage(self.wand, c_label.as_ptr()) })
    }

    pub fn write_images(&self, path: &str, adjoin: bool) -> Result<()> {
        let c_name = CString::new(path).map_err(|_| "path string contains null byte")?;
        self.result_from_boolean(unsafe {
            bindings::MagickWriteImages(self.wand, c_name.as_ptr(), adjoin.into())
        })
    }

    /// Read the image data from the named file.
    pub fn read_image(&self, path: &str) -> Result<()> {
        let c_name = CString::new(path).map_err(|_| "path string contains null byte")?;
        self.result_from_boolean(unsafe { bindings::MagickReadImage(self.wand, c_name.as_ptr()) })
    }

    /// Read the image data from the vector of bytes.
    pub fn read_image_blob<T: AsRef<[u8]>>(&self, data: T) -> Result<()> {
        let int_slice = data.as_ref();
        let size = int_slice.len();
        self.result_from_boolean(unsafe {
            bindings::MagickReadImageBlob(self.wand, int_slice.as_ptr() as *const c_void, size)
        })
    }

    /// Same as read_image, but reads only the width, height, size and format of an image,
    /// without reading data.
    pub fn ping_image(&self, path: &str) -> Result<()> {
        let c_name = CString::new(path).map_err(|_| "path string contains null byte")?;
        self.result_from_boolean(unsafe { bindings::MagickPingImage(self.wand, c_name.as_ptr()) })
    }

    /// Same as read_image, but reads only the width, height, size and format of an image,
    /// without reading data.
    pub fn ping_image_blob<T: AsRef<[u8]>>(&self, data: T) -> Result<()> {
        let int_slice = data.as_ref();
        let size = int_slice.len();
        self.result_from_boolean(unsafe {
            bindings::MagickPingImageBlob(self.wand, int_slice.as_ptr() as *const c_void, size)
        })
    }

    /// Composes all the image layers from the current given image onward to produce a single image
    /// of the merged layers.
    ///
    /// The inital canvas's size depends on the given LayerMethod, and is initialized using the
    /// first images background color. The images are then composited onto that image in sequence
    /// using the given composition that has been assigned to each individual image.
    ///
    /// * `method`: the method of selecting the size of the initial canvas.
    ///   MergeLayer: Merge all layers onto a canvas just large enough to hold all the actual
    ///   images. The virtual canvas of the first image is preserved but otherwise ignored.
    ///
    ///     FlattenLayer: Use the virtual canvas size of first image. Images which fall outside
    ///   this canvas is clipped. This can be used to 'fill out' a given virtual canvas.
    ///
    ///     MosaicLayer: Start with the virtual canvas of the first image, enlarging left and right
    ///   edges to contain all images. Images with negative offsets will be clipped.
    pub fn merge_image_layers(&self, method: LayerMethod) -> Result<MagickWand> {
        let wand_ptr = unsafe { bindings::MagickMergeImageLayers(self.wand, method) };
        Self::result_from_ptr_with_error_message(
            wand_ptr,
            MagickWand::from_ptr,
            "failed to merge image layers",
        )
    }

    /// Returns the number of images associated with a magick wand.
    pub fn get_number_images(&self) -> usize {
        unsafe { bindings::MagickGetNumberImages(self.wand) }
    }

    /// Compare two images and return tuple `(distortion, diffImage)`
    /// `diffImage` is `None` if `distortion == 0`
    pub fn compare_images(
        &self,
        reference: &MagickWand,
        metric: MetricType,
    ) -> (f64, Option<MagickWand>) {
        let mut distortion: f64 = 0.0;
        let wand_ptr = unsafe {
            bindings::MagickCompareImages(self.wand, reference.wand, metric, &mut distortion)
        };

        let wand = Self::result_from_ptr_with_error_message(wand_ptr, MagickWand::from_ptr, "").ok();
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
        self.result_from_boolean(unsafe {
            bindings::MagickCompositeImage(
                self.wand,
                reference.wand,
                composition_operator,
                MagickBooleanType::from(clip_to_self),
                x,
                y,
            )
        })
    }

    /// Compose another image onto self with gravity using composition_operator
    pub fn compose_images_gravity(
        &self,
        reference: &MagickWand,
        composition_operator: CompositeOperator,
        gravity_type: GravityType,
    ) -> Result<()> {
        self.result_from_boolean(unsafe {
            bindings::MagickCompositeImageGravity(
                self.wand,
                reference.wand,
                composition_operator,
                gravity_type,
            )
        })
    }

    /// Rebuilds image sequence with each frame size the same as first frame, and composites each frame atop of previous.
    /// Only affects GIF, and other formats with multiple pages/layers.
    pub fn coalesce(&mut self) -> Result<MagickWand> {
        let wand_ptr = unsafe { bindings::MagickCoalesceImages(self.wand) };
        Self::result_from_ptr_with_error_message(
            wand_ptr,
            MagickWand::from_ptr,
            "failed to coalesce images",
        )
    }

    /// Replaces colors in the image from a color lookup table.
    pub fn clut_image(&self, clut_wand: &MagickWand, method: PixelInterpolateMethod) -> Result<()> {
        self.result_from_boolean(unsafe {
            bindings::MagickClutImage(self.wand, clut_wand.wand, method)
        })
    }

    pub fn hald_clut_image(&self, clut_wand: &MagickWand) -> Result<()> {
        self.result_from_boolean(unsafe {
            bindings::MagickHaldClutImage(self.wand, clut_wand.wand)
        })
    }

    pub fn fx(&mut self, expression: &str) -> Result<MagickWand> {
        let c_expression =
            CString::new(expression).map_err(|_| "expression string contains null byte")?;
        let wand_ptr = unsafe { bindings::MagickFxImage(self.wand, c_expression.as_ptr()) };
        Self::result_from_ptr_with_error_message(
            wand_ptr,
            MagickWand::from_ptr,
            "failed to fx the image",
        )
    }

    pub fn set_size(&self, columns: usize, rows: usize) -> Result<()> {
        self.result_from_boolean(unsafe { bindings::MagickSetSize(self.wand, columns, rows) })
    }

    /// Define two 'quantum_range' functions because the bindings::QuantumRange symbol
    /// is not available if hdri is disabled in the compiled ImageMagick libs
    #[cfg(not(feature = "disable-hdri"))]
    fn quantum_range(&self) -> Result<f64> {
        Ok(bindings::QuantumRange)
    }

    /// with disable-hdri enabled we define our own quantum_range
    /// values lifted directly from magick-type.h
    #[cfg(feature = "disable-hdri")]
    fn quantum_range(&self) -> Result<f64> {
        match bindings::MAGICKCORE_QUANTUM_DEPTH {
            8 => Ok(255.0f64),
            16 => Ok(65535.0f64),
            32 => Ok(4294967295.0f64),
            64 => Ok(18446744073709551615.0f64),
            _ => Err(MagickError(
                "Quantum depth must be one of 8, 16, 32 or 64".to_string(),
            )),
        }
    }

    /// Level an image. Black and white points are multiplied with QuantumRange to
    /// decrease dependencies on the end user.
    pub fn level_image(&self, black_point: f64, gamma: f64, white_point: f64) -> Result<()> {
        let quantum_range = self.quantum_range()?;

        self.result_from_boolean(unsafe {
            bindings::MagickLevelImage(
                self.wand,
                black_point * quantum_range,
                gamma,
                white_point * quantum_range,
            )
        })
    }

    /// Applies the reversed [level_image](Self::level_image). It compresses the full range of color values, so
    /// that they lie between the given black and white points. Gamma is applied before the values
    /// are mapped. It can be used to de-contrast a greyscale image to the exact levels specified.
    pub fn levelize_image(&self, black_point: f64, gamma: f64, white_point: f64) -> Result<()> {
        let quantum_range = self.quantum_range()?;

        self.result_from_boolean(unsafe {
            bindings::MagickLevelizeImage(
                self.wand,
                black_point * quantum_range,
                gamma,
                white_point * quantum_range,
            )
        })
    }

    /// MagickNormalizeImage enhances the contrast of a color image by adjusting the pixels color
    /// to span the entire range of colors available
    pub fn normalize_image(&self) -> Result<()> {
        self.result_from_boolean(unsafe { bindings::MagickNormalizeImage(self.wand) })
    }

    /// MagickOrderedDitherImage performs an ordered dither based on a number of pre-defined
    /// dithering threshold maps, but over multiple intensity levels, which can be different for
    /// different channels, according to the input arguments.
    pub fn ordered_dither_image(&self, threshold_map: &str) -> Result<()> {
        let c_threshold_map =
            CString::new(threshold_map).map_err(|_| "threshold_map string contains null byte")?;

        self.result_from_boolean(unsafe {
            bindings::MagickOrderedDitherImage(self.wand, c_threshold_map.as_ptr())
        })
    }

    /// Apply sigmoidal contrast to the image
    ///
    /// Adjusts the contrast of an image with a non-linear sigmoidal contrast algorithm. Increase
    /// the contrast of the image using a sigmoidal transfer function without saturating highlights
    /// or shadows. Contrast indicates how much to increase the contrast (0 is none; 3 is typical;
    /// 20 is pushing it); mid-point indicates where midtones fall in the resultant image (0.0 is
    /// white; 0.5 is middle-gray; 1.0 is black). Set sharpen to `true` to increase the image
    /// contrast otherwise the contrast is reduced.
    ///
    /// * `sharpen`: increase or decrease image contrast
    /// * `strength`: strength of the contrast, the larger the number the more 'threshold-like' it becomes.
    /// * `midpoint`: midpoint of the function as a number in range [0, 1]
    pub fn sigmoidal_contrast_image(
        &self,
        sharpen: bool,
        strength: f64,
        midpoint: f64,
    ) -> Result<()> {
        let quantum_range = self.quantum_range()?;

        self.result_from_boolean(unsafe {
            bindings::MagickSigmoidalContrastImage(
                self.wand,
                sharpen.into(),
                strength,
                midpoint * quantum_range,
            )
        })
    }

    /// Extend the image as defined by the geometry, gravity, and wand background color. Set the
    /// (x,y) offset of the geometry to move the original wand relative to the extended wand.
    pub fn extend_image(&self, width: usize, height: usize, x: isize, y: isize) -> Result<()> {
        self.result_from_boolean(unsafe {
            bindings::MagickExtentImage(self.wand, width, height, x, y)
        })
    }

    pub fn profile_image<'a, T: Into<Option<&'a [u8]>>>(
        &self,
        name: &str,
        profile: T,
    ) -> Result<()> {
        let c_name = CString::new(name).map_err(|_| "name string contains null byte")?;
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
        self.result_from_boolean(result)
    }

    pub fn strip_image(&self) -> Result<()> {
        self.result_from_boolean(unsafe { bindings::MagickStripImage(self.wand) })
    }

    pub fn flip_image(&self) -> Result<()> {
        self.result_from_boolean(unsafe { bindings::MagickFlipImage(self.wand) })
    }

    pub fn negate_image(&self) -> Result<()> {
        self.result_from_boolean(unsafe { bindings::MagickNegateImage(self.wand, MagickTrue) })
    }

    pub fn flop_image(&self) -> Result<()> {
        self.result_from_boolean(unsafe { bindings::MagickFlopImage(self.wand) })
    }

    pub fn blur_image(&self, radius: f64, sigma: f64) -> Result<()> {
        self.result_from_boolean(unsafe { bindings::MagickBlurImage(self.wand, radius, sigma) })
    }

    pub fn gaussian_blur_image(&self, radius: f64, sigma: f64) -> Result<()> {
        self.result_from_boolean(unsafe {
            bindings::MagickGaussianBlurImage(self.wand, radius, sigma)
        })
    }

    /// Replace each pixel with corresponding statistic from the neighborhood of the specified width and height.
    ///
    /// * `statistic_type`: the statistic type (e.g. `StatisticType::Median`, `StatisticType::Mode`, etc.).
    /// * `width`: the width of the pixel neighborhood.
    /// * `height`: the height of the pixel neighborhood.
    pub fn statistic_image(
        &self,
        statistic_type: StatisticType,
        width: usize,
        height: usize,
    ) -> Result<()> {
        self.result_from_boolean(unsafe {
            bindings::MagickStatisticImage(self.wand, statistic_type, width, height)
        })
    }

    /// Calculate median for each pixel's neighborhood.
    ///
    /// See [statistic_image](Self::statistic_image)
    pub fn median_blur_image(&self, width: usize, height: usize) -> Result<()> {
        self.statistic_image(StatisticType::Median, width, height)
    }

    /// Adaptively resize the currently selected image.
    pub fn adaptive_resize_image(&self, width: usize, height: usize) -> Result<()> {
        self.result_from_boolean(unsafe {
            bindings::MagickAdaptiveResizeImage(self.wand, width, height)
        })
    }

    /// Rotate the currently selected image by the given number of degrees,
    /// filling any empty space with the background color of a given PixelWand
    pub fn rotate_image(&self, background: &PixelWand, degrees: f64) -> Result<()> {
        self.result_from_boolean(unsafe {
            bindings::MagickRotateImage(self.wand, background.as_ptr(), degrees)
        })
    }

    /// Trim the image removing the backround color from the edges.
    pub fn trim_image(&self, fuzz: f64) -> Result<()> {
        self.result_from_boolean(unsafe { bindings::MagickTrimImage(self.wand, fuzz) })
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
            // Note: The C MagickGetImagePage function always returns true
            // and exits on error, so we don't check the return value here.
            bindings::MagickGetImagePage(self.wand, &mut width, &mut height, &mut x, &mut y);
        }
        (width, height, x, y)
    }

    /// Reset the Wand page canvas and position.
    pub fn reset_image_page(&self, page_geometry: &str) -> Result<()> {
        let c_page_geometry =
            CString::new(page_geometry).map_err(|_| "page_geometry contains null byte")?;
        self.result_from_boolean(unsafe {
            bindings::MagickResetImagePage(self.wand, c_page_geometry.as_ptr())
        })
    }

    /// Returns a value associated with the specified artifact.
    ///
    /// * `artifact`: the artifact.
    pub fn get_image_artifact(&self, artifact: &str) -> Result<String> {
        let c_artifact =
            CString::new(artifact).map_err(|_| "artifact string contains null byte")?;

        let c_value = unsafe { bindings::MagickGetImageArtifact(self.wand, c_artifact.as_ptr()) };
        Self::result_from_ptr_with_error_message(
            c_value,
            Self::c_char_into_string,
            format!("missing artifact: {artifact}"),
        )
    }

    pub fn get_image_artifacts(&self, pattern: &str) -> Result<Vec<String>> {
        let c_pattern = CString::new(pattern)
            .map_err(|_| MagickError("artifact string contains null byte".to_string()))?;
        let mut num_of_artifacts: size_t = 0;

        let c_values = unsafe {
            bindings::MagickGetImageArtifacts(self.wand, c_pattern.as_ptr(), &mut num_of_artifacts)
        };

        Self::result_from_ptr_with_error_message(
            c_values,
            |c_values| Self::c_char_to_string_vec(c_values, num_of_artifacts),
            "image has no artifacts",
        )
    }

    /// Sets a key-value pair in the image artifact namespace. Artifacts differ from properties.
    /// Properties are public and are generally exported to an external image format if the format
    /// supports it. Artifacts are private and are utilized by the internal ImageMagick API to
    /// modify the behavior of certain algorithms.
    ///
    /// * `artifact`: the artifact.
    /// * `value`: the value.
    ///
    /// # Example
    ///
    /// This example shows how you can blend an image with its blurred copy with 50% opacity by
    /// setting "compose:args" to "50". This is equivalent to having `-define compose:args=50` when
    /// using imagemagick cli.
    ///
    /// ```
    /// use magick_rust::{MagickWand, PixelWand, CompositeOperator};
    ///
    /// fn main() -> Result<(), magick_rust::MagickError> {
    ///     let mut wand1 = MagickWand::new();
    ///     wand1.new_image(4, 4, &PixelWand::new())?; // Replace with `read_image` to open your image file
    ///     let wand2 = wand1.clone();
    ///
    ///     wand1.median_blur_image(10, 10)?;
    ///
    ///     wand1.set_image_artifact("compose:args", "50")?;
    ///     wand1.compose_images(&wand2, CompositeOperator::Blend, false, 0, 0)?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn set_image_artifact(&mut self, artifact: &str, value: &str) -> Result<()> {
        let c_artifact =
            CString::new(artifact).map_err(|_| "artifact string contains null byte")?;
        let c_value = CString::new(value).map_err(|_| "value string contains null byte")?;

        self.result_from_boolean(unsafe {
            bindings::MagickSetImageArtifact(self.wand, c_artifact.as_ptr(), c_value.as_ptr())
        })
    }

    /// Deletes a wand artifact.
    ///
    /// * `artifact`: the artifact.
    pub fn delete_image_artifact(&mut self, artifact: &str) -> Result<()> {
        let c_artifact =
            CString::new(artifact).map_err(|_| "artifact string contains null byte")?;

        Self::result_from_boolean_with_error_message(
            unsafe { bindings::MagickDeleteImageArtifact(self.wand, c_artifact.as_ptr()) },
            format!("missing artifact: {artifact}"),
        )
    }

    /// Retrieve the named image property value.
    pub fn get_image_property(&self, name: &str) -> Result<String> {
        let c_name = CString::new(name).map_err(|_| "name string contains null byte")?;
        let c_value = unsafe { bindings::MagickGetImageProperty(self.wand, c_name.as_ptr()) };

        Self::result_from_ptr_with_error_message(
            c_value,
            Self::c_char_into_string,
            format!("missing property: {name}"),
        )
    }

    pub fn get_image_properties(&self, pattern: &str) -> Result<Vec<String>> {
        let c_pattern = CString::new(pattern)
            .map_err(|_| MagickError("artifact string contains null byte".to_string()))?;
        let mut num_of_artifacts: size_t = 0;

        let c_values = unsafe {
            bindings::MagickGetImageProperties(self.wand, c_pattern.as_ptr(), &mut num_of_artifacts)
        };

        self.result_from_ptr(c_values, |c_values| Self::c_char_to_string_vec(c_values, num_of_artifacts))
    }

    /// Set the named image property.
    pub fn set_image_property(&self, name: &str, value: &str) -> Result<()> {
        let c_name = CString::new(name).map_err(|_| "name string contains null byte")?;
        let c_value = CString::new(value).map_err(|_| "value string contains null byte")?;
        self.result_from_boolean(unsafe {
            bindings::MagickSetImageProperty(self.wand, c_name.as_ptr(), c_value.as_ptr())
        })
    }

    /// Returns a `PixelWand` instance for the pixel specified by x and y offests.
    pub fn get_image_pixel_color(&self, x: isize, y: isize) -> Option<PixelWand> {
        let pw = PixelWand::new();

        let result = unsafe { bindings::MagickGetImagePixelColor(self.wand, x, y, pw.as_ptr()) };
        self.result_from_boolean(result).map(|_| pw).ok()
    }

    /// Sets the image sampling factors.
    ///
    /// samplingFactors: An array of floats representing the sampling factor for each color component (in RGB order).
    pub fn set_sampling_factors(&self, samplingFactors: &[f64]) -> Result<()> {
        self.result_from_boolean( unsafe {
            bindings::MagickSetSamplingFactors(
                self.wand,
                samplingFactors.len(),
                &samplingFactors[0],
            )
        })
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
                        .map(|wand_ptr| PixelWand::from_ptr(*wand_ptr))
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
        self.result_from_boolean(unsafe { bindings::MagickSharpenImage(self.wand, radius, sigma) })
    }

    /// Set the background color.
    pub fn set_background_color(&self, pixel_wand: &PixelWand) -> Result<()> {
        self.result_from_boolean(unsafe {
            bindings::MagickSetBackgroundColor(self.wand, pixel_wand.as_ptr())
        })
    }

    /// Set the image background color.
    pub fn set_image_background_color(&self, pixel_wand: &PixelWand) -> Result<()> {
        self.result_from_boolean(unsafe {
            bindings::MagickSetImageBackgroundColor(self.wand, pixel_wand.as_ptr())
        })
    }

    /// Returns the image resolution as a pair (horizontal resolution, vertical resolution)
    pub fn get_image_resolution(&self) -> Result<(f64, f64)> {
        let mut x_resolution = 0f64;
        let mut y_resolution = 0f64;
        self.result_from_boolean(unsafe {
            bindings::MagickGetImageResolution(self.wand, &mut x_resolution, &mut y_resolution)
        })
        .map(|_| (x_resolution, y_resolution))
    }

    /// Sets the image resolution
    pub fn set_image_resolution(&self, x_resolution: f64, y_resolution: f64) -> Result<()> {
        self.result_from_boolean(unsafe {
            bindings::MagickSetImageResolution(self.wand, x_resolution, y_resolution)
        })
    }

    /// Sets the wand resolution
    pub fn set_resolution(&self, x_resolution: f64, y_resolution: f64) -> Result<()> {
        self.result_from_boolean(unsafe {
            bindings::MagickSetResolution(self.wand, x_resolution, y_resolution)
        })
    }

    /// Returns the image resolution as a pair (horizontal resolution, vertical resolution)
    pub fn sepia_tone_image(&self, threshold: f64) -> Result<()> {
        self.result_from_boolean(unsafe {
            bindings::MagickSepiaToneImage(self.wand, threshold * self.quantum_range()?)
        })
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
        let c_map = CString::new(map).ok()?;
        let capacity = width * height * map.len();
        let mut pixels = vec![0; capacity];

        unsafe {
            if bindings::MagickExportImagePixels(
                self.wand,
                x,
                y,
                width,
                height,
                c_map.as_ptr(),
                bindings::StorageType::CharPixel,
                pixels.as_mut_ptr() as *mut c_void,
            ) == MagickTrue
            {
                Some(pixels)
            } else {
                None
            }
        }
    }

    pub fn export_image_pixels_double(
        &self,
        x: isize,
        y: isize,
        width: usize,
        height: usize,
        map: &str,
    ) -> Option<Vec<f64>> {
        let c_map = CString::new(map).expect("map contains null byte");
        let capacity = width * height * map.len();
        let mut pixels = Vec::with_capacity(capacity);
        pixels.resize(capacity, 0.0);

        unsafe {
            if bindings::MagickExportImagePixels(
                self.wand,
                x,
                y,
                width,
                height,
                c_map.as_ptr(),
                bindings::StorageType::DoublePixel,
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
    pub fn resize_image(&self, width: usize, height: usize, filter: FilterType) -> Result<()> {
        self.result_from_boolean(unsafe {
            bindings::MagickResizeImage(self.wand, width, height, filter)
        })
    }

    /// Resize image by specifying the new size in percent of last size.
    ///
    /// Effectively resizes image to (current width * `width_scale`, current height *
    /// `height_scale`)
    pub fn scale_image(
        &self,
        width_scale: f64,
        height_scale: f64,
        filter: FilterType,
    ) -> Result<()> {
        if width_scale < 0.0 {
            return Err(MagickError("negative width scale given".to_string()));
        }
        if height_scale < 0.0 {
            return Err(MagickError("negative height scale given".to_string()));
        }

        let width = self.get_image_width();
        let height = self.get_image_height();

        let width = ((width as f64) * width_scale) as usize;
        let height = ((height as f64) * height_scale) as usize;

        self.resize_image(width, height, filter)
    }

    /// Resize the image to the specified width and height, using the
    /// 'thumbnail' optimizations which remove a lot of image meta-data with the goal
    /// of producing small low cost images suited for display on the web.
    pub fn thumbnail_image(&self, width: usize, height: usize) -> Result<()> {
        self.result_from_boolean(unsafe {
            bindings::MagickThumbnailImage(self.wand, width, height)
        })
    }

    /// Extract a region of the image. The width and height is used as the size
    /// of the region. X and Y is the offset.
    pub fn crop_image(&self, width: usize, height: usize, x: isize, y: isize) -> Result<()> {
        self.result_from_boolean(unsafe {
            bindings::MagickCropImage(self.wand, width, height, x, y)
        })
    }

    /// Sample the image to the target resolution
    ///
    /// This is incredibly fast, as it does 1-1 pixel mapping for downscales, and box filtering for
    /// upscales
    pub fn sample_image(&self, width: usize, height: usize) -> Result<()> {
        self.result_from_boolean(unsafe { bindings::MagickSampleImage(self.wand, width, height) })
    }

    /// Resample the image to the specified horizontal and vertical resolution, using the
    /// specified filter type.
    pub fn resample_image(
        &self,
        x_resolution: f64,
        y_resolution: f64,
        filter: FilterType,
    ) -> Result<()> {
        self.result_from_boolean(unsafe {
            bindings::MagickResampleImage(self.wand, x_resolution, y_resolution, filter)
        })
    }

    /// Rescale the image using seam carving algorithm
    pub fn liquid_rescale_image(
        &self,
        width: usize,
        height: usize,
        delta_x: f64,
        rigidity: f64,
    ) -> Result<()> {
        self.result_from_boolean(unsafe {
            bindings::MagickLiquidRescaleImage(self.wand, width, height, delta_x, rigidity)
        })
    }

    /// Implodes the image towards the center by the specified percentage
    pub fn implode(&self, amount: f64, method: PixelInterpolateMethod) -> Result<()> {
        self.result_from_boolean(unsafe { bindings::MagickImplodeImage(self.wand, amount, method) })
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
                bindings::MagickResizeImage(self.wand, new_width, new_height, FilterType::Lanczos);
            }
        }
    }

    /// Detect if the loaded image is not in top-left orientation, and
    /// hence should be "auto" oriented so it is suitable for viewing.
    pub fn requires_orientation(&self) -> bool {
        self.get_image_orientation() != OrientationType::TopLeft
    }

    /// Automatically adjusts the loaded image so that its orientation is
    /// suitable for viewing (i.e. top-left orientation).
    ///
    /// Returns `true` if successful or `false` if an error occurred.
    pub fn auto_orient(&self) -> bool {
        unsafe { bindings::MagickAutoOrientImage(self.wand) == MagickTrue }
    }

    /// Write the current image to the provided path.
    pub fn write_image(&self, path: &str) -> Result<()> {
        let c_name = CString::new(path).map_err(|_| "name string contains null byte")?;
        self.result_from_boolean(unsafe { bindings::MagickWriteImage(self.wand, c_name.as_ptr()) })
    }

    /// Write the image in the desired format to a new blob.
    ///
    /// The `format` argument may be any ImageMagick supported image
    /// format (e.g. GIF, JPEG, PNG, etc).
    pub fn write_image_blob(&self, format: &str) -> Result<Vec<u8>> {
        let c_format = CString::new(format).map_err(|_| "format string contains null byte")?;
        let mut length: size_t = 0;
        let blob = unsafe {
            bindings::MagickResetIterator(self.wand);
            bindings::MagickSetImageFormat(self.wand, c_format.as_ptr());
            bindings::MagickGetImageBlob(self.wand, &mut length)
        };

        self.result_from_ptr(blob, |blob| Self::c_array_into_vec(blob, length))
    }

    /// Write the images in the desired format to a new blob.
    ///
    /// The `format` argument may be any ImageMagick supported image
    /// format (e.g. GIF, JPEG, PNG, etc).
    pub fn write_images_blob(&self, format: &str) -> Result<Vec<u8>> {
        let c_format = CString::new(format).map_err(|_| "format string contains null byte")?;
        let mut length: size_t = 0;
        let blob = unsafe {
            bindings::MagickSetIteratorIndex(self.wand, 0);
            bindings::MagickSetImageFormat(self.wand, c_format.as_ptr());
            bindings::MagickGetImagesBlob(self.wand, &mut length)
        };

        Ok(Self::c_array_into_vec(blob, length))
    }

    /// Return false if the image alpha channel is not activated.
    /// That is, the image is RGB rather than RGBA or CMYK rather than CMYKA
    pub fn get_image_alpha_channel(&self) -> bool {
        let res = unsafe { bindings::MagickGetImageAlphaChannel(self.wand) };
        res == MagickTrue
    }

    /// Renders the drawing wand on the current image
    pub fn draw_image(&mut self, drawing_wand: &DrawingWand) -> Result<()> {
        self.result_from_boolean(unsafe { bindings::MagickDrawImage(self.wand, drawing_wand.as_ptr()) })
    }

    /// Removes skew from the image. Skew is an artifact that
    /// occurs in scanned images because of the camera being misaligned,
    /// imperfections in the scanning or surface, or simply because the paper was
    /// not placed completely flat when scanned
    pub fn deskew_image(&mut self, threshold: f64) -> Result<()> {
        self.result_from_boolean(unsafe { bindings::MagickDeskewImage(self.wand, threshold) })
    }

    /// Sets image clip mask.
    ///
    /// * `pixel_mask`: type of mask, Read or Write.
    /// * `clip_mask`: the clip_mask wand.
    pub fn set_image_mask(&mut self, pixel_mask: PixelMask, clip_mask: &MagickWand) -> Result<()> {
        self.result_from_boolean(unsafe {
            bindings::MagickSetImageMask(self.wand, pixel_mask, clip_mask.wand)
        })
    }

    /// Set image channel mask
    pub fn set_image_channel_mask(&mut self, option: ChannelType) -> ChannelType {
        unsafe { bindings::MagickSetImageChannelMask(self.wand, option) }
    }

    /// Apply an arithmetic, relational, or logical
    /// expression to an image.  Use these operators to lighten or darken an image,
    /// to increase or decrease contrast in an image, or to produce the "negative"
    /// of an image.
    pub fn evaluate_image(&mut self, op: MagickEvaluateOperator, val: f64) -> Result<()> {
        self.result_from_boolean(unsafe { bindings::MagickEvaluateImage(self.wand, op, val) })
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
        self.result_from_boolean(unsafe {
            bindings::MagickBorderImage(self.wand, pixel_wand.as_ptr(), width, height, compose)
        })
    }

    /// Simulate an image shadow
    pub fn shadow_image(&self, alpha: f64, sigma: f64, x: isize, y: isize) -> Result<()> {
        self.result_from_boolean(unsafe {
            bindings::MagickShadowImage(self.wand, alpha, sigma, x, y)
        })
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
        let pixel_map = CString::new(map).map_err(|_| "map string contains null byte")?;
        self.result_from_boolean(unsafe {
            bindings::MagickImportImagePixels(
                self.wand,
                x,
                y,
                columns,
                rows,
                pixel_map.as_ptr(),
                bindings::StorageType::CharPixel,
                pixels.as_ptr() as *const libc::c_void,
            )
        })
    }

    pub fn import_image_pixels_double(
        &mut self,
        x: isize,
        y: isize,
        columns: usize,
        rows: usize,
        pixels: &[f64],
        map: &str,
    ) -> Result<()> {
        let pixel_map = CString::new(map).expect("map string contains null byte");
        Self::result_from_boolean_with_error_message( unsafe {
            bindings::MagickImportImagePixels(
                self.wand,
                x,
                y,
                columns,
                rows,
                pixel_map.as_ptr(),
                bindings::StorageType::DoublePixel,
                pixels.as_ptr() as *const c_void,
            )
        }, "unable to import pixels")
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
    pub fn auto_threshold(&self, method: AutoThresholdMethod) -> Result<()> {
        self.result_from_boolean(unsafe { bindings::MagickAutoThresholdImage(self.wand, method) })
    }

    /// Set the image colorspace, transforming (unlike `set_image_colorspace`) image data in
    /// the process.
    pub fn transform_image_colorspace(&self, colorspace: ColorspaceType) -> Result<()> {
        self.result_from_boolean(unsafe {
            bindings::MagickTransformImageColorspace(self.wand, colorspace)
        })
    }

    /// Reduce the number of colors in the image.
    pub fn quantize_image(
        &self,
        number_of_colors: usize,
        colorspace: ColorspaceType,
        tree_depth: usize,
        dither_method: DitherMethod,
        measure_error: bool,
    ) -> Result<()> {
        self.result_from_boolean(unsafe {
            bindings::MagickQuantizeImage(
                self.wand,
                number_of_colors,
                colorspace,
                tree_depth,
                dither_method,
                measure_error.into(),
            )
        })
    }

    /// Reduce the number of colors in the images.
    pub fn quantize_images(
        &self,
        number_of_colors: usize,
        colorspace: ColorspaceType,
        tree_depth: usize,
        dither_method: DitherMethod,
        measure_error: bool,
    ) -> Result<()> {
        self.result_from_boolean(unsafe {
            bindings::MagickQuantizeImages(
                self.wand,
                number_of_colors,
                colorspace,
                tree_depth,
                dither_method,
                measure_error.into(),
            )
        })
    }

    /// Applies an arithmetic, relational, or logical expression to an image. Use these operators
    /// to lighten or darken an image, to increase or decrease contrast in an image, or to produce
    /// the "negative" of an image.
    ///
    /// * `function`: the image function.
    /// * `args`: the function arguments.
    ///
    /// # Example
    ///
    /// This example show how you can apply smoothstep function (a polynomial `-2x^3 + 3x^2`) to
    /// every image pixel.
    ///
    /// ```
    /// use magick_rust::{MagickWand, PixelWand, MagickFunction};
    ///
    /// fn main() -> Result<(), magick_rust::MagickError> {
    ///     let mut wand1 = MagickWand::new();
    ///     wand1.new_image(4, 4, &PixelWand::new())?; // Replace with `read_image` to open your image file
    ///
    ///     // Apply smoothstep polynomial
    ///     wand1.function_image(MagickFunction::Polynomial, &[-2.0, 3.0, 0.0, 0.0])?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn function_image(&self, function: MagickFunction, args: &[f64]) -> Result<()> {
        let num_of_args: size_t = args.len();
        self.result_from_boolean(unsafe {
            bindings::MagickFunctionImage(self.wand, function, num_of_args, args.as_ptr())
        })
    }

    /// Returns an image where each pixel is the sum of the pixels in the image sequence after
    /// applying its corresponding terms (coefficient and degree pairs).
    ///
    /// * `terms`: the list of polynomial coefficients and degree pairs and a constant.
    pub fn polynomial_image(&self, terms: &[f64]) -> Result<()> {
        if terms.len() & 1 != 1 {
            return Err(MagickError("no constant coefficient given".to_string()));
        }

        let num_of_terms: size_t = terms.len() >> 1;

        self.result_from_boolean(unsafe {
            bindings::MagickPolynomialImage(self.wand, num_of_terms, terms.as_ptr())
        })
    }

    /// Applies a custom convolution kernel to the image.
    ///
    /// * `kernel_info`: An array of doubles representing the convolution kernel.
    pub fn convolve_image(&self, kernel_info: &KernelInfo) -> Result<()> {
        self.result_from_boolean(unsafe {
            bindings::MagickConvolveImage(self.wand, kernel_info.get_ptr())
        })
    }

    /// Applies a user supplied kernel to the image according to the given morphology method.
    ///
    /// * `morphology_method`: the morphology method to be applied.
    /// * `iterations`: apply the operation this many times (or no change). A value of -1 means loop until no change found. How this is applied may depend on the morphology method. Typically this is a value of 1.
    /// * `kernel_info`: An array of doubles representing the morphology kernel.
    pub fn morphology_image(
        &self,
        morphology_method: MorphologyMethod,
        iterations: isize,
        kernel_info: &KernelInfo,
    ) -> Result<()> {
        self.result_from_boolean(unsafe {
            bindings::MagickMorphologyImage(
                self.wand,
                morphology_method,
                iterations,
                kernel_info.get_ptr(),
            )
        })
    }

    /// Apply color transformation to an image. The method permits saturation changes, hue rotation,
    /// luminance to alpha, and various other effects. Although variable-sized transformation
    /// matrices can be used, typically one uses a 5x5 matrix for an RGBA image and a 6x6 for CMYKA
    /// (or RGBA with offsets). The matrix is similar to those used by Adobe Flash except offsets
    /// are in column 6 rather than 5 (in support of CMYKA images) and offsets are normalized
    /// (divide Flash offset by 255).
    ///
    /// * `color_matrix`: the color matrix.
    pub fn color_matrix_image(&self, color_matrix: &KernelInfo) -> Result<()> {
        self.result_from_boolean(unsafe {
            bindings::MagickColorMatrixImage(self.wand, color_matrix.get_ptr())
        })
    }

    /// Applies a channel expression to the specified image. The expression
    /// consists of one or more channels, either mnemonic or numeric (e.g. red, 1), separated by
    /// actions as follows:
    ///
    /// <=> exchange two channels (e.g. red<=>blue) => transfer a channel to another (e.g.
    /// red=>green) , separate channel operations (e.g. red, green) | read channels from next input
    /// image (e.g. red | green) ; write channels to next output image (e.g. red; green; blue) A
    /// channel without a operation symbol implies extract. For example, to create 3 grayscale
    /// images from the red, green, and blue channels of an image, use:
    ///
    /// * `expression`: the expression.
    pub fn channel_fx_image(&self, expression: &str) -> Result<MagickWand> {
        let c_expression =
            CString::new(expression).map_err(|_| "artifact string contains null byte")?;

        let wand_ptr = unsafe { bindings::MagickChannelFxImage(self.wand, c_expression.as_ptr()) };
        self.result_from_ptr(wand_ptr, MagickWand::from_ptr)
    }

    /// Combines one or more images into a single image. The grayscale value of the pixels of each
    /// image in the sequence is assigned in order to the specified channels of the combined image.
    /// The typical ordering would be image 1 => Red, 2 => Green, 3 => Blue, etc.
    ///
    /// * `colorspace`: the colorspace.
    pub fn combine_images(&self, colorspace: ColorspaceType) -> Result<MagickWand> {
        let wand_ptr = unsafe { bindings::MagickCombineImages(self.wand, colorspace) };
        self.result_from_ptr(wand_ptr, MagickWand::from_ptr)
    }

    /// Returns the current image from the magick wand.
    pub fn get_image(&self) -> Result<Image<'_>> {
        self.result_from_ptr(
            unsafe { bindings::GetImageFromMagickWand(self.wand) },
            Image::new,
        )
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
        get_image_orientation,           set_image_orientation,           MagickGetImageOrientation,         MagickSetImageOrientation,        OrientationType
        get_image_rendering_intent,      set_image_rendering_intent,      MagickGetImageRenderingIntent,     MagickSetImageRenderingIntent,    RenderingIntent
        get_image_scene,                 set_image_scene,                 MagickGetImageScene,               MagickSetImageScene,              usize
        get_image_type,                  set_image_type,                  MagickGetImageType,                MagickSetImageType,               ImageType
        get_image_units,                 set_image_units,                 MagickGetImageUnits,               MagickSetImageUnits,              ResolutionType
        get_interlace_scheme,            set_interlace_scheme,            MagickGetInterlaceScheme,          MagickSetInterlaceScheme,         InterlaceType
        get_interpolate_method,          set_interpolate_method,          MagickGetInterpolateMethod,        MagickSetInterpolateMethod,       PixelInterpolateMethod
        get_iterator_index,              set_iterator_index,              MagickGetIteratorIndex,            MagickSetIteratorIndex,           isize
        get_orientation,                 set_orientation,                 MagickGetOrientation,              MagickSetOrientation,             OrientationType
        get_pointsize,                   set_pointsize,                   MagickGetPointsize,                MagickSetPointsize,               f64
        get_type,                        set_type,                        MagickGetType,                     MagickSetType,                    ImageType
    );

    fn result_from_boolean(&self, no_error: MagickBooleanType) -> Result<()> {
        if no_error == MagickTrue {
            Ok(())
        } else {
            Err(MagickError(self.get_exception()?.0))
        }
    }

    fn result_from_boolean_with_error_message(
        no_error: MagickBooleanType,
        message: impl Into<String>,
    ) -> Result<()> {
        if no_error == MagickTrue {
            Ok(())
        } else {
            Err(MagickError(message.into()))
        }
    }

    fn result_from_ptr<P, T>(&self, ptr: *mut P, new: impl FnOnce(*mut P) -> T) -> Result<T> {
        if ptr.is_null() {
            Err(MagickError(self.get_exception()?.0))
        } else {
            Ok(new(ptr))
        }
    }

    fn result_from_ptr_with_error_message<P, T>(
        ptr: *mut P,
        new: impl FnOnce(*mut P) -> T,
        message: impl Into<String>,
    ) -> Result<T> {
        if ptr.is_null() {
            Err(MagickError(message.into()))
        } else {
            Ok(new(ptr))
        }
    }

    fn c_char_to_string_vec(c_values: *mut *mut c_char, num_of_artifacts: usize) -> Vec<String> {
        let mut values: Vec<String> = Vec::with_capacity(num_of_artifacts);
        for i in 0..num_of_artifacts {
            // convert (and copy) the C string to a Rust string
            let cstr = unsafe { CStr::from_ptr(*c_values.add(i)) };
            values.push(cstr.to_string_lossy().into_owned());
        }

        unsafe {
            bindings::MagickRelinquishMemory(c_values as *mut c_void);
        }

        values
    }

    fn c_char_into_string(c_value: *mut c_char) -> String {
        let value = unsafe { CStr::from_ptr(c_value) }
            .to_string_lossy()
            .into_owned();

        unsafe {
            bindings::MagickRelinquishMemory(c_value as *mut c_void);
        }

        value
    }

    fn c_array_into_vec(blob: *mut c_uchar, length: usize) -> Vec<u8> {
        let mut bytes = vec![0; length];

        unsafe {
            let ptr = bytes.as_mut_ptr();
            ptr::copy_nonoverlapping(blob, ptr, length);
            bindings::MagickRelinquishMemory(blob as *mut c_void);
        }

        bytes
    }
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
