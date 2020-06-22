extern crate gdl;
extern crate time;
use gdl::{Rgb, PhotonImage, ColorScheme, diagrams, background, text};
use gdl::text::*;
use gdl::{presets, new_with_background};
use gdl::elements::*;
use time::PreciseTime;

// See the examples dir for more examples
fn main() {
    // GRAPHIC 1
    let start = PreciseTime::now();
    let yellow = Rgb { r: 255, g:226, b: 98};
    let mut img = new_with_background(800, 800, &yellow);

    presets::text_banner(&mut img, "The Lemonade Co.", "Making great lemonade since 2002.");
    gdl::helpers::save_image(img, "output_graphic1.png");

    let end = PreciseTime::now();
    println!("Took {} seconds to create Graphic 1.", start.to(end));

    // GRAPHIC 2 
    let mut alleyway = gdl::helpers::open_image("examples/input_images/city_square.jpg");
    let white = Rgb { r: 255, g: 255, b: 255};

    draw_text(&mut alleyway, "Visit", 230, 270, "BebasKai", 140.0, &white);
    draw_text(&mut alleyway, "Stockholm", 230, 390, "BebasKai", 140.0, &white);

    gdl::helpers::save_image(alleyway, "output_graphic2.png");

    let end = PreciseTime::now();
    println!("Took {} seconds to create Graphic 2.", start.to(end));

    // GRAPHIC 3 
    let mut night = gdl::helpers::open_image("examples/input_images/drive.jpg");
    let white = Rgb { r: 255, g: 255, b: 255};

    draw_vertical_text(&mut night, "BUCHAREST BY NIGHT", 0, 0, "BebasKai", 110.0, "right", &white);
    gdl::helpers::save_image(night, "output_graphic3.png");

    let end = PreciseTime::now();
    println!("Took {} seconds to create Graphic 3.", start.to(end));


}