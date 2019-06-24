extern crate image;
use image::{GenericImage, GenericImageView, Rgba, DynamicImage, ImageBuffer, ImageRgba8};
extern crate imageproc;
extern crate rusttype;
use wasm_bindgen::prelude::*;
use imageproc::drawing::draw_text_mut;
use imageproc::morphology::dilate_mut;
use imageproc::distance_transform::Norm;
use rusttype::{FontCollection, Scale};
use crate::{PhotonImage, helpers, Rgb};

/// Add text to an image.
#[wasm_bindgen]
pub fn create_image(width: u32, height: u32, background_color: Rgb) -> PhotonImage {
    // create a pixel 
    let pixel =  image::Rgba([background_color.r, background_color.g, background_color.b, 100]);
    let image_buffer = ImageBuffer::from_pixel(width, height, pixel);
    let rgb_img = image::ImageRgba8(image_buffer);

    let raw_pixels = rgb_img.raw_pixels();
    return PhotonImage { raw_pixels: raw_pixels, width: width, height: height};
}
