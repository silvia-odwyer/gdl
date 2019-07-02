/// Resize images to specific sizes/for various social media platforms.

extern crate image;
use image::{GenericImageView, DynamicImage};
extern crate imageproc;
extern crate rusttype;
use wasm_bindgen::prelude::*;
use crate::{PhotonImage, helpers, Rgb};
use crate::text::*;
use crate::elements::*;

/// Resize images in a vec, returns a new vec with resized images.
#[wasm_bindgen]
pub fn resize_socialmedia(mut img: &mut PhotonImage, type: &str) {
    let sampling_filter = image::FilterType::Nearest;
    let (width, height) = match name {
            "linkedin_banner" => (1400, 425),
            "pinterest" => (735, 1102),
            "fb_ad" => (1200, 628), 
            "fb_post" => (940, 788),
            "instagram_post" => (1080, 1080),
            "twitter_post" => (1024, 512),
            "twitter_header" => (1500, 500),
            _ => (192, 120)
    };
    img = image::ImageRgba8(image::imageops::resize(img, width, height, sampling_filter));
    
}