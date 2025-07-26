extern crate magick_rust;
use magick_rust::{CompositeOperator, MagickError, MagickWand, PixelWand, magick_wand_genesis};
use std::fs;
use std::sync::Once;

// Used to make sure MagickWand is initialized exactly once. Note that we do not
// bother shutting down, we simply exit when we're done.
static START: Once = Once::new();

// Read the named file and add a 10 pixel border around the image
fn add_border(filepath: &str, border_color: &str) -> Result<Vec<u8>, MagickError> {
    START.call_once(|| {
        magick_wand_genesis();
    });

    let wand = MagickWand::new();
    wand.read_image(filepath)?;

    let mut border = PixelWand::new();
    border.set_color(border_color)?;

    wand.border_image(&border, 10, 10, CompositeOperator::Over)?;
    wand.write_image_blob("jpeg")
}

fn main() {
    match add_border("tests/fixtures/snow-covered-cat.jpg", "red") {
        Ok(bytes) => {
            fs::write("border-cat.jpg", bytes).expect("write failed");
        }
        Err(err) => println!("error: {err}"),
    }
}
