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

mod fixtures;

use std::error::Error;
use std::io::Read;
use std::sync::Once;

use crate::fixtures::{
    ALL_FIXTURES, IMG_5745_JPG, IMG_5745_ROTL_JPG, RUST_GIF, RUST_PNG, RUST_SVG,
};
use magick_rust::MagickError;
use magick_rust::{MagickWand, PixelWand, magick_wand_genesis};

// Used to make sure MagickWand is initialized exactly once. Note that we
// do not bother shutting down, we simply exit when the tests are done.
static START: Once = Once::new();

#[test]
fn test_opening_all_fixtures_and_their_widths_and_heights() {
    START.call_once(|| {
        magick_wand_genesis();
    });

    for fixture in ALL_FIXTURES.iter() {
        let wand = MagickWand::new();
        fixture.read_image(&wand);
        fixture.assert_width(&wand);
        fixture.assert_height(&wand);
    }
}

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
    IMG_5745_JPG.read_image(&wand);
    IMG_5745_JPG.assert_width(&wand);
    IMG_5745_JPG.assert_height(&wand);

    let half_width = match wand.get_image_width() {
        1 => 1,
        width => width / 2,
    };
    let half_height = match wand.get_image_height() {
        1 => 1,
        height => height / 2,
    };
    assert!(
        wand.resize_image(half_width, half_height, magick_rust::FilterType::Lanczos)
            .is_ok()
    );
    assert_eq!(256, wand.get_image_width());
    assert_eq!(192, wand.get_image_height());
}

#[test]
fn test_thumbnail_image() {
    START.call_once(|| {
        magick_wand_genesis();
    });
    let wand = MagickWand::new();
    IMG_5745_JPG.read_image(&wand);
    IMG_5745_JPG.assert_width(&wand);
    IMG_5745_JPG.assert_height(&wand);
    let halfwidth = match wand.get_image_width() {
        1 => 1,
        width => width / 2,
    };
    let halfheight = match wand.get_image_height() {
        1 => 1,
        height => height / 2,
    };
    assert!(wand.thumbnail_image(halfwidth, halfheight).is_ok());
    assert_eq!(256, wand.get_image_width());
    assert_eq!(192, wand.get_image_height());
}

