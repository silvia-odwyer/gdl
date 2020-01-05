extern crate image;
use image::{GenericImage, GenericImageView, DynamicImage, RgbaImage};
extern crate imageproc;
extern crate rusttype;
use imageproc::drawing::*;
use crate::{Rgb, helpers, PhotonImage};
use image::{Rgba};
use wasm_bindgen::JsCast;
use imageproc::rect::Rect;
use wasm_bindgen::prelude::*;
use web_sys::{HtmlCanvasElement, ImageData, CanvasRenderingContext2d, CanvasGradient};
use wasm_bindgen::Clamped;

#[wasm_bindgen]
pub fn draw_rect(ctx: CanvasRenderingContext2d, width: f64, height: f64, x_pos: i32, y_pos: i32) {
    ctx.begin_path();
    ctx.rect(x_pos as f64, y_pos as f64, width, height);
    ctx.stroke();
}

#[wasm_bindgen]
pub fn draw_text(ctx: CanvasRenderingContext2d, font: &str, text: &str, x_pos: f64, y_pos: f64) {
    ctx.set_font(font);
    ctx.fill_text(text, x_pos, y_pos); 
}

#[wasm_bindgen]
pub fn draw_stroke_text(ctx: CanvasRenderingContext2d, font: &str, text: &str, x_pos: f64, y_pos: f64) {
    ctx.set_font(font);
    ctx.stroke_text(text, x_pos, y_pos); 
}

#[wasm_bindgen]
pub fn draw_gradient(ctx: CanvasRenderingContext2d, x_pos: f64, y_pos: f64, width: f64, height: f64, color1: &str, color2: &str) {
    let gradient = ctx.create_linear_gradient(x_pos, y_pos, x_pos + width, y_pos + height);
    gradient.add_color_stop(0.0, "rgb(34, 56, 90)").unwrap();
    gradient.add_color_stop(1.0, color2).unwrap();

    ctx.set_fill_style(&gradient);
    ctx.fill_rect(x_pos, y_pos, width, height); 
}

#[wasm_bindgen]
pub fn draw_preset_gradient(ctx: CanvasRenderingContext2d, x_pos: f64, y_pos: f64, width: f64, height: f64, preset: &str) {

    let (color1, color2) = match preset {
        "lemongrass" => ("blue", "yellow"),
        "flames" => ("#f12711", "#f5af19"),
        "pastel" => ("#8360c3", "#2ebf91"),
        _ => ("magenta", "black")
    };

    let gradient = ctx.create_linear_gradient(x_pos, y_pos, x_pos + width, y_pos + height);
    gradient.add_color_stop(0.0, color1).unwrap();
    gradient.add_color_stop(1.0, color2).unwrap();

    ctx.set_fill_style(&gradient);
    ctx.fill_rect(x_pos, y_pos, width, height); 
}

#[wasm_bindgen]
pub fn create_canvas_ctx(width: u32, height: u32) -> HtmlCanvasElement {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document
        .create_element("canvas").unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
    
    canvas.set_width(width as u32);
    canvas.set_height(height as u32);
        
    let ctx = canvas
        .get_context("2d").unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>().unwrap();
    
    
        // Place the new imagedata onto the canvas
        // ctx.put_image_data(&new_img_data.unwrap(), 0.0, 0.0);

    return canvas;
}