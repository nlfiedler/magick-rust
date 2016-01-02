/*
 * Copyright 2015 Nathan Fiedler
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

extern crate magick_rust;

use magick_rust::{MagickWand, magick_wand_genesis};
use magick_rust::filters::{FilterType};

use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::sync::{Once, ONCE_INIT};

// Used to make sure MagickWand is initialized exactly once. Note that we
// do not bother shutting down, we simply exit when the tests are done.
static START: Once = ONCE_INIT;

#[test]
fn test_new_drop() {
    START.call_once(|| {
        magick_wand_genesis();
    });
    MagickWand::new();
}

#[test]
fn test_resize_image() {
    START.call_once(|| {
        magick_wand_genesis();
    });
    let wand = MagickWand::new();
    assert!(wand.read_image("tests/data/IMG_5745.JPG").is_ok());
    assert_eq!(512, wand.get_image_width());
    assert_eq!(384, wand.get_image_height());
    let halfwidth = match wand.get_image_width() {
        1 => 1,
        width => width / 2
    };
    let halfheight = match wand.get_image_height() {
        1 => 1,
        height => height / 2
    };
    wand.resize_image(halfwidth, halfheight, FilterType::LanczosFilter, 1.0);
    assert_eq!(256, wand.get_image_width());
    assert_eq!(192, wand.get_image_height());
}

#[test]
fn test_read_from_blob() {
    START.call_once(|| {
        magick_wand_genesis();
    });
    let wand = MagickWand::new();

    let path = Path::new("tests/data/IMG_5745.JPG");
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open file: {}", Error::description(&why)),
        Ok(file) => file
    };
    let mut data: Vec<u8> = Vec::new();
    match file.read_to_end(&mut data) {
        Err(why) => panic!("couldn't read file: {}", Error::description(&why)),
        Ok(_) => ()
    };
    assert!(wand.read_image_blob(data).is_ok());
    assert_eq!(512, wand.get_image_width());
    assert_eq!(384, wand.get_image_height());
}

#[test]
fn test_write_to_blob() {
    START.call_once(|| {
        magick_wand_genesis();
    });
    let wand = MagickWand::new();
    assert!(wand.read_image("tests/data/IMG_5745.JPG").is_ok());
    assert_eq!(512, wand.get_image_width());
    assert_eq!(384, wand.get_image_height());
    let blob = wand.write_image_blob("jpeg").unwrap();
    assert_eq!(104061, blob.len());
    // should be able to read it back again
    assert!(wand.read_image_blob(blob).is_ok());
    assert_eq!(512, wand.get_image_width());
    assert_eq!(384, wand.get_image_height());
}

#[test]
fn test_fit() {
    START.call_once(|| {
        magick_wand_genesis();
    });
    let wand = MagickWand::new();
    assert!(wand.read_image("tests/data/IMG_5745.JPG").is_ok());
    assert_eq!(512, wand.get_image_width());
    assert_eq!(384, wand.get_image_height());
    wand.fit(240, 240);
    assert_eq!(240, wand.get_image_width());
    assert_eq!(180, wand.get_image_height());
}

#[test]
fn test_get_image_property() {
    START.call_once(|| {
        magick_wand_genesis();
    });
    let wand = MagickWand::new();
    assert!(wand.read_image("tests/data/IMG_5745.JPG").is_ok());
    // retrieve a property we know exists
    let found_value = wand.get_image_property("exif:DateTime");
    assert!(found_value.is_ok());
    assert_eq!("2014:04:23 13:33:08", found_value.unwrap());
    // retrieve a property that does not exist
    let missing_value = wand.get_image_property("exif:Foobar");
    assert!(missing_value.is_err());
    assert_eq!("missing property", missing_value.unwrap_err());
}
