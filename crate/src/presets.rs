/// Templates for rapid graphic creation.

extern crate image;
extern crate imageproc;
extern crate rusttype;
use wasm_bindgen::Clamped;
use wasm_bindgen::prelude::*;
use crate::{PhotonImage, helpers, Rgb};
use crate::text::*;
use crate::elements::*;
use crate::helpers::dyn_image_from_raw;
use crate::{graphics, resize, new_with_background};
use image::{DynamicImage, GenericImageView};

/// Preset: Centre text, with background image.
/// 
/// # Arguments
/// * `img` - A mutable ref to a PhotonImage.
/// * `main_text` - Main heading for the graphic.
/// * `width` - u32 - Desired width of final graphic 
/// * `height` - u32 - Desired height of final graphic
#[wasm_bindgen]
pub fn centre_text(mut background_img: &mut DynamicImage, main_text: &str) {
    let width = background_img.width();
    let height = background_img.height();

    let font_size = 150.0;

    let group_vec = text_to_vec(main_text, background_img.width(), font_size);

    let mut height_mul: f32 = 0.05;
    let white_rgb = Rgb { r: 255, g: 255, b: 255};
    for word_vec in group_vec {
        let text = word_vec.join(" ");
        draw_text(background_img, &text, (width as f32 * 0.3) as u32, (height as f32 * height_mul) as u32, "BebasKai", font_size, &white_rgb);
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
pub fn text_shades(mut background_img: &mut DynamicImage, main_text: &str) {
    let width = background_img.width();
    let height = background_img.height();

    let mut height_mul: f32 = 0.05;
    let white_rgb = Rgb { r: 255, g: 255, b: 255};
    for _ in 0..(height / 50) as usize{
        draw_text(background_img, main_text, (width as f32 * 0.05) as u32, (height as f32 * height_mul) as u32, "BebasKai", 110.0, &white_rgb);
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
pub fn repeat_text(mut background_img: &mut DynamicImage, main_text: &str) {
    let width = background_img.width();
    let height = background_img.height();

    let mut height_mul: f32 = 0.05;
    let white_rgb = Rgb { r: 255, g: 255, b: 255};
    for _ in 0..(height / 50) as usize{
        draw_text(background_img, main_text, (width as f32 * 0.05) as u32, (height as f32 * height_mul) as u32, "BebasKai", 110.0, &white_rgb);
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
pub fn text_banner(mut background_img: &mut DynamicImage, main_text: &str, small_text: &str) {
    let width = background_img.width();
    let height = background_img.height();

    let mut height_mul: f32 = 0.4;
    let black_rgb = Rgb { r: 0, g: 0, b: 0};
    
    draw_text(background_img, main_text, (width as f32 * 0.15) as u32, (height as f32 * height_mul) as u32, "BebasKai", 110.0, &black_rgb);
    draw_text(background_img, small_text, (width as f32 * 0.28) as u32, (height as f32 * (height_mul + 0.15)) as u32, "BebasKai", 30.0, &black_rgb);   
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
pub fn vertical_text(mut background_img: &mut DynamicImage, main_text: &str) {
    let width = background_img.width();
    let height = background_img.height();

    let height_mul: f32 = 0.4;
    let black_rgb = Rgb { r: 0, g: 0, b: 0};
    draw_solid_rect(&mut background_img, &black_rgb, height, (width as f32 * 0.2) as u32, 0, 0);
    
    let white_rgb = Rgb {r: 255, g: 255, b: 255};

    draw_vertical_text(background_img, main_text, (width as f32 * 0.15) as u32, (height as f32 * height_mul) as u32, "BebasKai", 110.0, "right", &white_rgb);
}

/// Preset: Right-hand side text.
/// 
/// # Arguments
/// * `img` - A mutable ref to a PhotonImage.
/// * `main_text` - Main heading for the graphic.
/// * `small_text` - Sub-heading/smaller text. 
#[wasm_bindgen]
pub fn rhs_text(mut background_img: &mut DynamicImage, main_text: &str) {
    let width = background_img.width();
    let height = background_img.height();

    let height_mul: f32 = 0.4;
    let rgb = Rgb { r: 255, g: 255, b: 255};
    
    draw_text(background_img, main_text, (width as f32 * 0.65) as u32, (height as f32 * height_mul) as u32, "BebasKai", 130.0, &rgb);
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
pub fn lhs_text(mut background_img: &mut DynamicImage, main_text: &str, small_text: &str, width: u32, height: u32) {
    let width = background_img.width();
    let height = background_img.height();

    let height_mul: f32 = 0.4;
    let black_rgb = Rgb { r: 0, g: 0, b: 0};
    
    draw_text(background_img, main_text, (width as f32 * 0.15) as u32, (height as f32 * height_mul) as u32, "BebasKai", 130.0, &black_rgb);
}

/// Preset: Right-hand side vertical text.
/// 
/// # Arguments
/// * `img` - A mutable ref to a PhotonImage.
/// * `main_text` - Main heading for the graphic.
/// * `small_text` - Sub-heading/smaller text. 
#[wasm_bindgen]
pub fn vertical_text_rhs(mut background_img: &mut DynamicImage, main_text: &str, small_text: &str) {
    let width = background_img.width();
    let height = background_img.height();

    let black_rgb = Rgb { r: 0, g: 0, b: 0};
    
    let red_rgb = Rgb { r: 200, g: 20, b: 50 };
    let white_rgb = Rgb { r: 255, g: 255, b: 255};

    let height_mul = 0.1;
    draw_solid_rect(&mut background_img, &white_rgb, (width as f32 * 0.2) as u32, height, (width as f32 * 0.8) as i32, 0);
    draw_vertical_text(&mut background_img, main_text, (width as f32 * 0.85) as u32, (height as f32 * height_mul) as u32, "BebasKai", 100.0, "right", &red_rgb);

}

/// Preset: Quote-style graphic, featuring prominence on the main text.
/// 
/// # Arguments
/// * `img` - A mutable ref to a PhotonImage.
/// * `main_text` - Main heading for the graphic.
/// * `small_text` - Sub-heading/smaller text. 
/// * `width` - u32 - Desired width of final graphic 
/// * `height` - ù32 - Desired height of final graphic
#[wasm_bindgen]
pub fn quote(mut background_img: &mut DynamicImage, main_text: &str, small_text: &str) {
    let _width = background_img.width();
    let height = background_img.height();

    let black_rgb = Rgb { r: 0, g: 0, b: 0};
    
    let red_rgb = Rgb { r: 200, g: 20, b: 50 };
    let white_rgb = Rgb { r: 255, g: 255, b: 255};

    let mut height_mul = 0.1;
    let font_size = 100.0;

    let group_vec = text_to_vec(main_text, background_img.width(), font_size);
    for word_vec in group_vec {
        let text = word_vec.join(" ");
        draw_text(&mut background_img, &text, 0, (height as f32 * height_mul) as u32, "Oswald", font_size, &black_rgb);
        height_mul += 0.1;
    }

}

/// Preset: Postcard-style image, featuring main text overlayed onto the image.
/// 
/// # Arguments
/// * `img` - A mutable ref to a PhotonImage.
/// * `main_text` - Main heading for the graphic.
/// * `small_text` - Sub-heading/smaller text. 
/// * `width` - u32 - Desired width of final graphic 
/// * `height` - ù32 - Desired height of final graphic
#[wasm_bindgen]
pub fn postcard(background_img: DynamicImage, main_text: &str, small_text: &str, width: u32, height: u32) -> DynamicImage {
    let white = Rgb { r: 255, g: 255, b: 255};
    let blk = Rgb { r: 56, g: 123, b: 254};

    let mut container_img = new_with_background(width, height, &white);
    let sampling_filter = image::FilterType::Nearest;

    let resized_img = image::ImageRgba8(image::imageops::resize(&background_img, width - 20, height - 20, sampling_filter));

    let main_img_width = background_img.width();
    let main_img_height = background_img.height();

    image::imageops::overlay(&mut container_img, &resized_img, 10, 10);

    let height_mul: f32 = 0.2;
    let black_rgb = Rgb { r: 0, g: 0, b: 0};
    
    draw_text(&mut container_img, main_text, (main_img_width as f32 * 0.15) as u32, 
    (main_img_height as f32 * height_mul) as u32, "MrDafoe-Regular", 130.0, &blk);
    return container_img;
}

// Convert a string of text to a vector containing vecs of words, 
// which will fit on an individual line or within a constraint.
fn text_to_vec(text: &str, width: u32, font_size: f32) -> Vec<Vec<&str>> {
      
    let mut word_vec = vec![];
    let font_size: f32 = font_size as f32 * 0.8;
    let mut group_vec = vec![];
    let mut total_width = 0;
    for word in text.split_whitespace() {
        // get len of word, num of chars, and multiply by font size.
        let width_word = font_size * word.len() as f32;
        total_width += width_word as u32;

        if total_width > width {
            group_vec.push(word_vec);
            word_vec = vec![];
            word_vec.push(word);
            total_width = width_word as u32;
        }
        else {
            word_vec.push(word);
        }
    }
    
    group_vec.push(word_vec);

    return group_vec
}