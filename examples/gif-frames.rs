//
// Copyright (c) 2026 Nathan Fiedler
//
// Demonstrates the image-list (frame) API for working with multi-image wands
// such as animated GIFs. We first synthesize a short animation, then use
// `images()` / `images_mut()` to inspect and edit individual frames:
//
//   - read frame dimensions through read-only `ImageRef` handles, and
//   - draw onto every frame through mutable `ImageMut` handles.
//
// This mirrors the workflow requested in GitHub issue #30.
//
use magick_rust::{DrawingWand, MagickError, MagickWand, PixelWand, magick_wand_genesis};
use std::fs;
use std::sync::Once;

static START: Once = Once::new();

const WIDTH: usize = 240;
const HEIGHT: usize = 120;
const FRAMES: usize = 6;

// Build a small animation whose frames cycle through the color wheel, then
// draw a marker onto each frame that scrolls across the image as it plays.
fn build_animation() -> Result<Vec<u8>, MagickError> {
    START.call_once(|| {
        magick_wand_genesis();
    });

    // Assemble the frames. Each frame is its own single-image wand that we
    // append onto the animation via the mutable image-list view.
    let mut wand = MagickWand::new();
    for i in 0..FRAMES {
        let mut frame = MagickWand::new();
        let mut background = PixelWand::new();
        background.set_color(&format!("hsl({}, 70%, 60%)", i * 360 / FRAMES))?;
        frame.new_image(WIDTH, HEIGHT, &background)?;
        frame.set_image_format("gif")?;
        // Frame delay is in 1/100ths of a second, so 25 == 250ms per frame.
        frame.set_image_delay(25)?;
        wand.images_mut().append(&frame)?;
    }

    // Read-only inspection of the list through `ImageRef` handles. Each getter
    // re-pins the wand's internal iterator, so the values always belong to the
    // frame the handle refers to, regardless of access order.
    {
        let images = wand.images();
        println!("animation has {} frame(s)", images.count());
        if let Some(first) = images.first() {
            println!(
                "  first frame: {}x{}",
                first.get_image_width(),
                first.get_image_height()
            );
        }
        if let Some(last) = images.last() {
            println!(
                "  last frame:  {}x{}, delay {} (1/100s)",
                last.get_image_width(),
                last.get_image_height(),
                last.get_image_delay()
            );
        }
    }

    // Mutable per-frame editing through `ImageMut` handles. The marker's
    // position advances with the frame index, so it scrolls across the image as
    // the animation plays. The closure must not add or remove frames; it only
    // edits the existing ones.
    let mut ink = PixelWand::new();
    ink.set_color("black")?;
    let radius = 16.0;
    wand.images_mut().try_for_each(|index, mut frame| {
        let mut draw = DrawingWand::new();
        draw.set_fill_color(&ink);
        let progress = (index + 1) as f64 / (FRAMES + 1) as f64;
        let cx = WIDTH as f64 * progress;
        let cy = HEIGHT as f64 * progress;
        // draw_circle takes the center and a point on the perimeter.
        draw.draw_circle(cx, cy, cx + radius, cy);
        frame.draw_image(&draw)
    })?;

    // `adjoin = true` writes every frame into a single animated GIF file.
    wand.write_images_blob("gif")
}

fn main() {
    match build_animation() {
        Ok(bytes) => {
            fs::write("gif-frames.gif", bytes).expect("write failed");
            println!("wrote gif-frames.gif");
        }
        Err(err) => println!("error: {err}"),
    }
}
