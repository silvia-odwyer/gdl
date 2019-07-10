extern crate gdl;
extern crate time;
use gdl::{Rgb, PhotonImage, new_with_background};
use gdl::text::*;
use gdl::elements::*;
use time::PreciseTime;

fn main() {
    let start = PreciseTime::now();

    // Create black and white Rgb colours.
    let white = Rgb { r: 255, g: 255, b: 255};
    let black = Rgb {r: 0, g: 0, b: 0};
    let rgb3 = Rgb {r: 200, g: 180, b: 50};

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
    draw_text(&mut img, "pink_pastel", 350, 350, "Roboto-Light", 60.0, &white);
    draw_text(&mut img, "lemongrass", 650, 350, "Roboto-Light", 60.0, &white);


    // Write the contents of this image in PNG format.
    gdl::helpers::save_image(img, "example_output/gradients.png");

    let end = PreciseTime::now();
    println!("Took {} seconds to create image.", start.to(end));
    println!("You'll find the output image in examples/example_output");
}