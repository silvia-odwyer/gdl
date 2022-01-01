extern crate gdl;
use gdl::diagrams::Chart;
use gdl::{diagrams, new_with_background, Rgb};
use std::time::Instant;

fn main() {
    let start = Instant::now();

    let _white = Rgb {
        r: 255,
        g: 255,
        b: 255,
    };
    let black = Rgb { r: 0, g: 0, b: 0 };
    let mut img = new_with_background(1500, 1500, &black);

    // Insert the data into a vec
    let data: Vec<u16> = vec![5, 10, 20, 40];

    // Create labels for the barchart
    let labels: Vec<String> = vec![
        "one".to_string(),
        "c_spec".to_string(),
        "d_spec".to_string(),
        "e_spec".to_string(),
    ];

    // Barchart bar color
    let blue = Rgb {
        r: 40,
        g: 50,
        b: 200,
    };

    // Create a barchart struct
    let barchart = Chart::new(
        "Earnings for 2019/2020".to_string(),
        blue,
        data,
        labels,
        1500,
        1500,
    );

    diagrams::draw_vertical_gradient_barchart(&mut img, &barchart, "lemongrass");
    gdl::helpers::save_image(img, "example_output/barchart.png");

    println!(
        "Took {} seconds to create image.",
        start.elapsed().as_secs()
    );
    println!("You'll find the output image in examples/example_output");
}
