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

use crate::bindings;

pub struct Image<'a> {
    image: *mut bindings::Image,
    phantom_data: PhantomData<&'a bindings::Image>,
}

impl Image<'_> {
    pub fn new(img: *mut bindings::Image) -> Self {
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

    pub unsafe fn get_ptr(&self) -> *mut bindings::Image {
        self.image
    }
}
