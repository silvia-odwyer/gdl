/// Add shapes and other elements to images.

extern crate image;
use image::{GenericImage, GenericImageView, DynamicImage, RgbaImage};
extern crate imageproc;
extern crate rusttype;
use imageproc::drawing::*;
use crate::{Rgb, LinSrgba, Gradient, Lch, Srgba, Rgba};
use palette::encoding::pixel::Pixel;
use imageproc::rect::Rect;
use crate::text::draw_text;
use crate::helpers;
use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use imageproc::pixelops::interpolate;

/// Draw a solid rectangle with a given background colour. 
/// 
/// # Arguments
/// * `img` - A mutable ref to a DynamicImage.
/// * `background_color` - Rgb color of rectangle.
/// * `width` - u32 - Desired width of rectangle.
/// * `height` - u32 - Desired height of rectangle.
/// * `x_pos` - X-coordinate of top corner of rectangle on `img`
/// * `y_pos` - y-coordinate of top corner of rectangle on `img`
#[wasm_bindgen]
pub fn draw_solid_rect(img: &mut DynamicImage, background_color: &Rgb, width: u32, height: u32, x_pos: i32, y_pos: i32) {    
    draw_filled_rect_mut(img, 
                        Rect::at(x_pos, y_pos).of_size(width, height), 
                        Rgba([background_color.r, background_color.g, 
                        background_color.b, 255u8]));

}

/// Draw an opaque rectangle, where the opacity is set to a certain u8 value. 
/// 
/// * `img` - A mutable ref to a DynamicImage.
/// * `background_color` - Rgb color of rectangle.
/// * `opacity` - The opacity of the rectangle.
/// * `width` - u32 - Desired width of rectangle.
/// * `height` - u32 - Desired height of rectangle.
/// * `x_pos` - X-coordinate of top corner of rectangle on `img`
/// * `y_pos` - y-coordinate of top corner of rectangle on `img`
#[wasm_bindgen]
pub fn draw_opaque_rect(img: &mut DynamicImage, background_color: &Rgb, opacity: u8, width: u32, height: u32, x_pos: i32, y_pos: i32) {
    
    draw_filled_rect_mut(img, 
                        Rect::at(x_pos, y_pos).of_size(width, height), 
                        Rgba([background_color.r, background_color.g, 
                        background_color.b, opacity]));
}

/// Draw a triangle.
/// 
/// /// * `img` - A mutable ref to a DynamicImage.
/// * `triangle` - Triangle struct.
#[wasm_bindgen]
pub fn draw_triangle(img: &mut DynamicImage, triangle: Triangle) {
    let point = Point::new(triangle.x1, triangle.y1);
    let point2 = Point::new(triangle.x2, triangle.y2);
    let point3 = Point::new(triangle.x3, triangle.y3);

    let points = vec![point, point2, point3];

    draw_convex_polygon_mut(img, 
                        points.as_slice(), 
                        Rgba([triangle.background_color.r, triangle.background_color.g, 
                        triangle.background_color.b, 255u8]));
}

/// Draw an equilateral triangle.
/// 
/// Not represented by a Triangle struct, because all sides are equal, and only one value should be entered.
/// 
/// * `img` - A mutable ref to a DynamicImage.
/// * `side_len` - Side of the equilateral triangle, which will constitute all 3 sides.
/// * `x_pos` - X-coordinate of top point of triangle on `img`
/// * `y_pos` - y-coordinate of top point of triangle on `img`
#[wasm_bindgen]
pub fn draw_equilateral_triangle(img: &mut DynamicImage, side_len: u32, x_pos: i32, y_pos: i32, background_color: &Rgb) {

    let point = Point::new(x_pos, y_pos);
    let point2 = Point::new(x_pos + side_len as i32, y_pos);
    let point3 = Point::new(x_pos + (side_len / 2) as i32, y_pos * 3);

    let points = vec![point, point2, point3];

    draw_convex_polygon_mut(img, 
                        points.as_slice(), 
                        Rgba([background_color.r, background_color.g, 
                            background_color.b, 255u8]));
}

/// Draw a solid rectangle with text placed in-centre.
/// 
/// # Arguments
/// * `img` - A mutable ref to a DynamicImage.
/// * `text` - Text to be placed inside the rectangle.
/// * `background_color` - Rgb color of rectangle.
/// * `width` - u32 - Desired width of rectangle.
/// * `height` - u32 - Desired height of rectangle.
/// * `x_pos` - X-coordinate of top corner of rectangle on `img`
/// * `y_pos` - y-coordinate of top corner of rectangle on `img`
#[wasm_bindgen]
pub fn draw_rect_text(img: &mut DynamicImage, text: &str, background_color: &Rgb, height: u32, width: u32, x_pos: i32, y_pos: i32) {
    draw_solid_rect(img, &background_color, height as u32, width as u32, x_pos, y_pos);      
    let rgb_white = Rgb { r: 255, g: 255, b: 255};
    draw_text(img, text, (x_pos as f32 + (width as f32 * 0.05)) as u32, (y_pos + 10) as u32, "Roboto-Bold", 30.0, &rgb_white);
}

