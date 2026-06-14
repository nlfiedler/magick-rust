//
// Copyright (c) 2026 Nathan Fiedler
//
// Replicates the ImageMagick command line:
//
//     magick -density 300 test.pdf -background white -alpha remove -alpha off output_page_%d.png
//
// The key detail is that the read resolution ("density") must be set on the wand
// *before* reading the PDF, since that is what controls how Ghostscript
// rasterizes the vector pages into pixels.
//
use magick_rust::{
    AlphaChannelOption, MagickError, MagickWand, PixelWand, magick_wand_genesis,
};
use std::sync::Once;

static START: Once = Once::new();

fn pdf_to_png(filepath: &str, output_prefix: &str) -> Result<(), MagickError> {
    START.call_once(|| {
        magick_wand_genesis();
    });
    let wand = MagickWand::new();

    // `-density 300`: set the resolution BEFORE reading so the PDF is
    // rasterized at 300 DPI rather than the default (typically 72 DPI).
    wand.set_resolution(300.0, 300.0)?;

    // Reads every page of the PDF into the wand as a separate image.
    wand.read_image(filepath)?;

    // `-background white`
    let mut background = PixelWand::new();
    background.set_color("white")?;

    // The alpha operations act on the current image in the wand's iterator, so
    // iterate over every page and apply them individually.
    wand.set_first_iterator();
    while wand.next_image() {
        wand.set_image_background_color(&background)?;
        // `-alpha remove`: flatten transparency against the background color.
        wand.set_image_alpha_channel(AlphaChannelOption::Remove)?;
        // `-alpha off`: disable the alpha channel entirely.
        wand.set_image_alpha_channel(AlphaChannelOption::Off)?;
    }

    // Write each page to its own file. `output_page_%d.png` expands to
    // output_page_0.png, output_page_1.png, and so on. The `adjoin = false`
    // argument tells ImageMagick to write one file per image.
    wand.write_images(&format!("{output_prefix}_%d.png"), false)
}

fn main() {
    match pdf_to_png("test.pdf", "output_page") {
        Ok(()) => println!("wrote output_page_*.png"),
        Err(err) => println!("error: {err}"),
    }
}
