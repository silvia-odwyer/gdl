/// Create image collages.

extern crate image;
use image::{GenericImageView, DynamicImage, Rgba};
extern crate imageproc;
extern crate rusttype;
use wasm_bindgen::prelude::*;
use crate::{PhotonImage, helpers, Rgb};
use crate::text::*;
use crate::elements::*;
use imageproc::drawing::draw_filled_rect_mut;
use imageproc::rect::Rect;

/// Two grid collage.
/// # Arguments
/// * `img` - A mutable ref to a PhotonImage.
/// * `img2` - A mutable ref to a PhotonImage.
/// * `width` - u32 - Desired width of final graphic 
/// * `height` - u32 - Desired height of final graphic
#[wasm_bindgen]
pub fn two_grid(photon_img: &PhotonImage, photon_img2: &PhotonImage, width: u32, height: u32) -> PhotonImage {
    // Convert all photon images to DynamicImages, for interop with the image crate.
    let photon_imgs = vec![photon_img, photon_img2];
    let mut imgs = to_dyn_img_vec(photon_imgs);

    // distribute the width evenly by allocating the same space to both images
    let img_width = width / 2;
    resize_imgs(&mut imgs, img_width, height);

    let mut container_img : DynamicImage = DynamicImage::new_rgba8(width, height);

    image::imageops::overlay(&mut container_img, &imgs[0], 0, 0);
    image::imageops::overlay(&mut container_img, &imgs[1], img_width, 0);

    // return the collage
    return PhotonImage {raw_pixels: container_img.raw_pixels(), width: container_img.width(), height: container_img.height()};
}

/// Four grid collage.
/// # Arguments
/// * `img` - A mutable ref to a PhotonImage.
/// * `img2` - A mutable ref to a PhotonImage.
/// * `main_text` - Main heading for the graphic.
/// * `width` - u32 - Desired width of final graphic 
/// * `height` - u32 - Desired height of final graphic
#[wasm_bindgen]
pub fn two_grid_text(photon_img: PhotonImage, photon_img2: PhotonImage, width: u32, height: u32) -> PhotonImage {
    let image = helpers::dyn_image_from_raw(&photon_img);
    let image2 = helpers::dyn_image_from_raw(&photon_img2);

    // distribute the width evenly by allocating the same space to both images
    let img_width = width / 2;
    let img_height = height / 2;

    let sampling_filter = image::FilterType::Nearest;
    let image = image::ImageRgba8(image::imageops::resize(&image, img_width, img_height, sampling_filter));
    let image2 = image::ImageRgba8(image::imageops::resize(&image2, img_width, img_height, sampling_filter));

    let mut container_img : DynamicImage = DynamicImage::new_rgba8(width, height);

    image::imageops::overlay(&mut container_img, &image, 0, 0);
    image::imageops::overlay(&mut container_img, &image2, image.width(), image.height());

    let mut photon_img = PhotonImage::new_from_rawpixels(container_img.raw_pixels(), container_img.width(), container_img.height());
    let white = Rgb{r: 255, g: 255, b: 255};
    let black = Rgb{r: 0, g: 0, b:0};
    let lilac = Rgb{r: 204, g: 195, b: 240};
    let yellow = Rgb{ r: 255, g: 226, b: 98};
    draw_solid_rect(&mut photon_img, &yellow, img_height, img_width, image.width() as i32, 0);
    draw_solid_rect(&mut photon_img, &lilac, img_height, img_width, 0, image.height() as i32);
    let rgb_white = Rgb { r: 255, g: 255, b: 255};

    draw_text(&mut photon_img, "Daisies In the Underground", image.width() + 30, img_height / 2, "Roboto-Bold", 30.0, &rgb_white);
    
    return photon_img;

}

/// Split-pane collage, with text on LHS and collage on RHS.
/// 
/// # Arguments
/// * `img` - A mutable ref to a PhotonImage.
/// * `img2` - A mutable ref to a PhotonImage.
/// * `main_text` - Main heading for the graphic.
/// * `width` - u32 - Desired width of final graphic 
/// * `height` - u32 - Desired height of final graphic
#[wasm_bindgen]
pub fn split_imgs_text(photon_img: PhotonImage, photon_img2: PhotonImage, width: u32, height: u32) -> PhotonImage {
    let image = helpers::dyn_image_from_raw(&photon_img);
    let image2 = helpers::dyn_image_from_raw(&photon_img2);

    // distribute the width evenly by allocating the same space to both images
    let img_width = width / 2;
    let img_height = height / 2;

    let sampling_filter = image::FilterType::Nearest;
    let image = image::ImageRgba8(image::imageops::resize(&image, img_width, img_height, sampling_filter));
    let image2 = image::ImageRgba8(image::imageops::resize(&image2, img_width, img_height, sampling_filter));

    let mut container_img : DynamicImage = DynamicImage::new_rgba8(width, height);

    image::imageops::overlay(&mut container_img, &image, img_width, 0);
    image::imageops::overlay(&mut container_img, &image2, image.width(), image.height());

    // return the collage
    let mut photon_img = PhotonImage::new_from_rawpixels(container_img.raw_pixels(), container_img.width(), container_img.height());
    let white = Rgb{r: 255, g: 255, b: 255};

    draw_solid_rect(&mut photon_img, &white, img_height * 2, img_width, 0, 0);

    draw_text(&mut photon_img, "Life Is An Adventure", 45, img_height / 2, "BebasKai", 80.0, &white);
    
    return photon_img;

}

