/// Draw text onto images.

extern crate image;
use image::{Rgba, DynamicImage};
extern crate imageproc;
extern crate rusttype;
use wasm_bindgen::prelude::*;
use imageproc::drawing::draw_text_mut;
use imageproc::morphology::dilate_mut;
use image::imageops::{rotate90, rotate270};
use imageproc::distance_transform::Norm;
use rusttype::{FontCollection, Scale};
use crate::{PhotonImage, helpers, Rgb};

/// Draw text onto an image.
///
/// ### Arguments
/// * `img` - Mutable reference to a PhotonImage.
/// * `text` - Text string to be drawn.
/// * `x` - X-coordinate of top corner of text.
/// * `y` - Y coordinae of top corner of text.
/// * `font` - Font name. Fonts available include Roboto-Regular, BebasKai, Roboto-Light, among many others. 
/// Full list of fonts available coming soon. 
/// * `font_size`: f32 that represents the font's size.
/// * `rgb`: Rgb text color.
#[wasm_bindgen]
pub fn draw_text(img: &mut PhotonImage, text: &str, x: u32, y:u32, font: &str, font_size: f32, rgb: &Rgb) {
        
    let mut image = helpers::dyn_image_from_raw(&img).to_rgba();

    let mut image2 : DynamicImage = DynamicImage::new_luma8(
            image.width(), image.height());

    // include_bytes! only takes a string literal
    let font_vec = match font {
        "Roboto-Regular" => Vec::from(include_bytes!("../fonts/Roboto-Regular.ttf") as &[u8]),
        "Lato-Regular" => Vec::from(include_bytes!("../fonts/Lato-Regular.ttf") as &[u8]),
        "Lato-Bold" => Vec::from(include_bytes!("../fonts/Lato-Bold.ttf") as &[u8]),
        "BebasKai" => Vec::from(include_bytes!("../fonts/BebasKai.ttf") as &[u8]),
        "Oswald-Regular" => Vec::from(include_bytes!("../fonts/Oswald-Regular.ttf") as &[u8]),
        "Norwester" => Vec::from(include_bytes!("../fonts/Norwester.ttf") as &[u8]),
        "Montserrat-Regular" => Vec::from(include_bytes!("../fonts/Montserrat-Regular.ttf") as &[u8]),
        "CaviarDreams" => Vec::from(include_bytes!("../fonts/CaviarDreams.ttf") as &[u8]),
        "Roboto-Light" => Vec::from(include_bytes!("../fonts/Roboto-Light.ttf") as &[u8]),
        "Roboto-Bold" => Vec::from(include_bytes!("../fonts/Roboto-Bold.ttf") as &[u8]),
        "Roboto-Black" => Vec::from(include_bytes!("../fonts/Roboto-Black.ttf") as &[u8]),
        "Roboto-Thin" => Vec::from(include_bytes!("../fonts/Roboto-Thin.ttf") as &[u8]),
        _ => Vec::from(include_bytes!("../fonts/Roboto-Bold.ttf") as &[u8])
    };

    let font = FontCollection::from_bytes(font_vec).unwrap().into_font().unwrap();
    let height = font_size;
    let scale = Scale { x: height * 1.0, y: height };
    let white = Rgb{r: 255, g: 255, b: 255};
    let black = Rgb{r: 0, g: 0, b:0};

    draw_text_mut(&mut image, Rgba([rgb.r as u8, rgb.g as u8, rgb.b as u8, 255u8]), x + 10, y - 10, scale, &font, text);
    let dynimage = image::ImageRgba8(image);
    img.raw_pixels = dynimage.raw_pixels();
    }

