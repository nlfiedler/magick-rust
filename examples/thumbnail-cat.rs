//
// Copyright (c) 2024 Nathan Fiedler
//
extern crate magick_rust;
use magick_rust::{magick_wand_genesis, MagickError, MagickWand};
use std::fs;
use std::sync::Once;

// Used to make sure MagickWand is initialized exactly once. Note that we do not
// bother shutting down, we simply exit when we're done.
static START: Once = Once::new();

// Read the named file and create a thumbnail bound by a rectangle that is 240
// by 240 pixels (for snow-covered-cat.jpg it will be 240x191 pixels).
fn resize(filepath: &str) -> Result<Vec<u8>, MagickError> {
    START.call_once(|| {
        magick_wand_genesis();
    });
    let wand = MagickWand::new();
    wand.read_image(filepath)?;
    wand.fit(240, 240);
    wand.write_image_blob("jpeg")
}

fn main() {
    match resize("tests/fixtures/snow-covered-cat.jpg") {
        Ok(bytes) => {
            fs::write("thumbnail-cat.jpg", bytes).expect("write failed");
        }
        Err(err) => println!("error: {err}"),
    }
}
