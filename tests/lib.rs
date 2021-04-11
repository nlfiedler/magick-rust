/*
 * Copyright 2015-2018 Nathan Fiedler
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

use magick_rust::{bindings, magick_wand_genesis, MagickWand, PixelWand, ResourceType};

use magick_rust::ToMagick;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::sync::Once;

// Used to make sure MagickWand is initialized exactly once. Note that we
// do not bother shutting down, we simply exit when the tests are done.
static START: Once = Once::new();

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
        width => width / 2,
    };
    let halfheight = match wand.get_image_height() {
        1 => 1,
        height => height / 2,
    };
    wand.resize_image(halfwidth, halfheight, bindings::FilterType_LanczosFilter);
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
        Err(why) => panic!("couldn't open file: {}", Error::to_string(&why)),
        Ok(file) => file,
    };
    let mut data: Vec<u8> = Vec::new();
    match file.read_to_end(&mut data) {
        Err(why) => panic!("couldn't read file: {}", Error::to_string(&why)),
        Ok(_) => (),
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

    let (distortion, diff) =
        wand1.compare_images(&wand2, bindings::MetricType_RootMeanSquaredErrorMetric);
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

#[test]
fn test_page_geometry() {
    START.call_once(|| {
        magick_wand_genesis();
    });
    let wand = MagickWand::new();
    assert!(wand.read_image("tests/data/rust.gif").is_ok());
    assert_eq!((156, 150, 39, 36), wand.get_image_page()); /* width, height, x offset, y offset */
    assert_eq!(80, wand.get_image_width());
    assert_eq!(76, wand.get_image_height());
}

#[test]
fn test_transform_image_colorspace() {
    START.call_once(|| {
        magick_wand_genesis();
    });
    let wand = MagickWand::new();
    assert!(wand.read_image("tests/data/IMG_5745.JPG").is_ok());
    assert_eq!(
        wand.get_image_colorspace(),
        bindings::ColorspaceType_sRGBColorspace
    );

    let pixel_color = wand.get_image_pixel_color(10, 10).unwrap();
    assert_ne!(pixel_color.get_hsl().hue, 0.0);

    assert!(wand
        .transform_image_colorspace(bindings::ColorspaceType_GRAYColorspace)
        .is_ok());
    assert_eq!(
        wand.get_image_colorspace(),
        bindings::ColorspaceType_GRAYColorspace
    );

    let pixel_grayscale = wand.get_image_pixel_color(10, 10).unwrap();
    assert_eq!(pixel_grayscale.get_hsl().hue, 0.0);

    /* The output of `export_image_pixels` should match
     * `convert -type Grayscale tests/data/IMG_5745.JPG[2x2+0+0] txt:` */
    assert_eq!(
        wand.export_image_pixels(0, 0, 2, 2, "I").unwrap(),
        vec![212, 212, 210, 210]
    )
}

#[test]
fn test_color_reduction() {
    START.call_once(|| {
        magick_wand_genesis();
    });
    let wand = MagickWand::new();
    assert!(wand.read_image("tests/data/IMG_5745.JPG").is_ok());
    // There is a slight degree of variability from platform to platform,
    // and version to version of ImageMagick.
    let image_colors = wand.get_image_colors();
    assert!(image_colors > 38000 || image_colors < 40000);

    assert!(wand
        .quantize_image(
            6,
            bindings::ColorspaceType_RGBColorspace,
            1,
            bindings::DitherMethod_UndefinedDitherMethod,
            false.to_magick()
        )
        .is_ok());
    assert_eq!(6, wand.get_image_colors());

    let histogram = wand.get_image_histogram().unwrap();
    assert_eq!(6, histogram.len());
    assert_eq!(
        wand.get_image_width() * wand.get_image_height(),
        histogram.iter().fold(0, |total_colors, wand| total_colors
            + wand.get_color_count())
    );
}

#[test]
fn test_set_image_background_color() {
    START.call_once(|| {
        magick_wand_genesis();
    });
    let wand = MagickWand::new();
    assert!(wand.read_image("tests/data/rust.png").is_ok());
    let mut pw = PixelWand::new();
    pw.set_color("#0000FF").unwrap();
    wand.set_image_background_color(&pw).unwrap();
    wand.set_image_alpha_channel(bindings::AlphaChannelOption_RemoveAlphaChannel)
        .unwrap();
    let blob = wand.write_image_blob("rgb").unwrap();
    assert_eq!(0u8, blob[0]);
    assert_eq!(0u8, blob[1]);
    assert_eq!(255u8, blob[2]);
}

#[test]
fn test_set_background_color() {
    START.call_once(|| {
        magick_wand_genesis();
    });
    let wand = MagickWand::new();
    let mut pw = PixelWand::new();
    pw.set_color("none").unwrap();
    wand.set_background_color(&pw).unwrap();
    assert!(wand.read_image("tests/data/rust.svg").is_ok());
    let blob = wand.write_image_blob("rgba").unwrap();
    assert_eq!(0u8, blob[0]);
    assert_eq!(0u8, blob[1]);
    assert_eq!(0u8, blob[2]);
    assert_eq!(0u8, blob[3]);
}

#[test]
fn test_set_size() {
    let wand = MagickWand::new();
    assert!(wand.set_size(100, 100).is_ok());
}

#[test]
fn test_clut_image() {
    START.call_once(|| {
        magick_wand_genesis();
    });
    let wand = MagickWand::new();
    assert!(wand.read_image("tests/data/IMG_5745.JPG").is_ok());

    let mut gradient = MagickWand::new();
    assert!(gradient.set_size(128, 20).is_ok());
    assert!(gradient.set_option("gradient:angle", "90").is_ok());
    assert!(gradient.read_image("gradient:black-yellow").is_ok());

    assert!(wand
        .clut_image(
            &gradient,
            bindings::PixelInterpolateMethod_BilinearInterpolatePixel
        )
        .is_ok());
}

#[test]
fn test_negate_image() {
    START.call_once(|| {
        magick_wand_genesis();
    });
    let wand = MagickWand::new();
    assert!(wand.read_image("tests/data/rust.png").is_ok());
    wand.negate_image().unwrap();
    let pixel_color = wand.get_image_pixel_color(0, 0).unwrap();
    assert_eq!(
        "srgb(255,255,255)",
        pixel_color.get_color_as_string().unwrap()
    );
}

#[test]
fn test_resource_limits() {
    START.call_once(|| {
        magick_wand_genesis();
    });
    MagickWand::set_resource_limit(ResourceType::Thread, 1).unwrap();
    let wand = MagickWand::new();
    assert!(wand.read_image("tests/data/rust.png").is_ok());
}
