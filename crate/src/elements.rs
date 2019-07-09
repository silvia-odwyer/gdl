/// Add shapes and other elements to images.

extern crate image;
use image::{GenericImage, GenericImageView, DynamicImage, ImageBuffer, RgbaImage};
extern crate imageproc;
extern crate rusttype;
use imageproc::drawing::*;
use crate::{PhotonImage, Rgb, LinSrgba, Gradient, Lch, Srgba, Rgba, Triangle};
use palette::encoding::pixel::Pixel;
use imageproc::rect::Rect;
use crate::text::draw_text;
use crate::helpers;
use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;

/// Draw a solid rectangle with a given background colour. 
/// 
/// # Arguments
/// * `img` - A mutable ref to a PhotonImage.
/// * `background_color` - Rgb color of rectangle.
/// * `width` - u32 - Desired width of rectangle.
/// * `height` - u32 - Desired height of rectangle.
/// * `x_pos` - X-coordinate of top corner of rectangle on `img`
/// * `y_pos` - y-coordinate of top corner of rectangle on `img`
#[wasm_bindgen]
pub fn draw_solid_rect(mut img: &mut PhotonImage, background_color: &Rgb, width: u32, height: u32, x_pos: i32, y_pos: i32) {
    let mut image = helpers::dyn_image_from_raw(&img).to_rgba();
    draw_filled_rect_mut(&mut image, 
                        Rect::at(x_pos, y_pos).of_size(width, height), 
                        Rgba([background_color.r, background_color.g, 
                        background_color.b, 255u8]));
    let dynimage = image::ImageRgba8(image);
    img.raw_pixels = dynimage.raw_pixels();
}

#[wasm_bindgen]
pub fn draw_opaque_rect(mut img: &mut PhotonImage, background_color: &Rgb, opacity: u8, width: u32, height: u32, x_pos: i32, y_pos: i32) {
    let mut image = helpers::dyn_image_from_raw(&img).to_rgba();
    draw_filled_rect_mut(&mut image, 
                        Rect::at(x_pos, y_pos).of_size(width, height), 
                        Rgba([background_color.r, background_color.g, 
                        background_color.b, opacity]));
    let dynimage = image::ImageRgba8(image);
    img.raw_pixels = dynimage.raw_pixels();
}

/// Draw a triangle.
#[wasm_bindgen]
pub fn draw_triangle(mut img: &mut PhotonImage, triangle: Triangle) {
    let mut image = helpers::dyn_image_from_raw(&img).to_rgba();

    let point = Point::new(triangle.x1, triangle.y1);
    let point2 = Point::new(triangle.x2, triangle.y2);
    let point3 = Point::new(triangle.x3, triangle.y3);

    let points = vec![point, point2, point3];

    draw_convex_polygon_mut(&mut image, 
                        points.as_slice(), 
                        Rgba([triangle.background_color.r, triangle.background_color.g, 
                        triangle.background_color.b, 255u8]));
    
    let dynimage = image::ImageRgba8(image);
    img.raw_pixels = dynimage.raw_pixels();
}

/// Draw a solid rectangle with text placed in-centre.
/// 
/// # Arguments
/// * `img` - A mutable ref to a PhotonImage.
/// * `text` - Text to be placed inside the rectangle.
/// * `background_color` - Rgb color of rectangle.
/// * `width` - u32 - Desired width of rectangle.
/// * `height` - u32 - Desired height of rectangle.
/// * `x_pos` - X-coordinate of top corner of rectangle on `img`
/// * `y_pos` - y-coordinate of top corner of rectangle on `img`
#[wasm_bindgen]
pub fn draw_rect_text(mut img: &mut PhotonImage, text: &str, background_color: &Rgb, height: u32, width: u32, x_pos: i32, y_pos: i32) {
    draw_solid_rect(&mut img, &background_color, height as u32, width as u32, x_pos, y_pos);      
    let rgb_white = Rgb { r: 255, g: 255, b: 255};
    draw_text(&mut img, text, (x_pos as f32 + (width as f32 * 0.05)) as u32, (y_pos + 10) as u32, "Roboto-Bold", 30.0, &rgb_white);
}

/// Draw a solid rectangle with a given background colour. 
// pub fn draw_diamond(mut img: &mut PhotonImage, background_color: &Rgb, height: u32, width: u32, x_pos: i32, y_pos: i32) {
//     let mut image = helpers::dyn_image_from_raw(&img).to_rgba();
//     let mut image2 : ImageBuffer = ImageBuffer::new(height, width);

