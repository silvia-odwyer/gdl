extern crate gdl;
extern crate time;
use gdl::{Rgb, PhotonImage, diagrams, new_with_background};
use time::PreciseTime;
use gdl::diagrams::{Chart};

fn main() {
    let start = PreciseTime::now();

    let beach = gdl::helpers::open_image("examples/input_images/beach.jpg");
    let white = Rgb { r: 255, g: 255, b: 255};
    let black = Rgb { r: 0, g: 0, b: 0};
    let mut img = new_with_background(1500, 1500, &black);
    
    let data: Vec<u16> = vec![0, 5, 40, 20, 30];
    let labels: Vec<String> = vec!["one".to_string(), "c_spec".to_string(), "d_spec".to_string(), "e_spec".to_string()];
    let blue = Rgb { r: 40, g: 50, b: 200};

    let barchart = Chart::new("Earnings for 2019/2020".to_string(), blue, data, labels, 1500, 1500);

    diagrams::draw_linechart(&mut img, &barchart);
    gdl::helpers::save_image(img, "postcard.png");

    let end = PreciseTime::now();
    println!("Took {} seconds to create image.", start.to(end));
    println!("You'll find the output image in examples/example_output");
}