/// Four grid collage.
/// 
/// # Arguments
/// * `img` - A mutable ref to a PhotonImage.
/// * `img2` - A mutable ref to a PhotonImage.
/// * `img3` - A mutable ref to a PhotonImage.
/// * `img4` - A mutable ref to a PhotonImage.
/// * `width` - u32 - Desired width of final graphic 
/// * `height` - u32 - Desired height of final graphic
#[wasm_bindgen]
pub fn four_grid(photon_img: &PhotonImage, photon_img2: &PhotonImage, photon_img3: &PhotonImage, photon_img4: &PhotonImage, width: u32, height: u32) -> PhotonImage {
    let photon_imgs = vec![photon_img, photon_img2, photon_img3, photon_img4];
    let mut imgs = to_dyn_img_vec(photon_imgs);

    // distribute the width evenly by allocating the same space to all images
    let img_width = width / 2;
    let img_height = height / 2;

    resize_imgs(&mut imgs, img_width, img_height);

    let mut container_img : DynamicImage = DynamicImage::new_rgba8(width, height);

    image::imageops::overlay(&mut container_img, &imgs[0], 0, 0);
    image::imageops::overlay(&mut container_img, &imgs[1], img_width, 0);
    image::imageops::overlay(&mut container_img, &imgs[2], 0, img_height);
    image::imageops::overlay(&mut container_img, &imgs[3], img_width, img_height);

    // return the collage
    let photon_img = PhotonImage {raw_pixels: container_img.raw_pixels(), width: container_img.width(), height: container_img.height()};
    return photon_img;
}

/// Create a triple grid collage graphic.
/// 
/// # Arguments
/// * `img` - A mutable ref to a PhotonImage.
/// * `img2` - A mutable ref to a PhotonImage.
/// * `img3` - A mutable ref to a PhotonImage.
/// * `width` - u32 - Desired width of final graphic 
/// * `height` - u32 - Desired height of final graphic
#[wasm_bindgen]
pub fn triple_grid(photon_img: &PhotonImage, photon_img2: &PhotonImage, photon_img3: &PhotonImage, width: u32, height: u32) -> PhotonImage {
    let photon_imgs = vec![photon_img, photon_img2, photon_img3];
    let mut imgs = to_dyn_img_vec(photon_imgs);

    // distribute the width evenly by allocating the same space to both images
    let img_width = width / 3;
    resize_imgs(&mut imgs, img_width, height);

    let mut container_img : DynamicImage = DynamicImage::new_rgba8(width, height);

    image::imageops::overlay(&mut container_img, &imgs[0], 0, 0);
    image::imageops::overlay(&mut container_img, &imgs[1], img_width, 0);
    image::imageops::overlay(&mut container_img, &imgs[2], img_width * 2, 0);

    // return the collage
    return PhotonImage {raw_pixels: container_img.raw_pixels(), width: container_img.width(), height: container_img.height()};
}

