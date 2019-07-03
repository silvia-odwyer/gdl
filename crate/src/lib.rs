//! GDL is a graphic design crate that allows developers to rapidly design and create 2D graphics natively and for the web.
//! 
//! This crate contains built-in templates for rapid graphic creation, resizing for social media, and utils for design, but also 
//! provides more low-level access to developers who wish to work with the elements directly, and create their
//! own designs from the elements available.
//! 
//! To view a full demo of templates available, visit the [official website](https://silvia-odwyer.github.io/gdl).
//! 
//! ### WebAssembly Use
//! To allow for universal communication between the core Rust library and WebAssembly, the functions have been generalised to allow for both native and in-browser use. 

use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, ImageData, HtmlCanvasElement};
use wasm_bindgen::Clamped;
use image::{GenericImageView, GenericImage, ImageBuffer, RgbaImage};
use imageproc::drawing::draw_text_mut;
use imageproc::drawing::draw_filled_rect_mut;
use palette::{Lch, Srgb, Srgba, Hue, Gradient};
use palette::rgb::LinSrgba;
use palette::encoding::pixel::Pixel;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// Provides the image's height, width, and contains the image's raw pixels.
/// For use when communicating between JS and WASM, and also natively. 
#[wasm_bindgen]
#[derive(Debug)]
pub struct PhotonImage {
    raw_pixels: Vec<u8>,
    width: u32, 
    height: u32,
}

#[wasm_bindgen]
impl PhotonImage {
    pub fn height(self) -> u32 {
        self.height
    }

    pub fn width(self) -> u32 {
        self.width
    }
    
    pub fn new_from_rawpixels(raw_pixels: Vec<u8>, width: u32, height: u32) -> PhotonImage {
        return PhotonImage {raw_pixels: raw_pixels, width: width, height: height}
    }
    
    pub fn new_from_imgdata(&mut self, img_data: ImageData, width: u32, height: u32) -> PhotonImage {
        let raw_pixels = to_raw_pixels(img_data);
        let new_vec = Vec::new();
        return PhotonImage {raw_pixels: new_vec, width: width, height: height}
    }

    pub fn new_with_background(width: u32, height: u32, background_color: Rgb) -> PhotonImage {
        // create a pixel 
        let pixel =  image::Rgba([background_color.r, background_color.g, background_color.b, 255]);
        let image_buffer = ImageBuffer::from_pixel(width, height, pixel);
        let rgba_img = image::ImageRgba8(image_buffer);

        let raw_pixels = rgba_img.raw_pixels();
        return PhotonImage { raw_pixels: raw_pixels, width: width, height: height};
    }

    pub fn new_with_pattern(width: u32, height: u32, background_color: Rgb) {
        // create a pixel 
        // let pixel =  image::Rgba([background_color.r, background_color.g, background_color.b, 255]);
        // let image_buffer = ImageBuffer::from_pixel(width, height, pixel);
        // let rgba_img = image::ImageRgba8(image_buffer);


        // let raw_pixels = rgba_img.raw_pixels();
        // return PhotonImage { raw_pixels: raw_pixels, width: width, height: height};
    }

    pub fn new_with_gradient(width: u32, height: u32) -> PhotonImage {
        // create a pixel 
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
    
    /// Create a new social media graphic. 
    /// Available types include: linkedin_banner, pinterest, fb_ad, fb_post, instagram_post, 
    /// twitter_header, twitter_post.
    pub fn new_socialmedia_graphic(name: &str) -> PhotonImage {
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
        let new_vec = Vec::new();
        return PhotonImage {raw_pixels: new_vec, width: width, height: height}
    }

    // pub fn word_cloud(&mut self) {
    //     let words = vec!["hello", "bonjour", "hola", "life", "buna", "words", "codes", "freedom", "liberty"];
    //     let mut inc = 20;
    //     let mut inc_y = 30;
    //     for word in words {
    //         self.draw_text(&word, inc, inc_y, "Lato-Regular", 50.0);
    //         inc += 100;
    //         inc_y += 100;
    //     }

    // }

    pub fn raw_pix(self) -> Vec<u8> {
        self.raw_pixels
    }
        
}


/// Provides the image's height, width, and contains the image's raw pixels.
/// For use when communicating between JS and WASM, and also natively. 
#[wasm_bindgen]
#[derive(Debug)]
pub struct ColorScheme {
    main_color: Rgb,
    analogous_swatch: Vec<Lch>,
}

#[wasm_bindgen]
impl ColorScheme {
    pub fn new(main_color: Rgb) -> ColorScheme {
        let distance = 120.0;

        let primary: Lch = Srgb::new(main_color.r, main_color.g, main_color.b)
        .into_format::<f32>()
        .into_linear()
        .into();

        let an_shift = 180.0 - (distance / 2.0);

        let analogous_swatch = vec![
            primary.shift_hue(an_shift), 
            primary.shift_hue(-an_shift)
        ];

        // gen secondary swatch

        
        // gen rectangular swatch
        return ColorScheme {main_color: main_color, analogous_swatch: analogous_swatch}
    }

