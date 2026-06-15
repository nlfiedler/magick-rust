//
// Copyright (c) 2026 Nathan Fiedler
//
// Replicates the "trim by a blurred copy" technique from
// https://legacy.imagemagick.org/Usage/crop/#trim_blur, i.e. the command line:
//
//     magick in.png -crop \
//         `magick in.png -virtual-pixel edge -blur 0x15 -fuzz 15% \
//                  -trim -format '%wx%h%O' info:`   +repage   out.png
//
// The inner command blurs a *copy* of the image so a noisy border becomes a
// uniform color, trims that copy, and reports the trimmed geometry. The outer
// command then crops the *original* (un-blurred) image to that geometry.
//
// The key gotcha: `MagickWand::trim_image` takes its fuzz in raw quantum units
// (0..=QuantumRange), not as a fraction. `-fuzz 15%` therefore becomes
// `0.15 * QuantumRange`, not `0.15`.
//
use magick_rust::{MagickError, MagickWand, VirtualPixelMethod, magick_wand_genesis};
use std::sync::Once;

static START: Once = Once::new();

// QuantumRange for a Q16 build of ImageMagick. For a Q8 build this is 255.0.
const QUANTUM_RANGE: f64 = 65535.0;

fn trim_blur_crop(input: &str, output: &str) -> Result<(), MagickError> {
    START.call_once(|| {
        magick_wand_genesis();
    });

    // The image we will ultimately crop and save.
    let wand = MagickWand::new();
    wand.read_image(input)?;

    // A throwaway blurred copy used only to measure where the border ends.
    let mut probe = wand.clone();
    probe.set_image_virtual_pixel_method(VirtualPixelMethod::Edge); // -virtual-pixel edge
    probe.blur_image(0.0, 15.0)?; // -blur 0x15
    probe.trim_image(0.15 * QUANTUM_RANGE)?; // -fuzz 15% -trim

    // `%wx%h%O`: the trimmed size plus its offset within the original image.
    // After a trim the offset lives in the image page geometry.
    let width = probe.get_image_width();
    let height = probe.get_image_height();
    let (_pw, _ph, x, y) = probe.get_image_page();

    // Crop the original to the geometry measured from the blurred copy, then
    // reset the page (the `+repage` in the command line).
    wand.crop_image(width, height, x, y)?;
    wand.reset_image_page("0x0+0+0")?;

    wand.write_image(output)
}

fn main() {
    match trim_blur_crop("input.png", "output.png") {
        Ok(()) => println!("wrote output.png"),
        Err(err) => println!("error: {err}"),
    }
}