/// Four-image collage with a centre square containing text.
/// 
/// # Arguments
/// * `img` - A mutable ref to a PhotonImage.
/// * `img2` - A mutable ref to a PhotonImage.
/// * `img3` - A mutable ref to a PhotonImage.
/// * `img4` - A mutable ref to a PhotonImage.
/// * `text` - Main heading for the graphic.
/// * `width` - u32 - Desired width of final graphic 
/// * `height` - u32 - Desired height of final graphic
#[wasm_bindgen]
pub fn four_grid_center_square(photon_img: &PhotonImage, photon_img2: &PhotonImage, photon_img3: &PhotonImage, photon_img4: &PhotonImage, text: &str, width: u32, height: u32) -> PhotonImage {
    let photon_imgs = vec![photon_img, photon_img2, photon_img3, photon_img4];
    let mut imgs = to_dyn_img_vec(photon_imgs);

    // distribute the width evenly by allocating the same space to both images
    let img_width = width / 2;
    let img_height = height / 2;

    resize_imgs(&mut imgs, img_width, img_height);

    let mut container_img : DynamicImage = DynamicImage::new_rgba8(width, height);

    image::imageops::overlay(&mut container_img, &imgs[0], 0, 0);
    image::imageops::overlay(&mut container_img, &imgs[1], img_width, 0);
    image::imageops::overlay(&mut container_img, &imgs[2], 0, img_height);
    image::imageops::overlay(&mut container_img, &imgs[3], img_width, img_height);

    // return the collage
    let mut photon_img = PhotonImage {raw_pixels: container_img.raw_pixels(), width: container_img.width(), height: container_img.height()};
    let white_rgb = Rgb { r: 255, g: 255, b: 255};
    let black_rgb = Rgb { r: 0, g: 0, b: 0};
    // Draw a square in the center
    
    let mut height_mul: f32 = 0.2;
    let mut word_vec = vec![];
    // Split words
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
    draw_solid_rect(&mut photon_img, &white_rgb, (width as f32 * 0.3) as u32, (height as f32 * 0.8) as u32, (width as f32 * 0.3) as i32, (height as f32 * 0.15) as i32);  
    
     for word in word_vec {
        draw_text(&mut photon_img, word, (width as f32 * 0.32) as u32, (height as f32 * height_mul) as u32, "BebasKai", 100.0, &black_rgb );  
        height_mul += 0.15;
    }
    
    return photon_img;
}

#[wasm_bindgen]
pub fn moodboard(photon_img: &PhotonImage, photon_img2: &PhotonImage, photon_img3: &PhotonImage, photon_img4: &PhotonImage, text: &str, width: u32, height: u32) -> PhotonImage {
    
    // Exclude the first image, since it will have different dimensions when resized.
    let photon_imgs = vec![photon_img2, photon_img3, photon_img4];
    let mut imgs = to_dyn_img_vec(photon_imgs);

    // distribute the width evenly by allocating the same space to both images
    let img_width = width / 4;
    let img_height = height / 4;

    let first_img_width: u32 = width - img_width;
    let first_dyn_img = helpers::dyn_image_from_raw(&photon_img);
    let sampling_filter = image::FilterType::Nearest;

    let image = image::ImageRgba8(image::imageops::resize(&first_dyn_img, first_img_width, (height as f32 * 0.8) as u32, sampling_filter));

    resize_imgs(&mut imgs, img_width, img_height);

    let mut container_img : DynamicImage = DynamicImage::new_rgba8(width, height);

    image::imageops::overlay(&mut container_img, &image, 0, 0);
    image::imageops::overlay(&mut container_img, &imgs[0], first_img_width, 0);
    image::imageops::overlay(&mut container_img, &imgs[1], first_img_width, img_height);
    image::imageops::overlay(&mut container_img, &imgs[2], first_img_width, img_height * 2);

    // return the collage
    let mut photon_img = PhotonImage {raw_pixels: container_img.raw_pixels(), width: container_img.width(), height: container_img.height()};
    let white_rgb = Rgb { r: 255, g: 255, b: 255};
    let black_rgb = Rgb { r: 0, g: 0, b: 0};
    // Draw a square in the center
    
    let mut height_mul: f32 = 0.75;
    
    draw_solid_rect(&mut photon_img, &white_rgb, width as u32, (height as f32 * 0.3) as u32, 0 as i32, (height as f32 * 0.75) as i32);  

    draw_text(&mut photon_img, text, (width as f32 * 0.10) as u32, (height as f32 * height_mul) as u32, "Oswald-Regular", 100.0, &black_rgb );  
    height_mul += 0.15;
    
    return photon_img;
}

#[wasm_bindgen]
pub fn feature_grid(photon_img: &PhotonImage, photon_img2: &PhotonImage, photon_img3: &PhotonImage, main_text: &str, width: u32, height: u32) -> PhotonImage {
    let photon_imgs = vec![photon_img, photon_img2, photon_img3];
    let mut imgs = to_dyn_img_vec(photon_imgs);

    // distribute the width evenly by allocating the same space to all images
    let img_width = width / 2;
    let img_height = height / 2;

    resize_imgs(&mut imgs, img_width, img_height);

    let mut container_img : DynamicImage = DynamicImage::new_rgba8(width, height);
    let white = Rgb{r: 255, g: 255, b: 255};

    draw_filled_rect_mut(&mut container_img, 
                        Rect::at(0, 0).of_size( (width / 2) as u32, (height / 2) as u32), 
                        Rgba([white.r, white.g, 
                        white.b, 255u8]));

    image::imageops::overlay(&mut container_img, &imgs[0], img_width, 0);
    image::imageops::overlay(&mut container_img, &imgs[1], 0, img_height);
    image::imageops::overlay(&mut container_img, &imgs[2], img_width, img_height);

    // return the collage
    let mut photon_img = PhotonImage {raw_pixels: container_img.raw_pixels(), width: container_img.width(), height: container_img.height()};
    let black_rgb = Rgb { r: 0, g: 0, b: 0};
    draw_text(&mut photon_img, main_text, (width as f32 * 0.1) as u32, (height as f32 * 0.3) as u32, "BebasKai", 100.0, &black_rgb );  
    return photon_img;
}