/// Draw text onto an image with a border around the text.
///
/// ### Arguments
/// * `img` - Mutable reference to a PhotonImage.
/// * `text` - Text string to be drawn.
/// * `x` - X-coordinate of top corner of text.
/// * `y` - Y coordinae of top corner of text.
#[wasm_bindgen]
pub fn draw_text_with_border(img: &mut PhotonImage, text: &str, x: u32, y: u32) {
    let mut image = helpers::dyn_image_from_raw(&img).to_rgba();

    let mut image2 : DynamicImage = DynamicImage::new_luma8(
        image.width(), image.height());

    let font = Vec::from(include_bytes!("../fonts/Roboto-Black.ttf") as &[u8]);
    let font = FontCollection::from_bytes(font).unwrap().into_font().unwrap();
    let height = 90f32;
    let scale = Scale { x: height * 1.0, y: height };
    draw_text_mut(&mut image2, Rgba([255u8, 255u8, 255u8, 255u8]), x, y, scale, &font, text);

    let mut image2 = image2.to_luma();
    dilate_mut(&mut image2, Norm::LInf, 14u8);

    // Add a border to the text.
    for x in 0..image2.width() {
        for y in 0..image2.height() {
            let pixval = 255 - image2.get_pixel(x, y).data[0];
            if pixval != 255 {
                let new_pix = Rgba([234, 23, 123, 255]);
                image.put_pixel(x, y, new_pix);
            }
        }
    }
    // pink
    // draw_text_mut(&mut image, Rgba([244u8, 36u8, 154u8, 255u8]), x + 10, y - 10, scale, &font, text);

    draw_text_mut(&mut image, Rgba([193u8, 255u8, 255u8, 255u8]), x + 10, y - 10, scale, &font, text);

    let dynimage = image::ImageRgba8(image);
    img.raw_pixels = dynimage.raw_pixels();
}

/// Draw vertical text onto an image.
/// 
/// This is done by drawing the text horizontally onto an image, 
/// then rotating this image by 90 degrees.
///
/// ### Arguments
/// * `img` - Mutable reference to a PhotonImage.
/// * `text` - Text string to be drawn.
/// * `x` - X-coordinate of top corner of text.
/// * `y` - Y coordinae of top corner of text.
/// * `font` - Font name. Fonts available include Roboto-Regular, BebasKai, Roboto-Light, among many others. 
/// Full list of fonts available coming soon. 
/// * `font_size`: f32 that represents the font's size.
/// * `rgb`: Rgb text color.
#[wasm_bindgen]
pub fn draw_vertical_text(img: &mut PhotonImage, text: &str, x: u32, y:u32, font: &str, font_size: f32, direction: &str, rgb: &Rgb) {
        
    let mut image = helpers::dyn_image_from_raw(&img).to_rgba();
    
    // Since the image will be rotated, the height of the container image will be the width of the 
    // text image.
    let height = img.width;
    let width = img.height;

    let mut image2 : DynamicImage = DynamicImage::new_rgba8(height / 2, width / 4);

    // include_bytes! only takes a string literal
    let font_vec = match font {
        "Roboto-Regular" => Vec::from(include_bytes!("../fonts/Roboto-Regular.ttf") as &[u8]),
        "Lato-Regular" => Vec::from(include_bytes!("../fonts/Lato-Regular.ttf") as &[u8]),
        "Lato-Bold" => Vec::from(include_bytes!("../fonts/Lato-Bold.ttf") as &[u8]),
        "BebasKai" => Vec::from(include_bytes!("../fonts/BebasKai.ttf") as &[u8]),
        "CaviarDreams" => Vec::from(include_bytes!("../fonts/CaviarDreams.ttf") as &[u8]),
        "Roboto-Light" => Vec::from(include_bytes!("../fonts/Roboto-Light.ttf") as &[u8]),
        "Roboto-Bold" => Vec::from(include_bytes!("../fonts/Roboto-Bold.ttf") as &[u8]),
        "Roboto-Black" => Vec::from(include_bytes!("../fonts/Roboto-Black.ttf") as &[u8]),
        "Roboto-Thin" => Vec::from(include_bytes!("../fonts/Roboto-Thin.ttf") as &[u8]),
        _ => Vec::from(include_bytes!("../fonts/Roboto-Bold.ttf") as &[u8])
    };

    let font = FontCollection::from_bytes(font_vec).unwrap().into_font().unwrap();
    let scale = Scale { x: font_size * 1.0, y: font_size };
    let white = Rgb{r: 255, g: 255, b: 255};
    let black = Rgb{r: 0, g: 0, b:0};
    draw_text_mut(&mut image2, Rgba([rgb.r as u8, rgb.g as u8, rgb.b as u8, 255u8]), 10, 10, scale, &font, &text);


    let mut image2 = image2.to_rgba();

    // draw_text_mut(&mut image, Rgba([rgb.r as u8, rgb.g as u8, rgb.b as u8, 255u8]), x + 10, y - 10, scale, &font, text);
    
    let mut container_img = helpers::dyn_image_from_raw(&img);

    let rotated_img = match direction {
        "left" => rotate90(&image2),
        _ => rotate270(&image2)
    };

    let image2 = image::ImageRgba8(rotated_img);

    image::imageops::overlay(&mut container_img, &image2, x, y);

    img.raw_pixels = container_img.raw_pixels();
}