//     draw_filled_rect_mut(&mut image2, 
//                         Rect::at(0, 0).of_size(width, height), 
//                         Rgba([background_color.r, background_color.g, 
//                         background_color.b, 255u8]));

//     image::imageops::overlay(&mut image, &image2, x_pos as u32, y_pos as u32);
         
//     let dynimage = image::ImageRgba8(image);
//     img.raw_pixels = dynimage.raw_pixels();
// }


/// Draw a rectangle filled with a gradient.
/// 
/// ### Arguments
/// * `img` - A mutable ref to a PhotonImage.
/// * `text` - Text to be placed inside the rectangle.
/// * `background_color` - Rgb color of rectangle.
/// * `width` - u32 - Desired width of gradient rectangle.
/// * `height` - u32 - Desired height of gradient rectangle.
/// * `x_pos` - X-coordinate of top corner of rectangle on `img`
/// * `y_pos` - y-coordinate of top corner of rectangle on `img`
#[wasm_bindgen]
pub fn draw_gradient_rect(img: &mut PhotonImage, height: u32, width: u32, x_pos: u32, y_pos: u32) {
    let mut image = helpers::dyn_image_from_raw(&img).to_rgba();

    let rect = create_gradient(width, height);
    let rect = helpers::dyn_image_from_raw(&rect).to_rgba();
        
    image::imageops::overlay(&mut image, &rect, x_pos, y_pos);

    let dynimage = image::ImageRgba8(image);
    img.raw_pixels = dynimage.raw_pixels();
}

/// Preset: Draw a gradient rectangle filled with a gradient.
/// 
/// ### Arguments
/// * `img` - A mutable ref to a PhotonImage.
/// * `width` - u32 - Desired width of gradient rectangle.
/// * `height` - u32 - Desired height of gradient rectangle.
/// * `x_pos` - X-coordinate of top corner of rectangle on `img`
/// * `y_pos` - y-coordinate of top corner of rectangle on `img`
/// * `preset_name` - Name of the preset. Examples include "lemongrass", "pink_blue", "pastel_pink", "pastel_mauve"
#[wasm_bindgen]
pub fn draw_preset_rect_gradient(img: &mut PhotonImage, width: u32, height: u32, x_pos: u32, y_pos: u32, preset_name: &str) {
    let mut image = helpers::dyn_image_from_raw(&img).to_rgba();

    let rect = create_gradient_preset(width, height, preset_name);
    let rect = helpers::dyn_image_from_raw(&rect).to_rgba();
        
    image::imageops::overlay(&mut image, &rect, x_pos, y_pos);

    let dynimage = image::ImageRgba8(image);
    img.raw_pixels = dynimage.raw_pixels();
}

/// Draw two rectangles stacked on each other, for added depth.
/// 
/// ### Arguments
/// * `img` - A mutable ref to a PhotonImage.
/// * `background_color1`: Rgb color of first rectangle.
/// * `background_color2` : Rgb color of second rectangle.
/// * `width` - u32 - Desired width of gradient rectangle.
/// * `height` - u32 - Desired height of gradient rectangle.
/// * `x_pos` - X-coordinate of top corner of rectangle on `img`
/// * `y_pos` - y-coordinate of top corner of rectangle on `img`
#[wasm_bindgen]
pub fn draw_stacked_rect(mut img: &mut PhotonImage, background_color1: &Rgb, background_color2: &Rgb, width: u32, height: u32, x_pos: i32, y_pos: i32) {
    let mut image = helpers::dyn_image_from_raw(&img).to_rgba();
    draw_filled_rect_mut(&mut image, 
                        Rect::at(x_pos, y_pos).of_size(width, height), 
                        Rgba([background_color1.r, background_color1.g, 
                        background_color1.b, 255u8]));

    draw_filled_rect_mut(&mut image, 
                        Rect::at(x_pos + 10, y_pos + 10).of_size(width, height), 
                        Rgba([background_color2.r, background_color2.g, 
                        background_color2.b, 255u8]));

    let dynimage = image::ImageRgba8(image);
    img.raw_pixels = dynimage.raw_pixels();
}

