/// Create various backgrounds comprising patterns, gradients, imagery, etc.,

extern crate image;
use image::{ GenericImage, DynamicImage};
extern crate imageproc;
extern crate rusttype;
use wasm_bindgen::prelude::*;
use imageproc::drawing::{draw_text_mut, draw_filled_circle_mut};
use imageproc::morphology::dilate_mut;
use crate::{PhotonImage, helpers, Rgb};
use image::GenericImageView;
use palette::{Lch, Srgb, Srgba, Hue, Gradient, Pixel};
use palette::rgb::LinSrgba;
use image::{ImageBuffer, RgbaImage};
use std::convert::TryInto;
use imageproc::drawing::draw_line_segment_mut;

/// Create a background image containing circles. 
/// 
/// Returns a PhotonImage.
/// 
/// # Arguments
/// * `width` - u32 - Desired width of final graphic 
/// * `height` - u32 - Desired height of final graphic
#[wasm_bindgen]
pub fn circle_background(width: u32, height: u32) -> PhotonImage {
    let background_color = Rgb { r: 190, g: 120, b: 200};
    let mut rgba_img = create_image_from_pixel(background_color, width, height);

    let white_pixel = image::Rgba([123, 10, 180, 255]);
    let circle_radius: i32 = (width as i32 / 10) + 15;

    for x in 0..5 {
        for y in 0..6 {
        
            draw_filled_circle_mut(&mut rgba_img, (x * circle_radius * 2, y * circle_radius * 2), circle_radius, white_pixel);

        }
    }

    return PhotonImage { raw_pixels: rgba_img.raw_pixels(), width: width, height: height}

}

/// Create a background image containing spaced circles 
/// 
/// Returns a PhotonImage.
/// 
/// # Arguments
/// * `width` - u32 - Desired width of final graphic 
/// * `height` - u32 - Desired height of final graphic
#[wasm_bindgen]
pub fn spaced_circle_background(width: u32, height: u32) -> PhotonImage {
    let background_color = Rgb { r: 190, g: 120, b: 200};

    let mut rgba_img = create_image_from_pixel(background_color, width, height);

    let white_pixel = image::Rgba([123, 10, 180, 255]);
    let circle_radius: i32 = (width as i32 / 10) - 35;
    let end: usize = (width / 10).try_into().unwrap();

    for x in 0..8 {
        for y in 0..10 {
            draw_filled_circle_mut(&mut rgba_img, (x * circle_radius * 3, y * circle_radius * 3) , circle_radius, white_pixel);
        }
    }

    return PhotonImage { raw_pixels: rgba_img.raw_pixels(), width: width, height: height}
}

/// Create a background filled with a solid color of type `Rgb`.
/// Returns a PhotonImage.
/// 
/// # Arguments
/// * `width` - u32 - Desired width of final graphic 
/// * `height` - u32 - Desired height of final graphic
/// * `background_color` - Rgb color the background should comprise of
#[wasm_bindgen]
pub fn solid_background(width: u32, height: u32, background_color: Rgb) -> PhotonImage {
    let rgba_img = create_image_from_pixel(background_color, width, height);
    return PhotonImage{ raw_pixels: rgba_img.raw_pixels(), width: width, height: height};
}

/// Create a lined background.
/// Returns a PhotonImage.
/// 
/// # Arguments
/// * `width` - u32 - Desired width of final graphic 
/// * `height` - u32 - Desired height of final graphic
/// * `background_color` - Rgb color the background should comprise of
#[wasm_bindgen]
pub fn lined_background(width: u32, height: u32, background_color: Rgb) -> PhotonImage {
    let mut rgba_img = create_image_from_pixel(background_color, width, height);
    let line_pixel = image::Rgba([255, 167, 90, 255]);
    
    for y in 0..50 {
        draw_line_segment_mut(&mut rgba_img, (0 as f32, (y * 20) as f32 ), (width as f32, (y * 20) as f32 ), line_pixel);
    }
    return PhotonImage{ raw_pixels: rgba_img.raw_pixels(), width: width, height: height };
}

/// Create a grid background.
/// Returns a PhotonImage.
/// 
/// # Arguments
/// * `width` - u32 - Desired width of final graphic 
/// * `height` - u32 - Desired height of final graphic
/// * `background_color` - Rgb color the background should comprise of.
#[wasm_bindgen]
pub fn grid_background(width: u32, height: u32, background_color: Rgb) -> PhotonImage {
    let mut rgba_img = create_image_from_pixel(background_color, width, height);

    let line_pixel = image::Rgba([255, 167, 90, 255]);
    
    for y in 0..50 {
        draw_line_segment_mut(&mut rgba_img, (0.0, (y * 20) as f32 ), (width as f32, (y * 20) as f32 ), line_pixel);

    }
    for x in 0..50 {
        draw_line_segment_mut(&mut rgba_img, (x as f32 * 50.0, 0.0), (x as f32 * 50.0, height as f32 ), line_pixel);

    }
    return PhotonImage{ raw_pixels: rgba_img.raw_pixels(), width: width, height: height };
}

/// Create a patterned background by overlaying an image in a series of rows and columns.
/// Returns a PhotonImage.
/// 
/// # Arguments
/// * `width` - u32 - Desired width of final graphic 
/// * `height` - u32 - Desired height of final graphic
/// * `background_color` - Rgb color the background should comprise of.
/// * `img` - A PhotonImage to be painted onto the background in a pattern.
#[wasm_bindgen]
pub fn pattern_from_img(width: u32, height: u32, background_color: Rgb, img: PhotonImage) -> PhotonImage {
    let mut rgba_img = create_image_from_pixel(background_color, width, height);
    let mut image = helpers::dyn_image_from_raw(&img).to_rgba();
    let img = image::ImageRgba8(image);

    for x in 0..8 {
        for y in 0..10 {
            image::imageops::overlay(&mut rgba_img, &img, x + img.width() * 3, y * img.height() + 50);
        }
    }
    return PhotonImage{ raw_pixels: rgba_img.raw_pixels(), width: width, height: height };
}

/// Create a gradient background.
/// /// Returns a PhotonImage.
/// 
/// # Arguments
/// * `width` - u32 - Desired width of final graphic 
/// * `height` - u32 - Desired height of final graphic
#[wasm_bindgen]
pub fn gradient_background(width: u32, height: u32) -> PhotonImage {
    let mut image = RgbaImage::new(width, height);
    let grad1 = Gradient::new(vec![
        LinSrgba::new(1.0, 0.1, 0.1, 1.0),
        LinSrgba::new(0.1, 0.1, 1.0, 1.0),
        LinSrgba::new(0.1, 1.0, 0.1, 1.0),
    ]);

    //The same colors and offsets as in grad1, but in a color space where the hue
    // is a component
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

      
fn create_image_from_pixel(background_color: Rgb, width: u32, height: u32, ) -> DynamicImage {
    let pixel = image::Rgba([background_color.r, background_color.g, background_color.b, 255]);
    let image_buffer = ImageBuffer::from_pixel(width, height, pixel);
    let mut rgba_img = image::ImageRgba8(image_buffer);
    return rgba_img;
}