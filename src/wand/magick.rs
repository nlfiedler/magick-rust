use std::fmt;
use std::ptr;
use std::ffi::{CStr, CString};
use libc::{c_uint, c_double, c_void};

use ::filters::FilterType;
use ::bindings;
use ::conversions::*;
use super::{DrawingWand, PixelWand};


/// MagickWand is a Rustic wrapper to the Rust bindings to ImageMagick.
///
/// Instantiating a `MagickWand` will construct an ImageMagick "wand"
/// on which operations can be performed via the `MagickWand` functions.
/// When the `MagickWand` is dropped, the ImageMagick wand will be
/// destroyed as well.
wand_common!(
    MagickWand,
    NewMagickWand, ClearMagickWand, IsMagickWand, CloneMagickWand, DestroyMagickWand,
    MagickClearException, MagickGetExceptionType, MagickGetException
);

impl MagickWand {

    pub fn new_image(&self, columns: u64, rows: u64, pixel_wand: &PixelWand) -> Result<(), &'static str> {
        match unsafe { bindings::MagickNewImage(self.wand, columns, rows, pixel_wand.wand) } {
            bindings::MagickTrue => Ok(()),
            _ => Err("Could not create image"),
        }
    }

    pub fn annotate_image(&mut self, drawing_wand: &DrawingWand, x: f64, y: f64, angle: f64, text: &str) -> Result<(), &'static str> {
        let c_string = try!(CString::new(text).map_err(|_| "could not convert to cstring"));
        match unsafe { bindings::MagickAnnotateImage(self.wand, drawing_wand.wand, x, y, angle, c_string.as_ptr() as *const _) } {
            bindings::MagickTrue => Ok(()),
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
            bindings::MagickTrue => Ok(()),
            _ => Err("failed to add label")
        }
    }

    pub fn write_images(&self, path: &str, adjoin: bool) -> Result<(), &'static str> {
        let c_name = CString::new(path).unwrap();
        let result = unsafe {
            bindings::MagickWriteImages(self.wand, c_name.as_ptr(), adjoin.to_magick())
        };
        match result {
            bindings::MagickTrue => Ok(()),
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

    /// Detect if the loaded image is not in top-left orientation, and
    /// hence should be "auto" oriented so it is suitable for viewing.
    pub fn requires_orientation(&self) -> bool {
        unsafe {
            bindings::MagickGetImageOrientation(self.wand) != bindings::TopLeftOrientation
        }
    }

    /// Automatically adjusts the loaded image so that its orientation is
    /// suitable for viewing (i.e. top-left orientation).
    ///
    /// Returns `true` if successful or `false` if an error occurred.
    pub fn auto_orient(&self) -> bool {
        unsafe {
            bindings::MagickAutoOrientImage(self.wand) == bindings::MagickTrue
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
        get_colorspace,                  set_colorspace,                  MagickGetColorspace,               MagickSetColorspace,              u32
        get_compression,                 set_compression,                 MagickGetCompression,              MagickSetCompression,             u32
        get_compression_quality,         set_compression_quality,         MagickGetCompressionQuality,       MagickSetCompressionQuality,      u64
        get_gravity,                     set_gravity,                     MagickGetGravity,                  MagickSetGravity,                 u32
        get_image_colorspace,            set_image_colorspace,            MagickGetImageColorspace,          MagickSetImageColorspace,         u32
        get_image_compose,               set_image_compose,               MagickGetImageCompose,             MagickSetImageCompose,            u32
        get_image_compression,           set_image_compression,           MagickGetImageCompression,         MagickSetImageCompression,        u32
        get_image_compression_quality,   set_image_compression_quality,   MagickGetImageCompressionQuality,  MagickSetImageCompressionQuality, u64
        get_image_delay,                 set_image_delay,                 MagickGetImageDelay,               MagickSetImageDelay,              u64
        get_image_depth,                 set_image_depth,                 MagickGetImageDepth,               MagickSetImageDepth,              u64
        get_image_dispose,               set_image_dispose,               MagickGetImageDispose,             MagickSetImageDispose,            u32
        get_image_endian,                set_image_endian,                MagickGetImageEndian,              MagickSetImageEndian,             u32
        get_image_fuzz,                  set_image_fuzz,                  MagickGetImageFuzz,                MagickSetImageFuzz,               f64
        get_image_gamma,                 set_image_gamma,                 MagickGetImageGamma,               MagickSetImageGamma,              f64
        get_image_gravity,               set_image_gravity,               MagickGetImageGravity,             MagickSetImageGravity,            u32
        get_image_index,                 set_image_index,                 MagickGetImageIndex,               MagickSetImageIndex,              i64
        get_image_interlace_scheme,      set_image_interlace_scheme,      MagickGetImageInterlaceScheme,     MagickSetImageInterlaceScheme,    u32
        get_image_interpolate_method,    set_image_interpolate_method,    MagickGetImageInterpolateMethod,   MagickSetImageInterpolateMethod,  u32
        get_image_iterations,            set_image_iterations,            MagickGetImageIterations,          MagickSetImageIterations,         u64
        get_image_orientation,           set_image_orientation,           MagickGetImageOrientation,         MagickSetImageOrientation,        u32
        get_image_rendering_intent,      set_image_rendering_intent,      MagickGetImageRenderingIntent,     MagickSetImageRenderingIntent,    u32
        get_image_scene,                 set_image_scene,                 MagickGetImageScene,               MagickSetImageScene,              u64
        get_image_type,                  set_image_type,                  MagickGetImageType,                MagickSetImageType,               u32
        get_image_units,                 set_image_units,                 MagickGetImageUnits,               MagickSetImageUnits,              u32
        get_interlace_scheme,            set_interlace_scheme,            MagickGetInterlaceScheme,          MagickSetInterlaceScheme,         u32
        get_interpolate_method,          set_interpolate_method,          MagickGetInterpolateMethod,        MagickSetInterpolateMethod,       u32
        get_iterator_index,              set_iterator_index,              MagickGetIteratorIndex,            MagickSetIteratorIndex,           i64
        get_orientation,                 set_orientation,                 MagickGetOrientation,              MagickSetOrientation,             u32
        get_pointsize,                   set_pointsize,                   MagickGetPointsize,                MagickSetPointsize,               f64
        get_type,                        set_type,                        MagickGetType,                     MagickSetType,                    u32
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
