/// Templates for rapid graphic creation.

extern crate image;
use image::{GenericImageView, DynamicImage};
extern crate imageproc;
extern crate rusttype;

use wasm_bindgen::Clamped;
use wasm_bindgen::prelude::*;
use crate::{PhotonImage, helpers, Rgb};
use crate::text::*;
use crate::elements::*;

/// Preset: Centre text, with background image.
/// 
/// # Arguments
/// * `img` - A mutable ref to a PhotonImage.
/// * `main_text` - Main heading for the graphic.
/// * `width` - u32 - Desired width of final graphic 
/// * `height` - ù32 - Desired height of final graphic
#[wasm_bindgen]
pub fn centre_text(mut background_img: &mut PhotonImage, main_text: &str, width: u32, height: u32) {
    let width = background_img.width;
    let height = background_img.height;
    let pangram: &str = "the quick brown fox jumps over the lazy dog";

    let word_vec = text_to_vec(main_text);

    let mut height_mul: f32 = 0.3;
    let white_rgb = Rgb { r: 255, g: 255, b: 255};
    for word in word_vec {
        draw_text(&mut background_img, word, (width as f32 * 0.3) as u32, (height as f32 * height_mul) as u32, "BebasKai", 150.0, &white_rgb);
        height_mul += 0.15;
    }
  
}

/// Preset: Repeat the same text on each line, with each line changing in shade.
/// 
/// # Arguments
/// * `img` - A mutable ref to a PhotonImage.
/// * `main_text` - Main heading for the graphic.
/// * `width` - u32 - Desired width of final graphic 
/// * `height` - ù32 - Desired height of final graphic
#[wasm_bindgen]
pub fn text_shades(mut background_img: &mut PhotonImage, main_text: &str, width: u32, height: u32) {
    let width = background_img.width;
    let height = background_img.height;

    let mut height_mul: f32 = 0.05;
    let white_rgb = Rgb { r: 255, g: 255, b: 255};
    for _ in 0..(height / 50) as usize{
        draw_text(&mut background_img, main_text, (width as f32 * 0.05) as u32, (height as f32 * height_mul) as u32, "BebasKai", 110.0, &white_rgb);
        height_mul += 0.1;
    }
  
}

/// Preset: Repeat the same text on each line.
/// 
/// # Arguments
/// * `img` - A mutable ref to a PhotonImage.
/// * `main_text` - Main heading for the graphic.
/// * `width` - u32 - Desired width of final graphic 
/// * `height` - ù32 - Desired height of final graphic
#[wasm_bindgen]
pub fn repeat_text(mut background_img: &mut PhotonImage, main_text: &str, width: u32, height: u32) {
    let width = background_img.width;
    let height = background_img.height;

    let mut height_mul: f32 = 0.05;
    let white_rgb = Rgb { r: 255, g: 255, b: 255};
    for _ in 0..(height / 50) as usize{
        draw_text(&mut background_img, main_text, (width as f32 * 0.05) as u32, (height as f32 * height_mul) as u32, "BebasKai", 110.0, &white_rgb);
        height_mul += 0.1;
    } 
}

/// Preset: text banner.
/// 
/// # Arguments
/// * `img` - A mutable ref to a PhotonImage.
/// * `main_text` - Main heading for the graphic.
/// * `small_text` - Sub-heading/smaller text. 
/// * `width` - u32 - Desired width of final graphic 
/// * `height` - ù32 - Desired height of final graphic
#[wasm_bindgen]
pub fn text_banner(mut background_img: &mut PhotonImage, main_text: &str, small_text: &str, width: u32, height: u32) {
    let width = background_img.width;
    let height = background_img.height;

    let mut height_mul: f32 = 0.4;
    let black_rgb = Rgb { r: 0, g: 0, b: 0};
    
    draw_text(&mut background_img, main_text, (width as f32 * 0.15) as u32, (height as f32 * height_mul) as u32, "BebasKai", 110.0, &black_rgb);
    draw_text(&mut background_img, small_text, (width as f32 * 0.28) as u32, (height as f32 * (height_mul + 0.15)) as u32, "BebasKai", 30.0, &black_rgb);   
}

