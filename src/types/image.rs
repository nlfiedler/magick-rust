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
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

use crate::bindings;
use crate::result::{MagickError, Result};
use crate::wand::MagickWand;

pub struct Image<'a> {
    image: *mut bindings::Image,
    phantom_data: PhantomData<&'a bindings::Image>,
}

impl Image<'_> {
    pub (crate) fn new(img: *mut bindings::Image) -> Self {
        // SAFETY: This is safe and also does not require an Image::drop() call as:
        //         The lifetime of Image is the same as the lifetime of wrapped bindings::Image.
        //         The bindings::Image is borrowed by the caller from MagickWand.
        //         Magickwand::drop() destroys the bindings::MagickWand, which
        //         destroys both the associated bindings::Image and itself.
        Image {
            image: img,
            phantom_data: PhantomData,
        }
    }

    pub (crate) unsafe fn get_ptr(&self) -> *mut bindings::Image {
        self.image
    }
}

/// Move the wand's internal iterator to `index`.
///
/// ImageMagick's `MagickWand` has a single internal iterator that every
/// per-image operation reads from. The image-list views below re-pin that
/// iterator on every frame access so that an unrelated call cannot leave it
/// pointing at the wrong frame.
///
/// This is a `&self` counterpart to [`MagickWand::set_iterator_index`] (which
/// takes `&mut self`); the shared borrow is sound because it mutates only the
/// iterator cursor in C-allocated memory behind the wand pointer. `index` is
/// always validated against the image count before this is called, so the
/// boolean return is ignored.
fn pin(wand: &MagickWand, index: isize) {
    debug_assert!(
        index >= 0 && (index as usize) < wand.get_number_images(),
        "frame index {index} out of bounds (count {})",
        wand.get_number_images()
    );
    unsafe {
        bindings::MagickSetIteratorIndex(wand.as_ptr(), index);
    }
}

/// A read-only view over the images (frames) held by a [`MagickWand`].
///
/// Obtained from [`MagickWand::images`]. Creating the view resets the wand's
/// iterator, and each frame handed out by [`Images::get`], [`Images::first`],
/// [`Images::last`], or the iteration helpers re-pins the iterator to the
/// correct frame before delegating, so frame access stays consistent regardless
/// of call order.
pub struct Images<'w> {
    wand: &'w MagickWand,
}

impl<'w> Images<'w> {
    pub(crate) fn new(wand: &'w MagickWand) -> Self {
        wand.reset_iterator();
        Images { wand }
    }

    /// The number of images (frames) in the list.
    pub fn count(&self) -> usize {
        self.wand.get_number_images()
    }

    /// Returns `true` if the list contains no images.
    pub fn is_empty(&self) -> bool {
        self.count() == 0
    }

    /// Borrow the frame at `index`, or `None` if out of bounds.
    pub fn get(&self, index: usize) -> Option<ImageRef<'_>> {
        if index < self.count() {
            Some(ImageRef {
                wand: self.wand,
                index: index as isize,
            })
        } else {
            None
        }
    }

    /// Borrow the first frame, or `None` if the list is empty.
    pub fn first(&self) -> Option<ImageRef<'_>> {
        self.get(0)
    }

    /// Borrow the last frame, or `None` if the list is empty.
    pub fn last(&self) -> Option<ImageRef<'_>> {
        self.count().checked_sub(1).and_then(|i| self.get(i))
    }

    /// Visit every frame in order, passing its index and a borrow to `f`.
    pub fn for_each(&self, mut f: impl FnMut(usize, ImageRef<'_>)) {
        for index in 0..self.count() {
            f(
                index,
                ImageRef {
                    wand: self.wand,
                    index: index as isize,
                },
            );
        }
    }

    /// Like [`Images::for_each`], but `f` may fail; the first error stops
    /// iteration and is returned.
    pub fn try_for_each(&self, mut f: impl FnMut(usize, ImageRef<'_>) -> Result<()>) -> Result<()> {
        for index in 0..self.count() {
            f(
                index,
                ImageRef {
                    wand: self.wand,
                    index: index as isize,
                },
            )?;
        }
        Ok(())
    }
}

/// A mutable view over the images (frames) held by a [`MagickWand`].
///
/// Obtained from [`MagickWand::images_mut`]. Because ImageMagick exposes a
/// single internal iterator, only one [`ImageMut`] may be borrowed at a time;
/// this is enforced by the borrow checker, as each accessor borrows the view
/// mutably.
pub struct ImagesMut<'w> {
    wand: &'w mut MagickWand,
}