/// Draw a solid rectangle with a given background colour. 
// pub fn draw_diamond(mut img: &mut DynamicImage, background_color: &Rgb, height: u32, width: u32, x_pos: i32, y_pos: i32) {
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
/// * `img` - A mutable ref to a DynamicImage.
/// * `text` - Text to be placed inside the rectangle.
/// * `background_color` - Rgb color of rectangle.
/// * `width` - u32 - Desired width of gradient rectangle.
/// * `height` - u32 - Desired height of gradient rectangle.
/// * `x_pos` - X-coordinate of top corner of rectangle on `img`
/// * `y_pos` - y-coordinate of top corner of rectangle on `img`
#[wasm_bindgen]
pub fn draw_gradient_rect(img: &mut DynamicImage, height: u32, width: u32, x_pos: u32, y_pos: u32) {
    let rect = create_gradient(width, height);
        
    image::imageops::overlay(img, &rect, x_pos, y_pos);
}

/// Preset: Draw a gradient rectangle filled with a gradient.
/// 
/// ### Arguments
/// * `img` - A mutable ref to a DynamicImage.
/// * `width` - u32 - Desired width of gradient rectangle.
/// * `height` - u32 - Desired height of gradient rectangle.
/// * `x_pos` - X-coordinate of top corner of rectangle on `img`
/// * `y_pos` - y-coordinate of top corner of rectangle on `img`
/// * `preset_name` - Name of the preset. Examples include "lemongrass", "pink_blue", "pastel_pink", "pastel_mauve"
#[wasm_bindgen]
pub fn draw_preset_rect_gradient(img: &mut DynamicImage, width: u32, height: u32, x_pos: u32, y_pos: u32, preset_name: &str) {
    let rect = create_gradient_preset(width, height, preset_name);
        
    image::imageops::overlay(img, &rect, x_pos, y_pos);
}

/// Draw two rectangles stacked on each other, for added depth.
/// 
/// ### Arguments
/// * `img` - A mutable ref to a DynamicImage.
/// * `background_color1`: Rgb color of first rectangle.
/// * `background_color2` : Rgb color of second rectangle.
/// * `width` - u32 - Desired width of gradient rectangle.
/// * `height` - u32 - Desired height of gradient rectangle.
/// * `x_pos` - X-coordinate of top corner of rectangle on `img`
/// * `y_pos` - y-coordinate of top corner of rectangle on `img`
#[wasm_bindgen]
pub fn draw_stacked_rect(img: &mut DynamicImage, background_color1: &Rgb, background_color2: &Rgb, width: u32, height: u32, x_pos: i32, y_pos: i32) {
    draw_filled_rect_mut(img, 
                        Rect::at(x_pos, y_pos).of_size(width, height), 
                        Rgba([background_color1.r, background_color1.g, 
                        background_color1.b, 255u8]));

    draw_filled_rect_mut(img, 
                        Rect::at(x_pos + 10, y_pos + 10).of_size(width, height), 
                        Rgba([background_color2.r, background_color2.g, 
                        background_color2.b, 255u8]));
}

/// Draw multiple borders stacked on each other, for added depth.
/// 
/// ### Arguments
/// * `img` - A mutable ref to a DynamicImage.
/// * `background_color1`: Rgb color of first rectangle.
/// * `background_color2` : Rgb color of second rectangle.
/// * `width` - u32 - Desired width of gradient rectangle.
/// * `height` - u32 - Desired height of gradient rectangle.
/// * `x_pos` - X-coordinate of top corner of rectangle on `img`
/// * `y_pos` - y-coordinate of top corner of rectangle on `img`
#[wasm_bindgen]
pub fn draw_stacked_borders(img: &mut DynamicImage, background_color: &Rgb, width: u32, height: u32, mut x_pos: i32, mut y_pos: i32) {
    
    
    for i in 0..3 {
        draw_hollow_rect_mut(img, 
                        Rect::at(x_pos, y_pos).of_size(width, height), 
                        Rgba([background_color.r, background_color.g, 
                        background_color.b, 255u8]));

        x_pos -= 40;
        y_pos += 40;
    }
}

/// Create a gradient element in the shape of a Rect.
/// 
/// Returns a DynamicImage.
/// 
/// ### Arguments
/// * `width` - u32 - Desired width of gradient.
/// * `height` - u32 - Desired height of gradient.
#[wasm_bindgen]
pub fn create_gradient(width: u32, height: u32) -> DynamicImage {
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
    return rgba_img;
}

/// Apply a preset gradient by passing in a name. 
///
/// ### Arguments
/// * `width` - u32 - Desired width of rectangle.
/// * `height` - u32 - Desired height of rectangle.
/// * `name` - The preset to be used. Presets available include: pinkblue, lemongrass
#[wasm_bindgen]
pub fn create_gradient_preset(width: u32, height: u32, name: &str) -> DynamicImage {
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
    return rgba_img;
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


/// Triangle struct, which represents the color and co-ordinates
/// of a Triangle. 
#[wasm_bindgen]
#[derive(Debug)]
pub struct Triangle {
    background_color: Rgb,
    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32,
    pub x3: i32,
    pub y3: i32
}

#[wasm_bindgen]
impl Triangle {

    /// Create a new Triangle, with specified co-ordinates for its 3 points, and a background color.
    pub fn new(background_color: Rgb, x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32) -> Triangle {
        return Triangle {background_color: background_color, x1: x1, y1: y1, x2: x2, y2: y2, x3: x3, y3: y3};
    }

    pub fn background_color(self) -> Rgb {
        return self.background_color
    }
}