/// Create a gradient element in the shape of a Rect.
/// 
/// Returns a PhotonImage.
/// 
/// ### Arguments
/// * `width` - u32 - Desired width of gradient.
/// * `height` - u32 - Desired height of gradient.
#[wasm_bindgen]
pub fn create_gradient(width: u32, height: u32) -> PhotonImage {
    let mut image = RgbaImage::new(width, height);

    // Create a gradient.
    let grad1 = Gradient::new(vec![
        LinSrgba::new(1.0, 0.1, 0.1, 1.0),
        LinSrgba::new(0.1, 0.1, 1.0, 1.0),
        LinSrgba::new(0.1, 1.0, 0.1, 1.0),
    ]);

    let grad3 = Gradient::new(vec![
        Lch::from(LinSrgba::new(1.0, 0.1, 0.1, 1.0)),
        Lch::from(LinSrgba::new(0.1, 0.1, 1.0, 1.0)),
        Lch::from(LinSrgba::new(0.1, 1.0, 0.1, 1.0)),
    ]);

    for (i, c1) in grad1
        .take(width as usize)
        .enumerate()
    {
        let c1 = Srgba::from_linear(c1).into_format().into_raw();
        {
            let mut sub_image = image.sub_image(i as u32, 0, 1, height);
            let (width, height) = sub_image.dimensions();
            for x in 0..width {
                for y in 0..height {
                    sub_image.put_pixel(x, y, image::Rgba {
                        data: c1
                    });
                }
            }
        }
    }
    let rgba_img = image::ImageRgba8(image);
    let raw_pixels = rgba_img.raw_pixels();
    return PhotonImage { raw_pixels: raw_pixels, width: width, height: height};
}

/// Apply a preset gradient by passing in a name. 
///
/// ### Arguments
/// * `width` - u32 - Desired width of rectangle.
/// * `height` - u32 - Desired height of rectangle.
/// * `name` - The preset to be used. Presets available include: pinkblue, lemongrass
#[wasm_bindgen]
pub fn create_gradient_preset(width: u32, height: u32, name: &str) -> PhotonImage {
    let mut image = RgbaImage::new(width, height);

    let gradient = match name {
        "pinkblue" => Gradient::new(vec![
            LinSrgba::new(0.2039, 0.5803, 0.90196, 1.0),
            LinSrgba::new(0.92549, 0.431372549, 0.6784313 , 1.0),
        ]),
        "lemongrass" => Gradient::new(vec![
            LinSrgba::new(0.2039, 0.5803, 0.90196, 1.0),
            LinSrgba::new(0.40392156, 0.69803921, 0.43529 , 1.0),
        ]),
        "pink_pastel" => Gradient::new(vec![
            LinSrgba::new(0.93725, 0.19607, 0.85098, 1.0),
            LinSrgba::new(0.537254, 1.0, 0.9921568, 1.0),
        ]),
        "mauve_pastel" => Gradient::new(vec![
            LinSrgba::new(0.498039, 0.498039, 0.835294, 1.0),
            LinSrgba::new(0.537254, 0.6588235, 0.905882, 1.0),
        ]),
        _ => Gradient::new(vec![
            LinSrgba::new(0.2039, 0.5803, 0.90196, 1.0),
            LinSrgba::new(0.52549, 0.69803921, 0.43529 , 1.0),
        ])

    };

    for (i, c1) in gradient
        .take(width as usize)
        .enumerate()
    {
        let c1 = Srgba::from_linear(c1).into_format().into_raw();
        {
            let mut sub_image = image.sub_image(i as u32, 0, 1, height);
            let (width, height) = sub_image.dimensions();
            for x in 0..width {
                for y in 0..height {
                    sub_image.put_pixel(x, y, image::Rgba {
                        data: c1
                    });
                }
            }
        }
    }
    let rgba_img = image::ImageRgba8(image);
    let raw_pixels = rgba_img.raw_pixels();
    return PhotonImage { raw_pixels: raw_pixels, width: width, height: height};
}

// #[wasm_bindgen]
// pub fn draw_dyn_rect(image: &mut DynamicImage, background_color: &Rgb, height: u32, width: u32, x_pos: i32, y_pos: i32) {
//     let mut image = image.to_rgba();
//     draw_filled_rect_mut(&mut image, 
//                         Rect::at(x_pos, y_pos).of_size(width, height), 
//                         Rgba([background_color.r, background_color.g, 
//                         background_color.b, 255u8]));
// }
 
// GRADIENT COLORS 
// #67b26f → #4ca2cd: 103, 178, 111 -> 76 162 205
// #ee0979 →  #ff6a00: 238, 9, 121 -> 255, 106, 0


// Telegram:  #1c92d2 → #f2fcfe 
// Digital Water:  #74ebd5 → #acb6e5 
// Hydrogen:  #667db6 →  #0082c8 →  #0082c8 →  #667db6
// Blue Coral:  #36d1dc →  #5b86e5 