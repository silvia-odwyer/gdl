extern crate image;
use image::{GenericImageView, DynamicImage};
extern crate imageproc;
extern crate rusttype;
use wasm_bindgen::prelude::*;
use crate::{PhotonImage, helpers, Rgb};
use crate::text::*;
use crate::elements::*;

/// Preset: Centre text, with background image.
#[wasm_bindgen]
pub fn centre_text(mut background_img: &mut PhotonImage, main_text: &str, width: u32, height: u32) {
    let width = &background_img.width / 2;
    let height = &background_img.height / 2;
    draw_text(&mut background_img, main_text, width - 100, height, "Roboto-Bold", 150.0);
}
