extern crate gdl;
use gdl::elements::*;
use gdl::text::*;
use gdl::{new_with_background, Rgb};
use std::time::Instant;

fn main() {
    let start = Instant::now();

    // Create black and white Rgb colours.
    let white = Rgb {
        r: 255,
        g: 255,
        b: 255,
    };
    let black = Rgb { r: 0, g: 0, b: 0 };
    let _rgb3 = Rgb {
        r: 200,
        g: 180,
        b: 50,
    };

    let width = 1000;
    let height = 500;

    // Create a new image with a background color (a PhotonImage is returned)
    let mut img = new_with_background(width, height, &black);

    // Draw gradients
    draw_preset_rect_gradient(&mut img, 300, 300, 30, 30, "pinkblue");
    draw_preset_rect_gradient(&mut img, 300, 300, 330, 30, "pink_pastel");
    draw_preset_rect_gradient(&mut img, 300, 300, 630, 30, "lemongrass");

    // Draw text
    draw_text(&mut img, "pink_blue", 90, 350, "Roboto-Light", 60.0, &white);
    draw_text(
        &mut img,
        "pink_pastel",
        350,
        350,
        "Roboto-Light",
        60.0,
        &white,
    );
    draw_text(
        &mut img,
        "lemongrass",
        650,
        350,
        "Roboto-Light",
        60.0,
        &white,
    );

    // Write the contents of this image in PNG format.
    gdl::helpers::save_image(img, "example_output/gradients.png");

    println!(
        "Took {} seconds to create image.",
        start.elapsed().as_secs()
    );
    println!("You'll find the output image in examples/example_output");
}