impl<'w> ImagesMut<'w> {
    pub(crate) fn new(wand: &'w mut MagickWand) -> Self {
        wand.reset_iterator();
        ImagesMut { wand }
    }

    /// The number of images (frames) in the list.
    pub fn count(&self) -> usize {
        self.wand.get_number_images()
    }

    /// Returns `true` if the list contains no images.
    pub fn is_empty(&self) -> bool {
        self.count() == 0
    }

    /// Mutably borrow the frame at `index`, or `None` if out of bounds.
    pub fn get(&mut self, index: usize) -> Option<ImageMut<'_>> {
        if index < self.count() {
            Some(ImageMut {
                wand: &mut *self.wand,
                index: index as isize,
            })
        } else {
            None
        }
    }

    /// Mutably borrow the first frame, or `None` if the list is empty.
    pub fn first(&mut self) -> Option<ImageMut<'_>> {
        self.get(0)
    }

    /// Mutably borrow the last frame, or `None` if the list is empty.
    pub fn last(&mut self) -> Option<ImageMut<'_>> {
        self.count().checked_sub(1).and_then(|i| self.get(i))
    }

    /// Remove the frame at `index` from the list.
    ///
    /// Note that removing a frame shifts the indices of all later frames down
    /// by one, so any indices computed before the removal are stale afterwards.
    pub fn remove(&mut self, index: usize) -> Result<()> {
        let count = self.count();
        if index >= count {
            return Err(MagickError(format!(
                "image index {index} out of bounds (count {count})"
            )));
        }
        pin(self.wand, index as isize);
        self.wand.remove_image()
    }

    /// Append all frames from `other` to the end of this list.
    pub fn append(&mut self, other: &MagickWand) -> Result<()> {
        self.wand.set_last_iterator();
        self.wand.add_image(other)
    }

    /// Visit every frame in order, passing its index and a mutable borrow to
    /// `f`.
    ///
    /// `f` must not add or remove frames (e.g. via [`ImageMut`]'s deref to
    /// [`MagickWand::remove_image`] / [`MagickWand::add_image`]): the set of
    /// indices to visit is fixed when iteration begins, so changing the frame
    /// count mid-iteration visits the wrong frames. Use [`ImagesMut::remove`]
    /// or [`ImagesMut::append`] before or after iterating instead.
    pub fn for_each(&mut self, mut f: impl FnMut(usize, ImageMut<'_>)) {
        for index in 0..self.count() {
            f(
                index,
                ImageMut {
                    wand: &mut *self.wand,
                    index: index as isize,
                },
            );
        }
    }

    /// Like [`ImagesMut::for_each`], but `f` may fail; the first error stops
    /// iteration and is returned. The same "do not add or remove frames"
    /// caveat as [`ImagesMut::for_each`] applies.
    pub fn try_for_each(
        &mut self,
        mut f: impl FnMut(usize, ImageMut<'_>) -> Result<()>,
    ) -> Result<()> {
        for index in 0..self.count() {
            f(
                index,
                ImageMut {
                    wand: &mut *self.wand,
                    index: index as isize,
                },
            )?;
        }
        Ok(())
    }
}

/// A handle to a single frame for read-only access.
///
/// Each accessor pins the wand's iterator to this frame and then reads, so the
/// returned values always correspond to this frame regardless of what other
/// frames are touched in between. Unlike [`ImageMut`], this handle deliberately
/// does *not* deref to [`MagickWand`]: doing so would both expose the wand's
/// image-mutating methods (defeating the read-only contract) and hand out a
/// `&MagickWand` that silently goes stale once another frame is accessed.
pub struct ImageRef<'a> {
    wand: &'a MagickWand,
    index: isize,
}

