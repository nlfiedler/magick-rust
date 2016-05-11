use std::ffi::{CStr, CString};
use std::ptr;
use libc::{c_uint, c_double, c_void};
use ::filters::FilterType;
use ::bindings;

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
