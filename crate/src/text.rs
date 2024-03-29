//! Draw text onto images.

use crate::Rgb;
use image::imageops::{rotate180, rotate270, rotate90};
use image::{DynamicImage, GenericImage, GenericImageView, Rgba};
use imageproc::distance_transform::Norm;
use imageproc::drawing::draw_text_mut;
use imageproc::morphology::dilate_mut;
use rusttype::{Font, Scale};
use std::fs;

/// Draw text onto an image with a border around the text.
///
/// ### Arguments
/// * `img` - Mutable reference to a DynamicImage.
/// * `text` - Text string to be drawn.
/// * `x` - X-coordinate of top corner of text.
/// * `y` - Y coordinae of top corner of text.
pub fn draw_text_with_border(image: &mut DynamicImage, font: &str, text: &str, x: u32, y: u32) {
    let mut image2: DynamicImage = DynamicImage::new_luma8(image.width(), image.height());

    let font_vec = open_font(font);
    let font = Font::try_from_bytes(&font_vec).unwrap();
    let height = 90f32;
    let scale = Scale {
        x: height * 1.0,
        y: height,
    };
    draw_text_mut(
        &mut image2,
        Rgba([255u8, 255u8, 255u8, 255u8]),
        x,
        y,
        scale,
        &font,
        text,
    );

    let mut image2 = image2.to_luma8();
    dilate_mut(&mut image2, Norm::LInf, 14u8);

    // Add a border to the text.
    for x in 0..image2.width() {
        for y in 0..image2.height() {
            let pixval = 255 - image2.get_pixel(x, y).0[0];
            if pixval != 255 {
                let new_pix = Rgba([234, 23, 123, 255]);
                image.put_pixel(x, y, new_pix);
            }
        }
    }
    draw_text_mut(
        image,
        Rgba([193u8, 255u8, 255u8, 255u8]),
        x + 10,
        y - 10,
        scale,
        &font,
        text,
    );
}

/// Draw text onto an image.
///
/// ### Arguments
/// * `img` - Mutable reference to a DynamicImage.
/// * `text` - Text string to be drawn.
/// * `x` - X-coordinate of top corner of text.
/// * `y` - Y coordinae of top corner of text.
/// * `font` - Font name. Fonts available include Roboto-Regular, BebasKai, Roboto-Light, among many others.
/// Full list of fonts available coming soon.
/// * `font_size`: f32 that represents the font's size.
/// * `rgb`: Rgb text color.
pub fn draw_text(
    image: &mut DynamicImage,
    text: &str,
    x: u32,
    y: u32,
    font: &str,
    font_size: f32,
    rgb: &Rgb,
) {
    // include_bytes! only takes a string literal
    let font_vec = open_font(font);

    let font = Font::try_from_bytes(&font_vec).unwrap();
    let height = font_size;
    let scale = Scale {
        x: height * 1.0,
        y: height,
    };

    draw_text_mut(
        image,
        Rgba([rgb.r as u8, rgb.g as u8, rgb.b as u8, 255u8]),
        x + 10,
        y - 10,
        scale,
        &font,
        text,
    );
}

/// Draw vertical text onto an image.
///
/// This is done by drawing the text horizontally onto an image,
/// then rotating this image by 90 degrees.
///
/// ### Arguments
/// * `img` - Mutable reference to a DynamicImage.
/// * `text` - Text string to be drawn.
/// * `x` - X-coordinate of top corner of text.
/// * `y` - Y coordinae of top corner of text.
/// * `font` - Font name. Fonts available include Roboto-Regular, BebasKai, Roboto-Light, among many others.
/// Full list of fonts available coming soon.
/// * `font_size`: f32 that represents the font's size.
/// * `direction`: The direction the text should be facing, either "left" or "right".
/// * `rgb`: Rgb text color.
pub fn draw_vertical_text(
    img: &mut DynamicImage,
    text: &str,
    x: u32,
    y: u32,
    font: &str,
    font_size: f32,
    direction: &str,
    rgb: &Rgb,
) {
    if direction == "left" {
        draw_rotated_text(img, text, x, y, font, font_size, "270", rgb);
    } else if direction == "right" {
        draw_rotated_text(img, text, x, y, font, font_size, "90", rgb);
    }
}

/// Draw single letters in a vertical column to create a vertical-text effect.
///
/// # Arguments
/// * `img` - A mutable ref to a DynamicImage.
/// * `text` - The text to be drawn onto the image.
/// * `font` - The font type.
/// * `font_size` - The size of the font.
/// * `rgb` - Rgb color.
pub fn draw_vertical_text_single(
    img: &mut DynamicImage,
    text: &str,
    x: u32,
    mut y: u32,
    font: &str,
    font_size: f32,
    rgb: &Rgb,
) {
    for c in text.split("") {
        draw_text(img, c, x, y, font, font_size, rgb);
        y += (font_size * 0.8) as u32;
    }
}

/// Draw upside-down text.
///
/// ### Arguments
/// * `img` - Mutable reference to a DynamicImage.
/// * `text` - Text string to be drawn.
/// * `x` - X-coordinate of top corner of text.
/// * `y` - Y coordinae of top corner of text.
/// * `font` - Font name. Fonts available include Roboto-Regular, BebasKai, Roboto-Light, among many others.
/// Full list of fonts available coming soon.
/// * `font_size`: f32 that represents the font's size.
/// * `rgb`: Rgb text color.
pub fn draw_upsidedown_text(
    img: &mut DynamicImage,
    text: &str,
    x: u32,
    y: u32,
    font: &str,
    font_size: f32,
    rgb: &Rgb,
) {
    draw_rotated_text(img, text, x, y, font, font_size, "180", rgb);
}

// Draw rotated text. Available: 90, 180, 270.
fn draw_rotated_text(
    image: &mut DynamicImage,
    text: &str,
    x: u32,
    y: u32,
    font: &str,
    font_size: f32,
    rotation: &str,
    rgb: &Rgb,
) {
    // Since the image will be rotated, the height of the container image will be the width of the
    // text image.

    let font_img_height = text.len() as f32 * (font_size * 0.48);
    let font_img_width = font_size * 1.3;
    let mut image2: DynamicImage =
        DynamicImage::new_luma8(font_img_height as u32, font_img_width as u32);

    let font_vec = open_font(font);

    let font = Font::try_from_bytes(&font_vec).unwrap();
    let scale = Scale {
        x: font_size * 1.0,
        y: font_size,
    };
    draw_text_mut(
        &mut image2,
        Rgba([rgb.r as u8, rgb.g as u8, rgb.b as u8, 255u8]),
        10,
        10,
        scale,
        &font,
        &text,
    );

    let image2 = image2.to_rgba8();

    // draw_text_mut(&mut image, Rgba([rgb.r as u8, rgb.g as u8, rgb.b as u8, 255u8]), x + 10, y - 10, scale, &font, text);

    let rotated_img = match rotation {
        "90" => rotate90(&image2),
        "180" => rotate180(&image2),
        "270" => rotate270(&image2),
        _ => rotate90(&image2),
    };

    let mut image2 = image::DynamicImage::ImageRgba8(rotated_img);

    image::imageops::overlay(image, &mut image2, x, y);
}

fn open_font(font: &str) -> std::vec::Vec<u8> {
    let font_path = format!("fonts/{}.ttf", font);
    let font_vec1 = fs::read(font_path).expect("Unable to read file");

    return font_vec1;
}
