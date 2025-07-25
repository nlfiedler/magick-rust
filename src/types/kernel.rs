/*
 * Copyright 2024 5ohue
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
use std::ffi::CString;
use std::ops::Deref;
use crate::bindings;
use crate::{MagickError, Result};

/// Builder, that creates instances of [KernelInfo](self::KernelInfo)
///
/// # Examples
///
/// Here is an example of how you can use this struct to create a kernel to convolve an image:
///
/// ```
/// use magick_rust::{MagickWand, PixelWand, KernelBuilder};
///
/// fn main() -> Result<(), magick_rust::MagickError> {
///     let mut wand1 = MagickWand::new();
///     wand1.new_image(4, 4, &PixelWand::new())?; // Replace with `read_image` to open your image file
///     let wand2 = wand1.clone();
///
///     let kernel_info = KernelBuilder::default()
///         .set_size((3, 3))
///         .set_center((1, 1)) // Not really needed here - the center is in the middle of kernel
///                             // by default
///         .set_values(&[0.111, 0.111, 0.111,
///                       0.111, 0.111, 0.111,
///                       0.111, 0.111, 0.111])
///         .build()?;
///
///     wand1.convolve_image(&kernel_info)?;
///
///     Ok(())
/// }
/// ```
///
/// Here is an example of how you can use this struct to create builtin kernel to gaussian blur an
/// image (not the best way to do it, just an example):
///
/// ```
/// use magick_rust::{MagickWand, PixelWand, KernelBuilder, KernelInfoType, GeometryInfo};
///
/// fn main() -> Result<(), magick_rust::MagickError> {
///     let mut wand1 = MagickWand::new();
///     wand1.new_image(4, 4, &PixelWand::new())?; // Replace with `read_image` to open your image file
///     let wand2 = wand1.clone();
///
///     let mut geom_info = GeometryInfo::new();
///     geom_info.set_sigma(15.0);
///     let kernel_info = KernelBuilder::default()
///         .set_info_type(KernelInfoType::Gaussian)
///         .set_geom_info(geom_info)
///         .build_builtin()?;
///
///     wand1.convolve_image(&kernel_info)?;
///
///     Ok(())
/// }
/// ```
#[derive(Debug, Clone, Default)]
pub struct KernelBuilder {
    size: Option<(usize, usize)>,
    center: Option<(usize, usize)>,
    values: Option<Vec<f64>>,

    info_type: Option<crate::KernelInfoType>,
    geom_info: Option<crate::GeometryInfo>,
}

impl KernelBuilder {
    /// Used for user defined kernels
    pub fn set_size(mut self, size: (usize, usize)) -> KernelBuilder {
        self.size = Some(size);
        self
    }

    /// Used for user defined kernels
    pub fn set_center(mut self, center: (usize, usize)) -> KernelBuilder {
        self.center = Some(center);
        self
    }

    /// Used for user defined kernels
    pub fn set_values(mut self, values: &[f64]) -> KernelBuilder {
        self.values = Some(values.into());
        self
    }

    pub fn build(&self) -> Result<KernelInfo> {
        let size = self
            .size
            .ok_or(MagickError("no kernel size given".to_string()))?;
        let values = self
            .values
            .as_ref()
            .ok_or(MagickError("no kernel values given".to_string()))?;

        if values.len() != size.0 * size.1 {
            return Err(MagickError(
                "kernel size doesn't match kernel values size".to_string(),
            ));
        }

        // Create kernel string
        let mut kernel_string = if let Some(center) = self.center {
            format!("{}x{}+{}+{}:", size.0, size.1, center.0, center.1)
        } else {
            format!("{}x{}:", size.0, size.1,)
        };

        // Add values
        values.iter().for_each(|x| {
            kernel_string.push_str(&format!("{x},"));
        });

        // Remove trailing ","
        kernel_string.pop();

        // Create null terminated string
        let c_kernel_string = CString::new(kernel_string).expect("CString::new() has failed");

        // Create kernel info
        let kernel_info =
            unsafe { bindings::AcquireKernelInfo(c_kernel_string.as_ptr(), std::ptr::null_mut()) };

        if kernel_info.is_null() {
            return Err(MagickError("failed to acquire kernel info".to_string()));
        }

        Ok(KernelInfo::new(kernel_info))
    }

    /// Used for builtin kernels
    pub fn set_info_type(mut self, info_type: crate::KernelInfoType) -> KernelBuilder {
        self.info_type = Some(info_type);
        self
    }

    /// Used for builtin kernels
    pub fn set_geom_info(mut self, geom_info: crate::GeometryInfo) -> KernelBuilder {
        self.geom_info = Some(geom_info);
        self
    }

    pub fn build_builtin(&self) -> Result<KernelInfo> {
        let info_type = self
            .info_type
            .ok_or(MagickError("no info type given".to_string()))?;
        let geom_info = self
            .geom_info
            .ok_or(MagickError("no geometry info given".to_string()))?;

        // Create kernel info
        let kernel_info = unsafe {
            bindings::AcquireKernelBuiltIn(info_type.into(), geom_info.inner(), std::ptr::null_mut())
        };

        if kernel_info.is_null() {
            return Err(MagickError(
                "failed to acquire builtin kernel info".to_string(),
            ));
        }

        Ok(KernelInfo::new(kernel_info))
    }
}

pub struct KernelInfo {
    kernel_info: *mut bindings::KernelInfo,
}

impl KernelInfo {
    fn new(kernel_info: *mut bindings::KernelInfo) -> KernelInfo {
        KernelInfo { kernel_info }
    }

    /// The values within the kernel is scaled directly using given scaling factor without change.
    pub fn scale(&mut self, factor: f64) {
        unsafe { bindings::ScaleKernelInfo(self.kernel_info, factor, bindings::GeometryFlags::NoValue) }
    }

    /// Kernel normalization is designed to ensure that any use of the kernel scaling factor with
    /// 'Convolve' or 'Correlate' morphology methods will fall into -1.0 to +1.0 range. Note that
    /// for non-HDRI versions of IM this may cause images to have any negative results clipped,
    /// unless some 'bias' is used.
    ///
    /// More specifically. Kernels which only contain positive values (such as a 'Gaussian' kernel)
    /// will be scaled so that those values sum to +1.0, ensuring a 0.0 to +1.0 output range for
    /// non-HDRI images.
    ///
    /// For Kernels that contain some negative values, (such as 'Sharpen' kernels) the kernel will
    /// be scaled by the absolute of the sum of kernel values, so that it will generally fall
    /// within the +/- 1.0 range.
    ///
    /// For kernels whose values sum to zero, (such as 'Laplacian' kernels) kernel will be scaled
    /// by just the sum of the positive values, so that its output range will again fall into the
    /// +/- 1.0 range.
    pub fn normalize(&mut self) {
        unsafe {
            bindings::ScaleKernelInfo(
                self.kernel_info,
                1.0,
                bindings::GeometryFlags::NormalizeValue,
            )
        }
    }

    /// For special kernels designed for locating shapes using 'Correlate', (often only containing
    /// +1 and -1 values, representing foreground/background matching) a special normalization
    /// method is provided to scale the positive values separately to those of the negative values,
    /// so the kernel will be forced to become a zero-sum kernel better suited to such searches.
    pub fn correlate_normalize(&mut self) {
        unsafe {
            bindings::ScaleKernelInfo(
                self.kernel_info,
                1.0,
                bindings::GeometryFlags::CorrelateNormalizeValue,
            )
        }
    }

    /// Adds a given amount of the 'Unity' Convolution Kernel to the given pre-scaled and
    /// normalized Kernel. This in effect adds that amount of the original image into the resulting
    /// convolution kernel. This value is usually provided by the user as a percentage value in the
    /// 'convolve:scale' setting.
    ///
    /// The resulting effect is to convert the defined kernels into blended soft-blurs, unsharp
    /// kernels or into sharpening kernels.
    pub fn unity_add(&mut self, scale: f64) {
        unsafe { bindings::UnityAddKernelInfo(self.kernel_info, scale) }
    }

    pub unsafe fn get_ptr(&self) -> *mut bindings::KernelInfo {
        self.kernel_info
    }
}

impl Drop for KernelInfo {
    fn drop(&mut self) {
        unsafe { bindings::DestroyKernelInfo(self.kernel_info) };
    }
}

impl Clone for KernelInfo {
    fn clone(&self) -> Self {
        let kernel_info = unsafe { bindings::CloneKernelInfo(self.kernel_info) };

        if kernel_info.is_null() {
            panic!("failed to clone kernel info");
        }

        KernelInfo::new(kernel_info)
    }
}

impl std::fmt::Debug for KernelInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unsafe { write!(f, "{:?}", *self.kernel_info) }
    }
}
