extern crate gdl;
extern crate time;
use gdl::{Rgb, new_with_background};
use gdl::text::*;
use time::PreciseTime;

fn main() {
    let start = PreciseTime::now();

    // Create black and white Rgb colours.
    let white = Rgb { r: 255, g: 255, b: 255};
    let black = Rgb {r: 0, g: 0, b: 0};

    // Create a new image with a background color (a PhotonImage is returned)
    let mut img = new_with_background(1000, 1000, &black);

    // Draw text by passing in a mutable ref to the PhotonImage created above.

    // A for loop could be used to avoid code repeation, of course, but this was omitted
    // to maintain simplicity and showcase the fundamentals of GDL. 
    draw_text(&mut img, "Roboto Black", 20, 30, "Roboto-Black", 110.0, &white);

    draw_text(&mut img, "Roboto Bold", 20, 120, "Roboto-Bold", 90.0, &white);

    draw_text(&mut img, "Roboto Regular", 20, 210, "Roboto-Regular", 80.0, &white);
    
    draw_text(&mut img, "Roboto Light", 20, 300, "Roboto-Light", 70.0, &white);

    draw_text(&mut img, "Bebas Kai", 20, 390, "BebasKai", 60.0, &white);

    // Write the contents of this image in PNG format.
    gdl::helpers::save_image(img, "example_output/text_output.png");

    let end = PreciseTime::now();
    println!("Took {} seconds to create image.", start.to(end));
    println!("You'll find the output image in examples/example_output");
}