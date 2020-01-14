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
use web_sys::{HtmlCanvasElement, ImageData, CanvasRenderingContext2d, TextMetrics, CanvasGradient};
use wasm_bindgen::Clamped;
use std::cmp::max;

#[wasm_bindgen]
pub fn draw_rect(ctx: &CanvasRenderingContext2d, width: f64, height: f64, x_pos: i32, y_pos: i32) {
    ctx.begin_path();
    ctx.rect(x_pos as f64, y_pos as f64, width, height);
    ctx.stroke();
}

#[wasm_bindgen]
pub fn draw_text(ctx: &CanvasRenderingContext2d, font: &str, text: &str, x_pos: f64, y_pos: f64) {
    ctx.set_font(font);
    ctx.fill_text(text, x_pos, y_pos); 
}

#[wasm_bindgen]
pub fn draw_stroke_text(ctx: &CanvasRenderingContext2d, font: &str, text: &str, x_pos: f64, y_pos: f64) {
    ctx.set_font(font);
    ctx.stroke_text(text, x_pos, y_pos); 
}

#[wasm_bindgen]
pub fn draw_gradient(ctx: &CanvasRenderingContext2d, x_pos: f64, y_pos: f64, width: f64, height: f64, color1: &str, color2: &str) {
    let gradient = ctx.create_linear_gradient(x_pos, y_pos, x_pos + width, y_pos + height);
    gradient.add_color_stop(0.0, "rgb(34, 56, 90)").unwrap();
    gradient.add_color_stop(1.0, color2).unwrap();

    ctx.set_fill_style(&gradient);
    ctx.fill_rect(x_pos, y_pos, width, height); 
}

#[wasm_bindgen]
pub fn draw_horizontal_stripes(ctx: &CanvasRenderingContext2d, width: f64, height: f64) {
    draw_stripes(ctx, width, height, "horizontal");
}

#[wasm_bindgen]
pub fn draw_vertical_stripes(ctx: &CanvasRenderingContext2d, width: f64, height: f64) {
    draw_stripes(ctx, width, height, "vertical");
}

#[wasm_bindgen]
pub fn draw_diagonal_stripes(ctx: &CanvasRenderingContext2d, width: f64, height: f64) {
    draw_stripes(ctx, width, height, "diagonal");
}

#[wasm_bindgen]
pub fn draw_stripes(ctx: &CanvasRenderingContext2d, width: f64, height: f64, orientation: &str) {
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

struct Word {
    width: f64,
    text: &'static str,
}

#[wasm_bindgen]
pub fn text_graphic(ctx: &CanvasRenderingContext2d, text: &str) {
    let width = ctx.canvas().unwrap().width() as f64;
    let max_space_size = 3.0;
    let min_space_size = 0.5;

    let text_align = ctx.text_align();
    ctx.set_text_align("left");

    let mut words_width = 0.0;

    let words = vec!["hi", "how", "are", "you"];
    let mut canvas_words: Vec<Word> = vec![];

    for word in &words {
        let w: f64 = ctx.measure_text(word).unwrap().width();
        words_width += w;
        let canvas_word = Word{width: w, text: word};
        canvas_words.push(canvas_word);
    };

    let count = words.len();
    let spaces = count - 1;
    let space_width = ctx.measure_text(" ").unwrap().width();
    let adj_space = max((space_width * min_space_size) as u32, ((width - words_width) / spaces as f64) as u32) as f64;
    let mut use_size = 0.0;

    // let use_size = adj_space > space_width * max_space_size ? space_width : adj_space;
    if adj_space > space_width * max_space_size {
        use_size = space_width;    
    }
    else {
        use_size = adj_space
    };

    let total_width = words_width + use_size * spaces as f64;

    // if (render_type == "MEASURE") {
    //     ctx.set_text_align(text_align);
    //     // ret total width
    // }
    
}


#[wasm_bindgen]
pub fn draw_preset_gradient(ctx: &CanvasRenderingContext2d, x_pos: f64, y_pos: f64, width: f64, height: f64, preset: &str) {

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
pub fn draw_gradient_lines(ctx: &CanvasRenderingContext2d, x_pos: f64, y_pos: f64, width: f64, height: f64, orientation: &str) {

    let gradient = ctx.create_linear_gradient(x_pos, y_pos, x_pos + width, y_pos + height);
    gradient.add_color_stop(0.0, "red").unwrap();
    gradient.add_color_stop(1.0, "magenta").unwrap();

    let num_stripes = 30;
    let thickness = width / num_stripes as f64;

    for i in 0..num_stripes * 2 {
        let i: f64 = i as f64;

        ctx.begin_path();
        ctx.set_stroke_style(&gradient);

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
pub fn draw_gradient_waves(ctx: CanvasRenderingContext2d, x_pos: f64, y_pos: f64, width: f64, height: f64) {
    // draw background on canvas
    draw_gradient(&ctx, x_pos, y_pos, width, height, "rgb(149, 64, 254)", "rgb(149, 64, 254)");

    let gradient = ctx.create_linear_gradient(x_pos, y_pos, x_pos + width, y_pos + height);
    gradient.add_color_stop(0.0, "rgb(0, 221, 255)").unwrap();
    gradient.add_color_stop(1.0, "blue").unwrap();

    ctx.move_to(50.0, 130.0);
    ctx.bezier_curve_to(300.0, 50.0, 200.0, 400.0, 490.0, 100.0);
    ctx.line_to(490.0, 300.0);
    ctx.line_to(50.0, 300.0);
    ctx.close_path();
     
    ctx.set_line_width(15.0);
    ctx.set_stroke_style(&gradient);
    ctx.set_fill_style(&gradient);
     
    ctx.stroke();
    ctx.fill();
}

#[wasm_bindgen]
pub fn draw_gradient_wave(ctx: CanvasRenderingContext2d, x_pos: f64, y_pos: f64, width: f64, height: f64) {

    let gradient = ctx.create_linear_gradient(x_pos, y_pos, x_pos + width, y_pos + height);
    gradient.add_color_stop(0.0, "rgb(0, 221, 255)").unwrap();
    gradient.add_color_stop(1.0, "blue").unwrap();

    ctx.move_to(50.0, 130.0);
    ctx.bezier_curve_to(300.0, 50.0, 200.0, 400.0, 490.0, 100.0);
    ctx.line_to(490.0, 300.0);
    ctx.line_to(50.0, 300.0);
    ctx.close_path();
     
    ctx.set_line_width(15.0);
    ctx.set_stroke_style(&gradient);
    ctx.set_fill_style(&gradient);
     
    ctx.stroke();
    ctx.fill();
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