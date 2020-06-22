extern crate image;
use image::{GenericImage, GenericImageView, DynamicImage, RgbaImage};
extern crate imageproc;
extern crate rusttype;
use imageproc::drawing::*;
use crate::{Rgb};
use palette::{LinSrgba, Gradient, Lch, Srgba};
use image::{Rgba};

use palette::encoding::pixel::Pixel;
use imageproc::rect::Rect;
use crate::text::draw_text;
// use crate::helpers;
use wasm_bindgen::prelude::*;

mod web {

    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    pub fn draw_solid_rect(photon_img: &PhotonImage, width: u32, height: u32, x_pos: i32, y_pos: i32) -> HtmlCanvasElement {
        let sampling_filter = image::FilterType::Nearest;
    
        let dyn_img = helpers::dyn_image_from_raw(&photon_img);
        let resized_img = image::ImageRgba8(image::imageops::resize(&dyn_img, width, height, sampling_filter));
    
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document
            .create_element("canvas").unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
    
        canvas.set_width(resized_img.width() * 2);
        canvas.set_height(resized_img.height() * 2);
    
        let new_img_data = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut resized_img.raw_pixels()), canvas.width(), canvas.height());
    
        let ctx = canvas
        .get_context("2d").unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>().unwrap();
    
    
        // Place the new imagedata onto the canvas
        ctx.put_image_data(&new_img_data.unwrap(), 0.0, 0.0);
    
        ctx.beginPath();
        ctx.rect(20.0, 30.0, 10.0, 20.0);
        ctx.stroke();
    
        return canvas;
    }
}