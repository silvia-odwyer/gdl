use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, ImageData, HtmlCanvasElement};
use wasm_bindgen::Clamped;
use image::{DynamicImage, GenericImageView, GenericImage, ImageBuffer, Rgba, RgbaImage};
use imageproc::drawing::draw_text_mut;
use imageproc::morphology::dilate_mut;
use rusttype::Scale;
use rusttype::{FontCollection};
use imageproc::distance_transform::Norm;
use imageproc::drawing::draw_filled_rect_mut;
use imageproc::rect::Rect;
use palette::{Lch, Srgb, Srgba, Hue, Gradient};
use palette::rgb::LinSrgba;
use palette::encoding::pixel::Pixel;

pub fn draw_flowchart(&mut img: PhotonImage, item1: &str) {
        let mut image = helpers::dyn_image_from_raw(&img).to_rgba();
        let rgb = Rgb{ r: 255, g: 255, b: 255 };
        let START_X: i32 = 20;
        let START_Y: i32 = 20;
        let width: i32 = 130;
        let height: i32 = 100;
        img.draw_solid_rect(&rgb, height as u32, width as u32, START_X, START_Y);      
        img.draw_solid_rect(&rgb, height as u32, width as u32, START_X + width + 50, START_Y);        

}

pub fn draw_barchart(&mut img: PhotonImage, title: &str, height: u32, width: u32) {
    let mut image = helpers::dyn_image_from_raw(&img).to_rgba();
    let rgb = Rgb{ r: 255, g: 255, b: 255 };
    let START_X: i32 = 20;
    let mut START_Y: i32 = 20;
    let mut bar_width: u32 = width - 2*(width / 10);
    let num_bars = 5;
    let bar_height: u32 = height / num_bars;
    let lilac = Rgb{r: 204, g: 195, b: 240};
    let yellow = Rgb{ r: 255, g: 226, b: 98};

    for _ in 0..num_bars {
        img.draw_solid_rect(&yellow, bar_height as u32, bar_width as u32, START_X, START_Y);
        START_Y += (bar_height + 20) as i32;      
        bar_width -= 20;
    }    

    img.draw_text(title, 10, START_Y as u32, "Lato-Regular", 50.0);
}
    
pub fn draw_histogram(&mut img, title: &str, height: u32, width: u32) {
    let mut image = helpers::dyn_image_from_raw(&img).to_rgba();
    let rgb = Rgb{ r: 255, g: 255, b: 255 };
    let START_X: i32 = 20;
    let mut START_Y: i32 = 20;
    let mut bar_width: u32 = width - 2*(width / 10);
    let num_bars = 5;
    let bar_height: u32 = height / num_bars;
    let lilac = Rgb{r: 204, g: 195, b: 240};
    let yellow = Rgb{ r: 255, g: 226, b: 98};

    for _ in 0..num_bars {
        img.draw_solid_rect(&lilac, bar_height as u32, bar_width as u32, START_X, START_Y);
        START_Y += (bar_height) as i32;      
        bar_width -= 20;
    }    

    img.draw_text(title, 10, START_Y as u32, "Lato-Regular", 50.0);
}