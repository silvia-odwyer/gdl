/// Create diagrams, flowcharts, graphs, etc.,

use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use crate::text::draw_text;
use crate::elements::draw_solid_rect;
use crate::Rgb;
use crate::Barchart;
use image::DynamicImage;
use crate::helpers;
use std::cmp::max;
use crate::elements::draw_preset_rect_gradient;
/// Draw a flowchart.
#[wasm_bindgen]
pub fn draw_flowchart(mut img: &mut DynamicImage, item1: &str) {
    let rgb = Rgb{ r: 255, g: 255, b: 255 };
    let start_x: i32 = 20;
    let start_y: i32 = 20;
    let width: i32 = 130;
    let height: i32 = 100;
    draw_solid_rect(img, &rgb, height as u32, width as u32, start_x, start_y);      
    draw_solid_rect(img, &rgb, height as u32, width as u32, start_x + width + 50, start_y);        
}

/// Draw a barchart, with a specified title and data.
pub fn draw_horizontal_barchart(mut img: &mut DynamicImage, barchart: Barchart) {

    let start_x: i32 = 20;
    let mut start_y: i32 = 20;

    let max_item = barchart.data.iter().max().unwrap();
    let mut max_bar_width: u32 = barchart.width - 2 * (barchart.width / 10);
    let num_bars: u32 = barchart.data.len() as u32;
    let bar_height: u32 = ((barchart.height / num_bars) as f32 * 0.8) as u32;

    println!("bar height is {} max bar width is {}", bar_height, max_bar_width);
    let lilac = Rgb{r: 204, g: 195, b: 240};
    let yellow = Rgb{ r: 255, g: 226, b: 98};

    for item in &barchart.data {
        let div =  max_item / item;
        let bar_width = max_bar_width / div as u32;
        draw_solid_rect(img, &barchart.color, bar_width as u32, bar_height as u32, start_x, start_y);
        start_y += (bar_height) as i32 + 30;    
    }    
    let rgb_white = Rgb { r: 255, g: 255, b: 255};

    draw_text(img, &barchart.title, 10, start_y as u32, "Lato-Regular", 50.0, &yellow);
}

/// Draw a vertical barchart, with a specified title and data.
pub fn draw_vertical_barchart(mut img: &mut DynamicImage, barchart: Barchart) {

    let mut start_x: i32 = 20;
    let mut start_y: i32 = (barchart.height - 40) as i32;

    let max_item = barchart.data.iter().max().unwrap();
    let mut max_bar_height: u32 = barchart.height - 2 * (barchart.height / 10);
    let num_bars: u32 = barchart.data.len() as u32;
    let bar_width: u32 = ((barchart.height / num_bars) as f32 * 0.8) as u32;

    println!("bar height is {} max bar width is {}", bar_width, max_bar_height);
    let lilac = Rgb{r: 204, g: 195, b: 240};
    let yellow = Rgb{ r: 255, g: 226, b: 98};

    draw_vertical_bars(img, &barchart, max_item, max_bar_height, bar_width);
    let rgb_white = Rgb { r: 255, g: 255, b: 255};

    draw_text(img, &barchart.title, 10, start_y as u32, "Lato-Regular", 50.0, &yellow);
}

/// Draw a vertical barchart, with a specified title and data.
pub fn draw_vertical_gradient_barchart(mut img: &mut DynamicImage, barchart: Barchart, preset: &str) {

    let mut start_x: u32 = 20;
    let mut start_y: u32 = (barchart.height - 40);

    let max_item = barchart.data.iter().max().unwrap();
    let mut max_bar_height: u32 = barchart.height - 2 * (barchart.height / 10);
    let num_bars: u32 = barchart.data.len() as u32;
    let bar_width: u32 = ((barchart.height / num_bars) as f32 * 0.8) as u32;

    println!("bar height is {} max bar width is {}", bar_width, max_bar_height);
    let lilac = Rgb{r: 204, g: 195, b: 240};
    let yellow = Rgb{ r: 255, g: 226, b: 98};

    for item in &barchart.data {
        let div =  max_item / item;
        let bar_height = max_bar_height / div as u32;
        draw_preset_rect_gradient(img, bar_width as u32, bar_height as u32, start_x, start_y - bar_height, preset);

        start_x += bar_width + 30;    
    }    
    let rgb_white = Rgb { r: 255, g: 255, b: 255};

    draw_text(img, &barchart.title, 10, start_y as u32, "Lato-Regular", 50.0, &yellow);
}

fn draw_vertical_bars(mut img: &mut DynamicImage, barchart: &Barchart, max_item: &u16, max_bar_height: u32, bar_width: u32) {
    
    let mut start_x: u32 = 20;
    let mut start_y: u32 = (barchart.height - 40);
    
    for item in &barchart.data {
        let div =  max_item / item;
        let bar_height = max_bar_height / div as u32;
        draw_solid_rect(img, &barchart.color, bar_width as u32, bar_height as u32, start_x as i32, (start_y - bar_height) as i32);
        start_x += bar_width + 30;    
    }
}

/// Draw a vertical barchart, with a specified title and data.
pub fn draw_horizontal_gradient_barchart(mut img: &mut DynamicImage, barchart: Barchart, preset: &str) {

    let start_x: u32 = 20;
    let mut start_y: u32 = 20;

    let max_item = barchart.data.iter().max().unwrap();
    let mut max_bar_width: u32 = barchart.width - 2 * (barchart.width / 10);
    let num_bars: u32 = barchart.data.len() as u32;
    let bar_height: u32 = ((barchart.height / num_bars) as f32 * 0.8) as u32;

    println!("bar height is {} max bar width is {}", bar_height, max_bar_width);
    let lilac = Rgb{r: 204, g: 195, b: 240};
    let yellow = Rgb{ r: 255, g: 226, b: 98};

    for item in &barchart.data {
        let div =  max_item / item;
        let bar_width = max_bar_width / div as u32;
        draw_preset_rect_gradient(img, bar_width as u32, bar_height as u32, start_x, start_y - bar_height, preset);
        start_y += bar_height + 30;    
    }    
    let rgb_white = Rgb { r: 255, g: 255, b: 255};

    draw_text(img, &barchart.title, 10, start_y as u32, "Lato-Regular", 50.0, &yellow);
}




/// Draw a histogram with a specified title, and data.
#[wasm_bindgen]
pub fn draw_histogram(mut img: &mut DynamicImage, title: &str, height: u32, width: u32) {
    let rgb = Rgb{ r: 255, g: 255, b: 255 };
    let start_x: i32 = 20;
    let mut start_y: i32 = 20;
    let mut bar_width: u32 = width - 2*(width / 10);
    let num_bars = 5;
    let bar_height: u32 = height / num_bars;
    let lilac = Rgb{r: 204, g: 195, b: 240};
    let yellow = Rgb{ r: 255, g: 226, b: 98};

    for _ in 0..num_bars {
        draw_solid_rect(img, &lilac, bar_height as u32, bar_width as u32, start_x, start_y);
        start_y += (bar_height) as i32;      
        bar_width -= 20;
    }    
    let rgb_white = Rgb { r: 255, g: 255, b: 255};
    draw_text(img, title, 10, start_y as u32, "Lato-Regular", 50.0, &rgb_white);
}