use std::marker::PhantomData;

use crate::bindings;

pub struct Image<'a> {
    image: *mut bindings::Image,
    phantom_data: PhantomData<&'a bindings::Image>,
}

impl Image<'_> {
    pub unsafe fn new(img: *mut bindings::Image) -> Self {
        Image {
            image: img,
            phantom_data: PhantomData
        }
    }

    pub unsafe fn get_ptr(&self) -> *mut bindings::Image {
        self.image
    }
}
