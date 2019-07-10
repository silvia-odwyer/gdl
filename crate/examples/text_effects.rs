extern crate gdl;
extern crate time;
use gdl::{Rgb, PhotonImage, new_with_background};
use gdl::text::*;
use time::PreciseTime;

fn main() {
    let start = PreciseTime::now();

    // Create black and white Rgb colours.
    let _white = Rgb { r: 255, g: 255, b: 255};
    let black = Rgb {r: 0, g: 0, b: 0};
    let rgb3 = Rgb {r: 200, g: 180, b: 50};

    let width = 800;
    let height = 800;
    // Create a new image with a background color (a PhotonImage is returned)
    let mut img = new_with_background(width, height, &black);

    // Draw text by passing in a mutable ref to the PhotonImage created above.

    draw_vertical_text(&mut img, "Roboto Bold", 50, 200, "Roboto-Bold", 90.0, "left", &rgb3);

    draw_vertical_text(&mut img, "Roboto Regular", width - 150, 220, "Roboto-Regular", 90.0, "right", &rgb3);

    draw_upsidedown_text(&mut img, "Break The Rules.", 10, 120, "Roboto Bold", 90.0, &rgb3);

    draw_vertical_text_single(&mut img, "Roboto", width / 2, (height / 3 - 100), "Roboto-Regular", 90.0, &rgb3);
    draw_vertical_text_single(&mut img, "HELLO", (width / 2) + 100, (height / 3 - 100), "BebasKai", 90.0, &rgb3 );

    // Write the contents of this image in PNG format.
    img.save("example_output/text_effects.png").unwrap();

    let end = PreciseTime::now();
    println!("Took {} seconds to create image.", start.to(end));
    println!("You'll find the output image in examples/example_output");
}