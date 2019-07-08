/// Resize images to specific sizes/for various social media platforms.

extern crate image;
extern crate imageproc;
extern crate rusttype;
use wasm_bindgen::prelude::*;
use crate::{PhotonImage, helpers};

/// Resize an image for a particular format on social media.
/// Available formats include: pinterest, fb_ad, fb_post, instagram_post, twitter_header, linkedin_banner
/// 
/// # Arguments
/// * `img` - A mutable ref to a PhotonImage.
/// * `type` - Social media format. The available types are shown above.
/// ### Example
/// ```
/// resize_socialmedia(&mut img, "linkedin_banner");
/// ```
#[wasm_bindgen]
pub fn resize_socialmedia(img: &PhotonImage, format: &str) -> PhotonImage {
    let sampling_filter = image::FilterType::Nearest;
    let dynimage = helpers::dyn_image_from_raw(&img);
    let (width, height) = match format {
            "linkedin_banner" => (1400, 425),
            "pinterest" => (735, 1102),
            "fb_ad" => (1200, 628), 
            "fb_post" => (940, 788),
            "instagram_post" => (1080, 1080),
            "twitter_post" => (1024, 512),
            "twitter_header" => (1500, 500),
            _ => (192, 120)
    };
    let resized_img = image::ImageRgba8(image::imageops::resize(&dynimage, width, height, sampling_filter));
    let raw_pixels = resized_img.raw_pixels();
    return PhotonImage { raw_pixels: raw_pixels, width: width, height: height};
}

/// Resizes each image in a vec of PhotonImages to the desired social media format.
pub fn resize_socialmedia_vec(imgs: Vec<PhotonImage>, format: &str) -> Vec<PhotonImage>{
    let mut resized_imgs = vec![];
    for img in imgs {
        let resized_img = resize_socialmedia(&img, format);
        resized_imgs.push(resized_img);
    }
    return resized_imgs;
}

/// Resizes each image in a vec of PhotonImages to each of
/// the available social media formats, and a vec of all new images is returned. 
pub fn resize_socialmedia_all(img: &PhotonImage) -> Vec<PhotonImage> {
    let formats = ["linkedin_banner", "pinterest", "fb_ad", "fb_post", "instagram_post", "twitter_post", "twitter_post", "twitter_header"];
    let mut resized_imgs = vec![];
    for format in &formats {
        let new_img = resize_socialmedia(&img, format);
        resized_imgs.push(new_img)
    }
    return resized_imgs;
}