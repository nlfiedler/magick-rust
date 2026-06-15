//
// Copyright (c) 2026 Nathan Fiedler
//
// Demonstrates the DrawingWand API: set fill/stroke properties on a
// DrawingWand, draw shapes onto it, then render it onto a MagickWand image with
// `draw_image`. This mirrors the C MagickWand "draw shapes" example:
// https://imagemagick.org/MagickWand/draw_shapes.htm
//
use magick_rust::{DrawingWand, MagickError, MagickWand, PixelWand, magick_wand_genesis};
use std::fs;
use std::sync::Once;

static START: Once = Once::new();

fn draw() -> Result<Vec<u8>, MagickError> {
    START.call_once(|| {
        magick_wand_genesis();
    });

    // A white canvas to draw on.
    let mut wand = MagickWand::new();
    let mut white = PixelWand::new();
    white.set_color("white")?;
    wand.new_image(300, 300, &white)?;

    // Configure how subsequent shapes are painted.
    let mut draw = DrawingWand::new();
    let mut stroke = PixelWand::new();
    stroke.set_color("black")?;
    draw.set_stroke_color(&stroke);
    draw.set_stroke_width(2.0);
    let mut fill = PixelWand::new();
    fill.set_color("red")?;
    draw.set_fill_color(&fill);

    // A handful of the now-available primitives.
    draw.draw_rectangle(20.0, 20.0, 120.0, 90.0);
    draw.draw_round_rectangle(150.0, 20.0, 280.0, 90.0, 20.0, 20.0);
    draw.draw_circle(70.0, 180.0, 70.0, 230.0);
    draw.draw_ellipse(220.0, 180.0, 60.0, 35.0, 0.0, 360.0);
    draw.draw_line(20.0, 270.0, 280.0, 270.0);
    draw.draw_polygon(&[(20.0, 260.0), (40.0, 240.0), (60.0, 260.0)]);

    // Nothing is drawn until the DrawingWand is rendered onto the image.
    wand.draw_image(&draw)?;
    wand.write_image_blob("png")
}

fn main() {
    match draw() {
        Ok(bytes) => {
            fs::write("draw-shapes.png", bytes).expect("write failed");
            println!("wrote draw-shapes.png");
        }
        Err(err) => println!("error: {err}"),
    }
}