    /// Create a swatch image in PNG format of the colour swatches generated
    /// from the main colour.
    pub fn create_swatch_image() {

    }
}


/// Provides the image's height, width, and contains the image's raw pixels.
/// For use when communicating between JS and WASM, and also natively. 
#[wasm_bindgen]
#[derive(Debug)]
pub struct Font {
    name: String,
    size: u8,
    color: Rgb
}

#[wasm_bindgen]
impl Font {
    pub fn new(name: &str, size: u8, color: Rgb) -> Font {
        return Font {name: name.to_string(), size: size, color: color};
    }
}


#[wasm_bindgen]
#[derive(Debug)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

#[wasm_bindgen]
impl Rgb {
    pub fn new(&mut self, r: u8, g: u8, b: u8) -> Rgb {
        return Rgb {r: r, g: g, b: b}
    }
}

#[wasm_bindgen]
pub fn drawRectWeb(ctx: CanvasRenderingContext2d) {
    ctx.rect(10.0, 20.0, 50.0, 50.0);
    ctx.stroke();
}

// Called by the JS entry point to ensure that everything is working as expected
#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    set_panic_hook();

    let window = web_sys::window().expect("No Window found, should have a Window");
    let document = window.document().expect("No Document found, should have a Document");

    let p: web_sys::Node = document.create_element("p")?.into();
    p.set_text_content(Some("You're successfully running WASM!"));

    let body = document.body().expect("ERR: No body found, should have a body");
    let body: &web_sys::Node = body.as_ref();
    body.append_child(&p)?;
    Ok(())
}

/// Get the ImageData from a 2D canvas context
#[wasm_bindgen]
pub fn getImageData(canvas: &HtmlCanvasElement, ctx: &CanvasRenderingContext2d) -> ImageData {
    set_panic_hook();
    let width = canvas.width();
    let height = canvas.height();

    // let data: ImageData = ctx.get_image_data(0.0, 0.0, 100.0, 100.0).unwrap();
    let mut data = ctx.get_image_data(0.0, 0.0, width as f64, height as f64).unwrap();
    let vec_data = data.data().to_vec();
    return data;
}

/// Place the ImageData onto the 2D context.
#[wasm_bindgen]
pub fn putImageData(canvas: HtmlCanvasElement, ctx: CanvasRenderingContext2d, mut new_image: PhotonImage) {
    // Convert the raw pixels back to an ImageData object.
    let new_imgdata = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut new_image.raw_pixels), canvas.width(), canvas.height());

    // Place the new imagedata onto the canvas
    ctx.put_image_data(&new_imgdata.unwrap(), 0.0, 0.0);
}

/// Convert the ImageData found in the canvas context to a PhotonImage,
/// which can be used to filter or apply effects to the image
#[wasm_bindgen]
#[no_mangle]
pub fn open_image(canvas: HtmlCanvasElement, ctx: CanvasRenderingContext2d) -> PhotonImage {
    let imgdata = getImageData(&canvas, &ctx);
    let raw_pixels = to_raw_pixels(imgdata);
    return PhotonImage {raw_pixels: raw_pixels, width: canvas.width(), height: canvas.height() }
}

/// Create a new RGB colour. TODO Will be using struct impl soon. 
#[wasm_bindgen]
pub fn new_rgb(r:u8, g:u8, b:u8) -> Rgb {
    let rgb = Rgb{r, g, b};
    return rgb;
}

#[wasm_bindgen]
pub fn to_raw_pixels(imgdata: ImageData) -> Vec<u8> {
    let mut img_vec = imgdata.data().to_vec();
    return img_vec;
}

/// Convert a PhotonImage to JS-compatible ImageData
#[wasm_bindgen]
pub fn to_image_data(photon_image: PhotonImage) -> ImageData {
    let mut raw_pixels = photon_image.raw_pixels;
    let width = photon_image.width;
    let height = photon_image.height;
    let new_imgdata = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut raw_pixels), width, height).unwrap();

    return new_imgdata;
}

/// Get the ImageData from a 2D canvas context
#[wasm_bindgen]
pub fn get_image_data(canvas: &HtmlCanvasElement, ctx: &CanvasRenderingContext2d) -> ImageData {
    let width = canvas.width();
    let height = canvas.height();

    let data = ctx.get_image_data(0.0, 0.0, width as f64, height as f64).unwrap();
    return data;
}

fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function to get better error messages if we ever panic.
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

pub mod text;
pub mod background;
pub mod diagrams;
pub mod collage;
pub mod helpers;
pub mod graphics;
pub mod elements;
pub mod presets;
pub mod resize;