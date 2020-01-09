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

// var color1 = “#24A8AC”,color2=”#0087CB”;
// var numberOfStripes = 30;
// for (var i=0;i < numberOfStripes;i++){
// var thickness = 300 / numberOfStripes;
// drawingContext.beginPath();
// drawingContext.strokeStyle = i % 2?color1:color2;
// drawingContext.lineWidth =thickness;

// drawingContext.moveTo(0,i*thickness + thickness/2);
// drawingContext.lineTo(300,i*thickness + thickness/2);
// drawingContext.stroke();

#[wasm_bindgen]
pub fn draw_horizontal_stripes(ctx: CanvasRenderingContext2d, width: f64, height: f64) {
    draw_stripes(ctx, width, height, "horizontal");
}

#[wasm_bindgen]
pub fn draw_vertical_stripes(ctx: CanvasRenderingContext2d, width: f64, height: f64) {
    draw_stripes(ctx, width, height, "vertical");
}

#[wasm_bindgen]
pub fn draw_diagonal_stripes(ctx: CanvasRenderingContext2d, width: f64, height: f64) {
    draw_stripes(ctx, width, height, "diagonal");
}

#[wasm_bindgen]
pub fn draw_stripes(ctx: CanvasRenderingContext2d, width: f64, height: f64, orientation: &str) {
    let color1 = JsValue::from_str("#D0C91F");
    let color2 = JsValue::from_str("#FF4C65");

    let num_stripes = 30;
    let thickness: f64 = width / num_stripes as f64;

    for i in 0..num_stripes * 2 {
        let i: f64 = i as f64;
        ctx.begin_path();
        let color = match i as u32 % 2 {
            0 => &color1,
            1 => &color2,
            _ => &color2,
        };

        ctx.set_stroke_style(color);
        ctx.set_line_width(thickness);
        if orientation == "vertical" {
            ctx.move_to(i * thickness + thickness /2.0 , 0.0);
            ctx.line_to(i* thickness + thickness / 2.0, height);
        }
        // horizontal stripes instead
        else if orientation == "horizontal"{
            ctx.move_to(0.0, i * thickness + thickness / 2.0);
            ctx.line_to(width, i * thickness + thickness / 2.0);
        }
        else if orientation == "diagonal" {
            ctx.set_line_cap("round");
            ctx.move_to(i * thickness + thickness / 2.0 - height, 0.0);
            ctx.line_to(i * thickness + thickness / 2.0, height);
        }

        ctx.stroke();
    }
}

#[wasm_bindgen]
pub fn draw_preset_gradient(ctx: CanvasRenderingContext2d, x_pos: f64, y_pos: f64, width: f64, height: f64, preset: &str) {

    let (color1, color2) = match preset {
        "lemongrass" => ("blue", "yellow"),
        "flames" => ("#f12711", "#f5af19"),
        "pastel" => ("#8360c3", "#2ebf91"),
        "pink" => ("#DD5E89", "#F7BB97"),
        "ocean" => ("#4CB8C4", "#3CD3AD"),
        "aquamarine" => ("#1A2980", "#26D0CE"),
        "rosewater" => ("#E55D87", "#5FC3E4"),
        "zigzag" => ("#3CA55C", "#B5AC49"), 
        "seaweed" => ("#348F50", "#56B4D3"),
        "tropics" => ("#16A085", "#F4D03F"),
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