extern crate image;
use image::{GenericImage, GenericImageView, Rgba, DynamicImage, RgbaImage};
extern crate imageproc;
extern crate rusttype;
use wasm_bindgen::prelude::*;
use imageproc::drawing::draw_text_mut;
use imageproc::morphology::dilate_mut;
use imageproc::distance_transform::Norm;
use rusttype::{FontCollection, Scale};
use crate::{PhotonImage, helpers, Rgb, LinSrgba, Gradient, Lch, Srgba};
use image::FilterType::Nearest;
use palette::encoding::pixel::Pixel;

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