#[wasm_bindgen]
pub fn triple_grid_text(photon_img: &PhotonImage, photon_img2: &PhotonImage, photon_img3: &PhotonImage, text: &str, width: u32, height: u32) -> PhotonImage {
    let photon_imgs = vec![photon_img, photon_img2, photon_img3];
    let mut imgs = to_dyn_img_vec(photon_imgs);

    // distribute the width evenly by allocating the same space to both images
    let img_width = width / 3;
    let img_height = (height as f32 * 0.8) as u32;
    resize_imgs(&mut imgs, img_width, img_height);

    let mut container_img : DynamicImage = DynamicImage::new_rgba8(width, height);
    let white = Rgb{r: 255, g: 255, b: 255};

    image::imageops::overlay(&mut container_img, &imgs[0], 0, 0);
    image::imageops::overlay(&mut container_img, &imgs[1], img_width, 0);
    image::imageops::overlay(&mut container_img, &imgs[2], img_width * 2, 0);

    let black_rgb = Rgb{ r: 0, g: 0, b:0 };


    // return the collage
    let mut photon_img =  PhotonImage {raw_pixels: container_img.raw_pixels(), width: container_img.width(), height: container_img.height()};

    draw_solid_rect(&mut photon_img, &white, width as u32, (height as f32 * 0.3) as u32, 0 as i32, (height as f32 * 0.75) as i32);  
    draw_text(&mut photon_img, text, (width as f32 * 0.05) as u32, (height as f32 * 0.8) as u32, "Montserrat-Regular", 90.0, &black_rgb );  

    return photon_img;
}

#[wasm_bindgen]
pub fn six_grid_text(photon_img: &PhotonImage, photon_img2: &PhotonImage, photon_img3: &PhotonImage,
                    photon_img4: &PhotonImage, photon_img5: &PhotonImage, photon_img6: &PhotonImage, 
                    text: &str, width: u32, height: u32) -> PhotonImage {
    
    let photon_imgs = vec![photon_img, photon_img2, photon_img3, photon_img4, photon_img5, photon_img6];
    let mut imgs = to_dyn_img_vec(photon_imgs);

    // distribute the width evenly by allocating the same space to both images
    let img_width = width / 3;
    let img_height = height / 3;
    resize_imgs(&mut imgs, img_width, img_height);

    let mut container_img : DynamicImage = DynamicImage::new_rgba8(width, height);
    let white = Rgb{r: 255, g: 255, b: 255};

    image::imageops::overlay(&mut container_img, &imgs[0], 0, 0);
    image::imageops::overlay(&mut container_img, &imgs[1], img_width, 0);
    image::imageops::overlay(&mut container_img, &imgs[2], img_width * 2, 0);
    image::imageops::overlay(&mut container_img, &imgs[3], 0, img_height * 2);
    image::imageops::overlay(&mut container_img, &imgs[4], img_width, img_height * 2);
    image::imageops::overlay(&mut container_img, &imgs[5], img_width * 2, img_height * 2);


    let black_rgb = Rgb{ r: 0, g: 0, b:0 };


    // return the collage
    let mut photon_img =  PhotonImage {raw_pixels: container_img.raw_pixels(), width: container_img.width(), height: container_img.height()};

    draw_solid_rect(&mut photon_img, &white, width as u32, img_height, 0 as i32, img_height as i32);  
    draw_text(&mut photon_img, text, (width as f32 * 0.05) as u32, (img_height + (img_height as f32 * 0.3) as u32) as u32, "Montserrat-Regular", 90.0, &black_rgb );  

    return photon_img;
}

fn to_dyn_img_vec(imgs: Vec<&PhotonImage>) -> Vec<DynamicImage> {
    let mut dyn_imgs = vec![];
    for img in imgs {
        let image = helpers::dyn_image_from_raw(&img);
        dyn_imgs.push(image);
    }

    return dyn_imgs
}

// Resize images in a vec, returns a new vec with resized images.
fn resize_imgs(imgs: &mut Vec<DynamicImage>, img_width: u32, img_height: u32) {
    let sampling_filter = image::FilterType::Nearest;

    for i in 0..imgs.len() {
        let item = &imgs[i];
        let image = image::ImageRgba8(image::imageops::resize(item, img_width, img_height, sampling_filter));

        imgs[i] = image;
    }
}