/// Generate read-only forwarding accessors on [`ImageRef`]. Each pins the
/// iterator to the frame, then delegates to the eponymous [`MagickWand`] getter.
macro_rules! frame_getters {
    ($($name:ident($($arg:ident: $ty:ty),*) -> $ret:ty;)*) => {
        impl ImageRef<'_> {
            $(
                #[doc = concat!(
                    "Read this frame's value; see [`MagickWand::",
                    stringify!($name), "`]."
                )]
                pub fn $name(&self, $($arg: $ty),*) -> $ret {
                    pin(self.wand, self.index);
                    self.wand.$name($($arg),*)
                }
            )*
        }
    };
}

frame_getters! {
    get_image_width() -> usize;
    get_image_height() -> usize;
    get_image_page() -> (usize, usize, isize, isize);
    get_image_resolution() -> Result<(f64, f64)>;
    get_image_range() -> Result<(f64, f64)>;
    get_image_colors() -> usize;
    get_image_alpha_channel() -> bool;
    get_image_virtual_pixel_method() -> crate::VirtualPixelMethod;
    get_image_pixel_color(x: isize, y: isize) -> Option<crate::PixelWand>;
    get_image_histogram() -> Option<Vec<crate::PixelWand>>;
    get_image_artifact(artifact: &str) -> Result<String>;
    get_image_artifacts(pattern: &str) -> Result<Vec<String>>;
    get_image_property(name: &str) -> Result<String>;
    get_image_properties(pattern: &str) -> Result<Vec<String>>;
    get_image_format() -> Result<String>;
    get_image_filename() -> Result<String>;
    get_image_compose() -> crate::CompositeOperator;
    get_image_colorspace() -> crate::ColorspaceType;
    get_image_compression() -> crate::CompressionType;
    get_image_compression_quality() -> usize;
    get_image_delay() -> usize;
    get_image_depth() -> usize;
    get_image_dispose() -> crate::DisposeType;
    get_image_endian() -> crate::EndianType;
    get_image_fuzz() -> f64;
    get_image_gamma() -> f64;
    get_image_gravity() -> crate::GravityType;
    get_image_interlace_scheme() -> crate::InterlaceType;
    get_image_interpolate_method() -> crate::PixelInterpolateMethod;
    get_image_iterations() -> usize;
    get_image_orientation() -> crate::OrientationType;
    get_image_rendering_intent() -> crate::RenderingIntent;
    get_image_scene() -> usize;
    get_image_type() -> crate::ImageType;
    get_image_units() -> crate::ResolutionType;
}

impl ImageRef<'_> {
    /// Borrow this frame as an [`Image`], e.g. for [`MagickWand::new_from_image`].
    ///
    /// The returned [`Image`] captures this frame's underlying image pointer, so
    /// it remains valid even if the wand's iterator is later moved to another
    /// frame.
    pub fn get_image(&self) -> Result<Image<'_>> {
        pin(self.wand, self.index);
        self.wand.get_image()
    }
}

/// A handle to a single frame for mutable access.
///
/// Derefs to [`MagickWand`], pinning the wand's iterator to this frame first,
/// so every existing per-image `MagickWand` operation works directly on the
/// frame.
pub struct ImageMut<'a> {
    wand: &'a mut MagickWand,
    index: isize,
}

impl Deref for ImageMut<'_> {
    type Target = MagickWand;

    fn deref(&self) -> &MagickWand {
        pin(self.wand, self.index);
        &*self.wand
    }
}

impl DerefMut for ImageMut<'_> {
    fn deref_mut(&mut self) -> &mut MagickWand {
        pin(self.wand, self.index);
        &mut *self.wand
    }
}