/// Preset: vertical text banner.
/// 
/// # Arguments
/// * `img` - A mutable ref to a PhotonImage.
/// * `main_text` - Main heading for the graphic.
/// * `small_text` - Sub-heading/smaller text. 
/// * `width` - u32 - Desired width of final graphic 
/// * `height` - ù32 - Desired height of final graphic
#[wasm_bindgen]
pub fn vertical_text(mut background_img: &mut PhotonImage, main_text: &str, width: u32, height: u32) {
    let width = background_img.width;
    let height = background_img.height;

    let mut height_mul: f32 = 0.4;
    let black_rgb = Rgb { r: 0, g: 0, b: 0};
    draw_solid_rect(&mut background_img, &black_rgb, height, (width as f32 * 0.2) as u32, 0, 0);
    
    let white_rgb = Rgb {r: 255, g: 255, b: 255};

    draw_vertical_text(&mut background_img, main_text, (width as f32 * 0.15) as u32, (height as f32 * height_mul) as u32, "BebasKai", 110.0, &white_rgb);
}

/// Preset: Right-hand side text.
/// 
/// # Arguments
/// * `img` - A mutable ref to a PhotonImage.
/// * `main_text` - Main heading for the graphic.
/// * `small_text` - Sub-heading/smaller text. 
/// * `width` - u32 - Desired width of final graphic 
/// * `height` - ù32 - Desired height of final graphic
#[wasm_bindgen]
pub fn rhs_text(mut background_img: &mut PhotonImage, main_text: &str, small_text: &str, width: u32, height: u32) {
    let width = background_img.width;
    let height = background_img.height;

    let mut height_mul: f32 = 0.4;
    let rgb = Rgb { r: 255, g: 255, b: 255};
    
    draw_text(&mut background_img, main_text, (width as f32 * 0.65) as u32, (height as f32 * height_mul) as u32, "BebasKai", 130.0, &rgb);
}

/// Preset: Left-hand side text.
/// 
/// # Arguments
/// * `img` - A mutable ref to a PhotonImage.
/// * `main_text` - Main heading for the graphic.
/// * `small_text` - Sub-heading/smaller text. 
/// * `width` - u32 - Desired width of final graphic 
/// * `height` - ù32 - Desired height of final graphic
#[wasm_bindgen]
pub fn lhs_text(mut background_img: &mut PhotonImage, main_text: &str, small_text: &str, width: u32, height: u32) {
    let width = background_img.width;
    let height = background_img.height;

    let mut height_mul: f32 = 0.4;
    let black_rgb = Rgb { r: 0, g: 0, b: 0};
    
    draw_text(&mut background_img, main_text, (width as f32 * 0.15) as u32, (height as f32 * height_mul) as u32, "BebasKai", 130.0, &black_rgb);
}

fn text_to_vec(text: &str) -> Vec<&str> {
        // let mut str_portion = String::new();
    // let mut word_vec = vec![];
    // let words = main_text.split(" ").collect::<Vec<&str>>();    
    // // let mut total_chars = 0;
    // let mut i = 0;
    // for word in &words {
    //     i += 1;
    //     total_chars += word.len();
    //     println!("word: {} total chars is: {}", word, total_chars);

    //     if total_chars > 7  && i < words.len() - 1 {
    //         total_chars = 0;
    //         word_vec.push(str_portion);
    //         let mut str_portion = String::new().push_str(word);
    //         continue;
    //     }
    //     else if total_chars > 7 && i < words.len() {
    //         word_vec.push(str_portion.clone());
    //     }
    //     else {
    //         str_portion.push_str(word);
    //     }
    // }


    // println!("word vec: {:?}", word_vec);

    // // Split main text into groups of strings
    // let chars: Vec<char> = main_text.chars().collect();    
    // let mut num_chars = 0;
    // let mut str_vec = vec![];

    // for c in chars {

    //     if num_chars > 7 {
    //         str_vec.push(str_portion);
    //         str_portion = "".to_string();
    //         continue;
    //     }
    //     num_chars += 1;
    //     str_portion.push(c);
    // }

    // println!("str vec: {:?}", str_vec);
    
    
    let mut word_vec = vec![];

    for word in text.split_whitespace() {
        println!("> {}", word);
        if word.len() > 7 {
            word_vec.push(word);
            continue;
        }
        else {
            word_vec.push(word);
        }
    }

    return word_vec;
}