#[test]
fn test_read_from_blob() {
    START.call_once(|| {
        magick_wand_genesis();
    });
    let wand = MagickWand::new();

    let mut file = IMG_5745_JPG.file();
    let mut data: Vec<u8> = Vec::new();
    if let Err(why) = file.read_to_end(&mut data) {
        panic!("couldn't read file: {}", <dyn Error>::to_string(&why))
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
    IMG_5745_JPG.read_image(&wand);
    IMG_5745_JPG.assert_width(&wand);
    IMG_5745_JPG.assert_height(&wand);
    let blob = wand.write_image_blob("jpeg").unwrap();
    let blob_len = blob.len();
    // There is a slight degree of variability from platform to platform,
    // and version to version of ImageMagick.
    assert!(blob_len > 103000 && blob_len < 105000);
    // should be able to read it back again
    assert!(wand.read_image_blob(&blob).is_ok());
    IMG_5745_JPG.assert_width(&wand);

    assert_eq!(384, wand.get_image_height());
}

#[test]
fn test_write_images_to_blob() {
    START.call_once(|| {
        magick_wand_genesis();
    });
    let wand = MagickWand::new();
    IMG_5745_JPG.read_image(&wand);
    IMG_5745_JPG.assert_width(&wand);
    IMG_5745_JPG.assert_height(&wand);
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
    IMG_5745_JPG.read_image(&wand);
    IMG_5745_JPG.assert_width(&wand);
    IMG_5745_JPG.assert_height(&wand);
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
    IMG_5745_JPG.read_image(&wand);
    // retrieve a property we know exists
    let found_value = wand.get_image_property("exif:DateTimeOriginal");
    assert!(found_value.is_ok());
    assert_eq!("2014:04:23 13:33:08", found_value.unwrap());
    // retrieve a property that does not exist
    let missing_value = wand.get_image_property("exif:Foobar");
    assert!(missing_value.is_err());
    assert_eq!(
        MagickError("missing property: exif:Foobar".to_string()),
        missing_value.unwrap_err()
    );
}

#[test]
fn test_requires_orientation() {
    START.call_once(|| {
        magick_wand_genesis();
    });
    let wand = MagickWand::new();
    IMG_5745_JPG.read_image(&wand);
    assert!(!wand.requires_orientation());
}

#[test]
fn test_auto_orient() {
    START.call_once(|| {
        magick_wand_genesis();
    });
    let wand = MagickWand::new();
    IMG_5745_ROTL_JPG.read_image(&wand);
    assert!(wand.requires_orientation());
    assert!(wand.auto_orient());
    assert!(!wand.requires_orientation());
}

#[test]
fn test_compare_images() {
    START.call_once(|| {
        magick_wand_genesis();
    });
    let wand1 = MagickWand::new();
    IMG_5745_JPG.read_image(&wand1);

    let wand2 = MagickWand::new();
    IMG_5745_ROTL_JPG.read_image(&wand2);
    wand2.auto_orient();

    let (distortion, diff) = wand1.compare_images(&wand2, magick_rust::MetricType::RootMeanSquared);
    assert!(distortion < 0.01);
    assert!(diff.is_some());
}

#[test]
fn test_set_option() {
    START.call_once(|| {
        magick_wand_genesis();
    });
    let mut wand = MagickWand::new();
    IMG_5745_JPG.read_image(&wand);
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
    RUST_GIF.read_image(&wand);
    assert_eq!((156, 150, 39, 36), wand.get_image_page()); /* width, height, x offset, y offset */
    RUST_GIF.assert_width(&wand);
    RUST_GIF.assert_height(&wand);
}

#[test]
fn test_transform_image_colorspace() {
    START.call_once(|| {
        magick_wand_genesis();
    });
    let wand = MagickWand::new();
    IMG_5745_JPG.read_image(&wand);
    assert_eq!(
        wand.get_image_colorspace(),
        magick_rust::ColorspaceType::sRGB
    );

    let pixel_color = wand.get_image_pixel_color(10, 10).unwrap();
    assert_ne!(pixel_color.get_hsl().hue, 0.0);

    assert!(
        wand.transform_image_colorspace(magick_rust::ColorspaceType::GRAY)
            .is_ok()
    );
    assert_eq!(
        wand.get_image_colorspace(),
        magick_rust::ColorspaceType::GRAY
    );

    let pixel_grayscale = wand.get_image_pixel_color(10, 10).unwrap();
    assert_eq!(pixel_grayscale.get_hsl().hue, 0.0);

    /* The output of `export_image_pixels` should match
     * `convert -type Grayscale tests/fixtures/IMG_5745.JPG[2x2+0+0] txt:` */
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
    IMG_5745_JPG.read_image(&wand);
    // There is a slight degree of variability from platform to platform,
    // and version to version of ImageMagick.
    let image_colors = wand.get_image_colors();
    assert!(image_colors > 38000 || image_colors < 40000);

    assert!(
        wand.quantize_image(
            6,
            magick_rust::ColorspaceType::RGB,
            1,
            magick_rust::DitherMethod::Undefined,
            false
        )
        .is_ok()
    );
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
    RUST_PNG.read_image(&wand);
    let mut pw = PixelWand::new();
    pw.set_color("#0000FF").unwrap();
    wand.set_image_background_color(&pw).unwrap();
    wand.set_image_alpha_channel(magick_rust::AlphaChannelOption::Remove)
        .unwrap();
    let blob = wand.write_image_blob("rgb").unwrap();
    assert_eq!(0u8, blob[0]);
    assert_eq!(0u8, blob[1]);
    assert_eq!(255u8, blob[2]);
}

#[test]
fn test_get_image_channel_range() {
    START.call_once(|| {
        magick_wand_genesis();
    });
    // A solid pure-red image: the red channel is fully saturated everywhere,
    // while green and blue are zero everywhere. This makes the per-channel
    // selection unambiguous.
    let mut wand = MagickWand::new();
    let mut background = PixelWand::new();
    background.set_color("red").unwrap();
    wand.new_image(8, 8, &background).unwrap();

    let red = wand
        .get_image_channel_range(magick_rust::ChannelType::Red)
        .unwrap();
    let green = wand
        .get_image_channel_range(magick_rust::ChannelType::Green)
        .unwrap();
    let blue = wand
        .get_image_channel_range(magick_rust::ChannelType::Blue)
        .unwrap();

    // Solid color, so minima == maxima within each channel.
    assert_eq!(red.0, red.1);
    assert!(red.1 > 0.0, "red channel should be saturated");
    assert_eq!((0.0, 0.0), green);
    assert_eq!((0.0, 0.0), blue);

    // The mask must be restored afterwards: an unrestricted range spans every
    // channel, so its maxima matches the fully-saturated red channel.
    let all = wand.get_image_range().unwrap();
    assert_eq!(red.1, all.1);
}

#[test]
#[cfg(any(target_os = "linux", target_os = "macos"))]
fn test_resource_limit_round_trip() {
    use magick_rust::ResourceType;
    START.call_once(|| {
        magick_wand_genesis();
    });
    // Use the Memory resource for a deterministic round trip: unlike Thread
    // (which ImageMagick clamps to the OpenMP-compiled maximum), the memory
    // limit is stored verbatim. Save and restore it so other tests are
    // unaffected, since resource limits are process-global.
    let original = MagickWand::get_resource_limit(ResourceType::Memory);
    MagickWand::set_resource_limit(ResourceType::Memory, 256 * 1024 * 1024).unwrap();
    assert_eq!(
        256 * 1024 * 1024,
        MagickWand::get_resource_limit(ResourceType::Memory)
    );
    MagickWand::set_resource_limit(ResourceType::Memory, original).unwrap();
    assert_eq!(
        original,
        MagickWand::get_resource_limit(ResourceType::Memory)
    );

    // The thread limit (the original request in issue #79) is at least one.
    assert!(MagickWand::get_resource_limit(ResourceType::Thread) >= 1);
}

#[test]
fn test_drawing_primitives() {
    START.call_once(|| {
        magick_wand_genesis();
    });
    let mut wand = MagickWand::new();
    let mut white = PixelWand::new();
    white.set_color("white").unwrap();
    wand.new_image(100, 100, &white).unwrap();

    // Fill a rounded rectangle over the center of the image with red.
    let mut draw = magick_rust::DrawingWand::new();
    let mut red = PixelWand::new();
    red.set_color("red").unwrap();
    draw.set_fill_color(&red);
    draw.draw_round_rectangle(20.0, 20.0, 80.0, 80.0, 10.0, 10.0);
    wand.draw_image(&draw).unwrap();

    // The center is now red...
    let center = wand.get_image_pixel_color(50, 50).unwrap();
    assert!(center.get_red() > 0.5 && center.get_green() < 0.5 && center.get_blue() < 0.5);
    // ...while a corner, outside the rounded rectangle, remains white.
    let corner = wand.get_image_pixel_color(2, 2).unwrap();
    assert!(corner.get_red() > 0.9 && corner.get_green() > 0.9 && corner.get_blue() > 0.9);
}

#[test]
fn test_floodfill_paint_image() {
    START.call_once(|| {
        magick_wand_genesis();
    });
    // A white image with a red square in the middle. Flood-filling the white
    // background from a corner with a transparent color is the "remove the
    // background" operation; the red foreground must be left untouched.
    let mut wand = MagickWand::new();
    let mut white = PixelWand::new();
    white.set_color("white").unwrap();
    wand.new_image(10, 10, &white).unwrap();
    wand.set_image_alpha_channel(magick_rust::AlphaChannelOption::Activate)
        .unwrap();

    let mut red = PixelWand::new();
    red.set_color("red").unwrap();
    let mut draw = magick_rust::DrawingWand::new();
    draw.set_fill_color(&red);
    draw.draw_rectangle(3.0, 3.0, 6.0, 6.0);
    wand.draw_image(&draw).unwrap();

    let mut transparent = PixelWand::new();
    transparent.set_color("none").unwrap();
    wand.floodfill_paint_image(&transparent, 0.0, &white, 0, 0, false)
        .unwrap();

    // The flooded corner is now transparent...
    let corner = wand.get_image_pixel_color(0, 0).unwrap();
    assert_eq!(0.0, corner.get_alpha());
    // ...while the red foreground square remains fully opaque.
    let center = wand.get_image_pixel_color(4, 4).unwrap();
    assert_eq!(1.0, center.get_alpha());
    assert!(center.get_red() > 0.5 && center.get_green() < 0.5 && center.get_blue() < 0.5);
}

#[test]
fn test_transparent_paint_image() {
    START.call_once(|| {
        magick_wand_genesis();
    });
    // A white image with a red square in the middle. Painting white as
    // transparent is the "-transparent white" operation; unlike a floodfill it
    // is not connectivity-bound, so every white pixel disappears while the red
    // foreground stays opaque.
    let mut wand = MagickWand::new();
    let mut white = PixelWand::new();
    white.set_color("white").unwrap();
    wand.new_image(10, 10, &white).unwrap();
    wand.set_image_alpha_channel(magick_rust::AlphaChannelOption::Activate)
        .unwrap();

    let mut red = PixelWand::new();
    red.set_color("red").unwrap();
    let mut draw = magick_rust::DrawingWand::new();
    draw.set_fill_color(&red);
    draw.draw_rectangle(3.0, 3.0, 6.0, 6.0);
    wand.draw_image(&draw).unwrap();

    // alpha == 0.0 makes the matched (white) pixels fully transparent.
    wand.transparent_paint_image(&white, 0.0, 0.0, false).unwrap();

    // Every white pixel is now transparent...
    let corner = wand.get_image_pixel_color(0, 0).unwrap();
    assert_eq!(0.0, corner.get_alpha());
    // ...while the red foreground square remains fully opaque.
    let center = wand.get_image_pixel_color(4, 4).unwrap();
    assert_eq!(1.0, center.get_alpha());
    assert!(center.get_red() > 0.5 && center.get_green() < 0.5 && center.get_blue() < 0.5);
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
    RUST_SVG.read_image(&wand);
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
    IMG_5745_JPG.read_image(&wand);

    let mut gradient = MagickWand::new();
    assert!(gradient.set_size(128, 20).is_ok());
    assert!(gradient.set_option("gradient:angle", "90").is_ok());
    assert!(gradient.read_image("gradient:black-yellow").is_ok());

    assert!(
        wand.clut_image(&gradient, magick_rust::PixelInterpolateMethod::Bilinear)
            .is_ok()
    );
}

#[test]
fn test_negate_image() {
    START.call_once(|| {
        magick_wand_genesis();
    });
    let wand = MagickWand::new();
    RUST_PNG.read_image(&wand);
    wand.negate_image().unwrap();
    let pixel_color = wand.get_image_pixel_color(0, 0).unwrap();
    assert_eq!(
        "srgb(255,255,255)",
        pixel_color.get_color_as_string().unwrap()
    );
}

#[test]
// opt-in platforms that have resource limits support
#[cfg(any(target_os = "linux", target_os = "macos"))]
fn test_resource_limits() {
    use magick_rust::ResourceType;
    START.call_once(|| {
        magick_wand_genesis();
    });
    MagickWand::set_resource_limit(ResourceType::Thread, 1).unwrap();
    let wand = MagickWand::new();
    RUST_PNG.read_image(&wand);
}

#[test]
fn test_auto_level() {
    START.call_once(|| {
        magick_wand_genesis();
    });
    let wand = MagickWand::new();
    IMG_5745_JPG.read_image(&wand);
    assert!(wand.auto_level().is_ok());
}

#[test]
fn test_auto_gamma() {
    START.call_once(|| {
        magick_wand_genesis();
    });
    let wand = MagickWand::new();
    IMG_5745_JPG.read_image(&wand);
    assert!(wand.auto_gamma().is_ok());
}

#[test]
fn test_image_compose() {
    START.call_once(|| {
        magick_wand_genesis();
    });
    let mut wand = MagickWand::new();
    wand.new_image(4, 4, &PixelWand::new()).unwrap();

    let operators = [
        magick_rust::CompositeOperator::Alpha,
        magick_rust::CompositeOperator::MinusDst,
        magick_rust::CompositeOperator::Over,
        magick_rust::CompositeOperator::Xor,
        magick_rust::CompositeOperator::Bumpmap,
        magick_rust::CompositeOperator::ChangeMask,
        magick_rust::CompositeOperator::Clear,
        magick_rust::CompositeOperator::ColorBurn,
        magick_rust::CompositeOperator::ColorDodge,
        magick_rust::CompositeOperator::Colorize,
        magick_rust::CompositeOperator::CopyBlack,
        magick_rust::CompositeOperator::CopyBlue,
        magick_rust::CompositeOperator::Copy,
        magick_rust::CompositeOperator::CopyCyan,
        magick_rust::CompositeOperator::CopyGreen,
        magick_rust::CompositeOperator::CopyMagenta,
        magick_rust::CompositeOperator::CopyAlpha,
        magick_rust::CompositeOperator::CopyRed,
        magick_rust::CompositeOperator::CopyYellow,
        magick_rust::CompositeOperator::Darken,
        magick_rust::CompositeOperator::DarkenIntensity,
        magick_rust::CompositeOperator::Difference,
        magick_rust::CompositeOperator::Displace,
        magick_rust::CompositeOperator::Dissolve,
        magick_rust::CompositeOperator::Distort,
        magick_rust::CompositeOperator::DivideDst,
    ];
    for op in operators.iter() {
        wand.set_image_compose(*op).unwrap();
        assert_eq!(*op, wand.get_image_compose());
    }
}

#[test]
fn test_import_export_pixels_roundtrip() {
    START.call_once(|| {
        magick_wand_genesis();
    });
    let w = 2;
    let h = 2;
    let map = "RGB";
    let pixels = [0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255];
    let mut wand = MagickWand::new();
    wand.new_image(4, 4, &PixelWand::new()).unwrap();
    assert!(wand.import_image_pixels(0, 0, w, h, &pixels, map).is_ok());
    let exported_pixels = wand.export_image_pixels(0, 0, w, h, map).unwrap();
    assert_eq!(exported_pixels.len(), pixels.len());
    assert!(
        exported_pixels
            .iter()
            .zip(pixels.iter())
            .all(|(a, b)| a == b)
    );
}

#[test]
fn test_image_list_frame_access() {
    START.call_once(|| {
        magick_wand_genesis();
    });
    // rust.gif is a two-frame animation: frame 0 is the full 156x150 canvas,
    // frame 1 is an optimized 80x76 sub-frame.
    let wand = MagickWand::new();
    RUST_GIF.read_image(&wand);

    let images = wand.images();
    assert_eq!(2, images.count());
    assert!(!images.is_empty());

    let first = images.first().expect("first frame");
    assert_eq!(
        (156, 150),
        (first.get_image_width(), first.get_image_height())
    );

    let last = images.last().expect("last frame");
    assert_eq!((80, 76), (last.get_image_width(), last.get_image_height()));

    // Two frame handles can be held at once and each still reports its own
    // frame's dimensions: every getter re-pins the iterator before reading.
    assert_eq!(156, first.get_image_width());
    assert_eq!(80, last.get_image_width());
    assert_eq!(156, first.get_image_width());

    // Out-of-bounds access yields None.
    assert!(images.get(2).is_none());

    // for_each visits every frame in order.
    let mut seen = Vec::new();
    images.for_each(|index, frame| seen.push((index, frame.get_image_width())));
    assert_eq!(vec![(0, 156), (1, 80)], seen);
}

#[test]
fn test_image_list_coalesce_and_draw() {
    START.call_once(|| {
        magick_wand_genesis();
    });
    let mut wand = MagickWand::new();
    RUST_GIF.read_image(&wand);

    // Coalescing expands every frame to the full canvas, so both frames become
    // 156x150 and can be edited uniformly.
    let mut coalesced = wand.coalesce().unwrap();
    {
        let images = coalesced.images();
        images.for_each(|_, frame| {
            assert_eq!(
                (156, 150),
                (frame.get_image_width(), frame.get_image_height())
            );
        });
    }

    // Paint a red square onto every frame via mutable frame borrows.
    let mut red = PixelWand::new();
    red.set_color("red").unwrap();
    let mut draw = magick_rust::DrawingWand::new();
    draw.set_fill_color(&red);
    draw.draw_rectangle(10.0, 10.0, 40.0, 40.0);

    coalesced
        .images_mut()
        .try_for_each(|_, mut frame| frame.draw_image(&draw))
        .unwrap();

    // The square is present on each frame independently.
    let images = coalesced.images();
    images.for_each(|_, frame| {
        let pixel = frame.get_image_pixel_color(25, 25).unwrap();
        assert!(pixel.get_red() > 0.5 && pixel.get_green() < 0.5 && pixel.get_blue() < 0.5);
    });
}

#[test]
fn test_image_list_remove() {
    START.call_once(|| {
        magick_wand_genesis();
    });
    let mut wand = MagickWand::new();
    RUST_GIF.read_image(&wand);

    assert_eq!(2, wand.images().count());
    wand.images_mut().remove(0).unwrap();
    assert_eq!(1, wand.images().count());

    // Removing past the end is an error rather than a panic.
    assert!(wand.images_mut().remove(5).is_err());
}
