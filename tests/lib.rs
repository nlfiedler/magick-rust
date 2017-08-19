/*
 * Copyright 2015-2017 Nathan Fiedler
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

use magick_rust::{MagickWand, magick_wand_genesis, MetricType};

use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::sync::{Once, ONCE_INIT};

// TODO: nathan does not understand how to expose the FilterType without
//       this ugliness, his Rust skills are sorely lacking
use magick_rust::bindings;

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
    wand.resize_image(halfwidth, halfheight, bindings::FilterType::LanczosFilter);
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
    assert!(wand.read_image_blob(&data).is_ok());
    assert_eq!(512, wand.get_image_width());
    assert_eq!(384, wand.get_image_height());
}

#[test]
fn test_write_image_to_blob() {
    START.call_once(|| {
        magick_wand_genesis();
    });
    let wand = MagickWand::new();
    assert!(wand.read_image("tests/data/IMG_5745.JPG").is_ok());
    assert_eq!(512, wand.get_image_width());
    assert_eq!(384, wand.get_image_height());
    let blob = wand.write_image_blob("jpeg").unwrap();
    let blob_len = blob.len();
    // There is a slight degree of variability from platform to platform,
    // and version to version of ImageMagick.
    assert!(blob_len > 103000 && blob_len < 105000);
    // should be able to read it back again
    assert!(wand.read_image_blob(&blob).is_ok());
    assert_eq!(512, wand.get_image_width());
    assert_eq!(384, wand.get_image_height());
}

#[test]
fn test_write_images_to_blob() {
    START.call_once(|| {
        magick_wand_genesis();
    });
    let wand = MagickWand::new();
    assert!(wand.read_image("tests/data/IMG_5745.JPG").is_ok());
    assert_eq!(512, wand.get_image_width());
    assert_eq!(384, wand.get_image_height());
    let blob = wand.write_images_blob("jpeg").unwrap();
    let blob_len = blob.len();
    // There is a slight degree of variability from platform to platform,
    // and version to version of ImageMagick.
    assert!(blob_len > 103000 && blob_len < 105000);
    // should be able to read it back again
    assert!(wand.read_image_blob(&blob).is_ok());
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
    let found_value = wand.get_image_property("exif:DateTimeOriginal");
    assert!(found_value.is_ok());
    assert_eq!("2014:04:23 13:33:08", found_value.unwrap());
    // retrieve a property that does not exist
    let missing_value = wand.get_image_property("exif:Foobar");
    assert!(missing_value.is_err());
    assert_eq!("missing property", missing_value.unwrap_err());
}

#[test]
fn test_requires_orientation() {
    START.call_once(|| {
        magick_wand_genesis();
    });
    let wand = MagickWand::new();
    assert!(wand.read_image("tests/data/IMG_5745.JPG").is_ok());
    assert_eq!(false, wand.requires_orientation());
}

#[test]
fn test_auto_orient() {
    START.call_once(|| {
        magick_wand_genesis();
    });
    let wand = MagickWand::new();
    assert!(wand.read_image("tests/data/IMG_5745_rotl.JPG").is_ok());
    assert_eq!(true, wand.requires_orientation());
    assert!(wand.auto_orient());
    assert_eq!(false, wand.requires_orientation());
}

#[test]
fn test_compare_images() {
    START.call_once(|| {
        magick_wand_genesis();
    });
    let wand1 = MagickWand::new();
    assert!(wand1.read_image("tests/data/IMG_5745.JPG").is_ok());

    let wand2 = MagickWand::new();
    assert!(wand2.read_image("tests/data/IMG_5745_rotl.JPG").is_ok());
    wand2.auto_orient();

    let (distortion, diff) = wand1.compare_images(&wand2, MetricType::RootMeanSquaredErrorMetric);
    assert!(distortion < 0.01);
    assert!(diff.is_some());
}

#[test]
fn test_set_option() {
    START.call_once(|| {
        magick_wand_genesis();
    });
    let mut wand = MagickWand::new();
    assert!(wand.read_image("tests/data/IMG_5745.JPG").is_ok());
    // The jpeg:size option is just a hint.
    wand.set_option("jpeg:size", "128x128").unwrap();
    let blob = wand.write_image_blob("jpeg").unwrap();
    assert!(wand.read_image_blob(&blob).is_ok());
    assert_eq!(192, wand.get_image_width());
    assert_eq!(144, wand.get_image_height());
}
