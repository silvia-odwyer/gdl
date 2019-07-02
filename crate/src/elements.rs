extern crate image;
use image::{GenericImage, GenericImageView, Rgba, RgbaImage};
extern crate imageproc;
extern crate rusttype;
use imageproc::drawing::draw_text_mut;
use crate::{PhotonImage, Rgb, LinSrgba, Gradient, Lch, Srgba};
use palette::encoding::pixel::Pixel;
use imageproc::drawing::draw_filled_rect_mut;
use imageproc::rect::Rect;
use crate::text::draw_text;
use crate::helpers;

/// Draw a solid rectangle with a given background colour. 
pub fn draw_solid_rect(mut img: &mut PhotonImage, background_color: &Rgb, height: u32, width: u32, x_pos: i32, y_pos: i32) {
    let mut image = helpers::dyn_image_from_raw(&img).to_rgba();
    draw_filled_rect_mut(&mut image, 
                        Rect::at(x_pos, y_pos).of_size(width, height), 
                        Rgba([background_color.r, background_color.g, 
                        background_color.b, 255u8]));
    let dynimage = image::ImageRgba8(image);
    img.raw_pixels = dynimage.raw_pixels();
}

/// Draw a solid rectangle with text placed in-centre.
pub fn draw_rect_text(mut img: &mut PhotonImage, text: &str, background_color: &Rgb, height: u32, width: u32, x_pos: i32, y_pos: i32) {
    draw_solid_rect(&mut img, &background_color, height as u32, width as u32, x_pos, y_pos);      
    let rgb_white = Rgb { r: 255, g: 255, b: 255};
    draw_text(&mut img, text, (x_pos as f32 + (width as f32 * 0.05)) as u32, (y_pos + 10) as u32, "Roboto-Bold", 30.0, &rgb_white);
}

/// Draw a rectangle filled with a gradient.
pub fn draw_gradient_rect(img: &mut PhotonImage, height: u32, width: u32, x_pos: u32, y_pos: u32) {
    let mut image = helpers::dyn_image_from_raw(&img).to_rgba();

    let rect = create_gradient(width, height);
    let rect = helpers::dyn_image_from_raw(&rect).to_rgba();
        
    image::imageops::overlay(&mut image, &rect, x_pos, y_pos);

    let dynimage = image::ImageRgba8(image);
    img.raw_pixels = dynimage.raw_pixels();
}

/// Create a gradient element in the shape of a Rect.
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
// GRADIENT COLORS 
// #3494e6 → #ec6ead: 52, 148, 230 -> 236, 110, 173
// #67b26f → #4ca2cd: 103, 178, 111 -> 76 162 205
// #ee0979 →  #ff6a00: 238, 9, 121 -> 255, 106, 0
// #ef32d9 → #89fffd: 239, 50, 217 -> 137 255 253
// #7f7fd5 → #86a8e7 → #91eae4: 127, 127, 213 -> 134, 168, 231 -> 145 234 228  
// Telegram:  #1c92d2 → #f2fcfe 
// Digital Water:  #74ebd5 → #acb6e5 
// Hydrogen:  #667db6 →  #0082c8 →  #0082c8 →  #667db6
// Blue Coral:  #36d1dc →  #5b86e5 
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
        _ => Gradient::new(vec![
            LinSrgba::new(0.2039, 0.5803, 0.90196, 1.0),
            LinSrgba::new(0.40392156, 0.69803921, 0.43529 , 1.0),
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