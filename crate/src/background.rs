extern crate image;
use image::{GenericImage, Rgba, DynamicImage};
extern crate imageproc;
extern crate rusttype;
use wasm_bindgen::prelude::*;
use imageproc::drawing::{draw_text_mut, draw_filled_circle_mut};
use imageproc::morphology::dilate_mut;
use crate::{PhotonImage, helpers, Rgb};
use image::GenericImageView;
use palette::{Gradient, Lch, LinSrgba, Pixel, Srgba};
use image::{ImageBuffer, RgbaImage};
use std::convert::TryInto;
use imageproc::drawing::draw_line_segment_mut;

/// Create a background image containing circles 
#[wasm_bindgen]
pub fn circle_background(width: u32, height: u32) -> PhotonImage {
    let background_color = Rgb { r: 190, g: 120, b: 200};

    let pixel =  image::Rgba([background_color.r, background_color.g, background_color.b, 255]);
    let rgba_img = create_image_from_pixel(pixel);

    let white_pixel = image::Rgba([123, 10, 180, 255]);
    let CIRCLE_RADIUS: i32 = (width as i32 / 10) + 15;

    for x in 0..5 {
        for y in 0..6 {
        
            draw_filled_circle_mut(&mut rgba_img, (x * CIRCLE_RADIUS * 2, y * CIRCLE_RADIUS * 2), CIRCLE_RADIUS, white_pixel);

        }
    }

    return PhotonImage { raw_pixels: rgba_img.raw_pixels(), width: width, height: height}

}

/// Create a background image containing circles 
#[wasm_bindgen]
pub fn spaced_circle_background(width: u32, height: u32) -> PhotonImage {
    let background_color = Rgb { r: 190, g: 120, b: 200};

    let rgba_img = create_image_from_pixel(background_color);

    let white_pixel = image::Rgba([123, 10, 180, 255]);
    let CIRCLE_RADIUS: i32 = (width as i32 / 10) - 35;
    let end: usize = (width / 10).try_into().unwrap();

    for x in 0..8 {
        for y in 0..10 {
            draw_filled_circle_mut(&mut rgba_img, (x * CIRCLE_RADIUS * 3, y * CIRCLE_RADIUS * 3) , CIRCLE_RADIUS, white_pixel);
        }
    }

    return PhotonImage { raw_pixels: rgba_img.raw_pixels(), width: width, height: height}
}

pub fn solid_background(width: u32, height: u32, background_color: Rgb) -> PhotonImage {
    let rgba_img = create_image_from_pixel(background_color);
    return PhotonImage{ raw_pixels: rgba_img.raw_pixels(), width: width, height: height};
}

pub fn lined_background(width: u32, height: u32, background_color: Rgb) -> PhotonImage {
    let rgba_img = create_image_from_pixel(background_color)
    let line_pixel = image::Rgba([255, 167, 90, 255]);
    
    for y in 0..50 {
        draw_line_segment_mut(&mut rgba_img, (0 as f32, (y * 20) as f32 ), (width as f32, (y * 20) as f32 ), line_pixel);
    }
    return PhotonImage{ raw_pixels: rgba_img.raw_pixels(), width: width, height: height };
}

pub fn checker_background(width: u32, height: u32, background_color: Rgb) -> PhotonImage {
    let rgba_img = create_image_from_pixel(background_color);

    let line_pixel = image::Rgba([255, 167, 90, 255]);
    
    for y in 0..50 {
        draw_line_segment_mut(&mut rgba_img, (0.0, (y * 20) as f32 ), (width as f32, (y * 20) as f32 ), line_pixel);

    }
    for x in 0..50 {
        draw_line_segment_mut(&mut rgba_img, (x as f32 * 50.0, 0.0), (x as f32 * 50.0, height as f32 ), line_pixel);

    }
    return PhotonImage{ raw_pixels: rgba_img.raw_pixels(), width: width, height: height };
}

fn create_image_from_pixel(background_color: Rgb) -> DynamicImage {
    let pixel = image::Rgba([background_color.r, background_color.g, background_color.b, 255]);
    let image_buffer = ImageBuffer::from_pixel(width, height, pixel);
    let mut rgba_img = image::ImageRgba8(image_buffer);
    return rgba_img;
}