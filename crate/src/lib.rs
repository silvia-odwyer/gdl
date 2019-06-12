use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, ImageData, HtmlCanvasElement, HtmlImageElement};
use wasm_bindgen::Clamped;
use web_sys::console;
use image::{DynamicImage, GenericImageView, GenericImage, ImageBuffer, Rgba};
use imageproc::drawing::draw_text_mut;
use imageproc::morphology::dilate_mut;
use rusttype::Scale;
use rusttype::{FontCollection};
use imageproc::distance_transform::Norm;
use imageproc::drawing::draw_filled_rect;
use imageproc::rect::Rect;
use imageproc::rect::Region;
use imageproc::drawing::draw_hollow_rect_mut;

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
    pub fn new(&mut self, img_data: ImageData, width: u32, height: u32) -> PhotonImage {
        let raw_pixels = to_raw_pixels(img_data);
        let new_vec = Vec::new();
        return PhotonImage {raw_pixels: new_vec, width: width, height: height}
    }

    
    pub fn new_with_background(width: u32, height: u32, background_color: Rgb) -> PhotonImage {
        // create a pixel 
        let pixel =  image::Rgba([background_color.r, background_color.g, background_color.b, 255]);
        let image_buffer = ImageBuffer::from_pixel(width, height, pixel);
        let rgb_img = image::ImageRgba8(image_buffer);

        let raw_pixels = rgb_img.raw_pixels();
        return PhotonImage { raw_pixels: raw_pixels, width: width, height: height};
    }

    pub fn draw_text(&mut self, text: &str, x: u32, y:u32) {
        
        let mut image = helpers::dyn_image_from_raw(&self).to_rgba();

        let mut image2 : DynamicImage = DynamicImage::new_luma8(
            image.width(), image.height());
        let font = "Roboto-Regular.ttf";

        let font_path = match font {
            "Roboto-Regular.ttf" => "../fonts/Roboto-Regular.ttf",
        };

        let font = Vec::from(include_bytes!(font_path) as &[u8]);
        let font = FontCollection::from_bytes(font).unwrap().into_font().unwrap();
        let height = 90f32;
        let scale = Scale { x: height * 1.0, y: height };
        draw_text_mut(&mut image2, Rgba([255u8, 255u8, 255u8, 255u8]), x, y, scale, &font, text);

        let mut image2 = image2.to_luma();
        dilate_mut(&mut image2, Norm::LInf, 4u8);

        draw_text_mut(&mut image, Rgba([25u8, 255u8, 255u8, 255u8]), x + 10, y - 10, scale, &font, text);
        let dynimage = image::ImageRgba8(image);
        self.raw_pixels = dynimage.raw_pixels();

    }

    pub fn draw_text_with_border(&mut self, text: &str, x: u32, y: u32) {
        let mut image = helpers::dyn_image_from_raw(&self).to_rgba();

        let mut image2 : DynamicImage = DynamicImage::new_luma8(
            image.width(), image.height());

        let font = Vec::from(include_bytes!("../fonts/Roboto-Black.ttf") as &[u8]);
        let font = FontCollection::from_bytes(font).unwrap().into_font().unwrap();
        let height = 90f32;
        let scale = Scale { x: height * 1.0, y: height };
        draw_text_mut(&mut image2, Rgba([255u8, 255u8, 255u8, 255u8]), x, y, scale, &font, text);

        let mut image2 = image2.to_luma();
        dilate_mut(&mut image2, Norm::LInf, 4u8);

        // Add a border to the text.
        for x in 0..image2.width() {
            for y in 0..image2.height() {
                let pixval = 255 - image2.get_pixel(x, y).data[0];
                if pixval != 255 {
                    let new_pix = Rgba([pixval, pixval, pixval, 255]);
                    image.put_pixel(x, y, new_pix);
                }
            }
        }

        draw_text_mut(&mut image, Rgba([255u8, 255u8, 255u8, 255u8]), x + 10, y - 10, scale, &font, text);



        let dynimage = image::ImageRgba8(image);
        self.raw_pixels = dynimage.raw_pixels();

    }

    pub fn draw_rect(&mut self, background_color: Rgb) {
        let red   = Rgba([255u8, 0u8,   0u8, 255u8]);
        let green = Rgba([0u8,   255u8, 0u8, 255u8]);
        let blue  = Rgba([0u8,   0u8,   255u8, 255u8]);
        let white = Rgba([255u8, 255u8, 255u8, 255u8]);

   
        let mut image = helpers::dyn_image_from_raw(&self).to_rgba();

        draw_hollow_rect_mut(&mut image, Rect::at(30, 10).of_size(1000, 200), white);
        let dynimage = image::ImageRgba8(image);
        self.raw_pixels = dynimage.raw_pixels();
    }


    pub fn raw_pix(self) -> Vec<u8> {
        self.raw_pixels
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
    let newData = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut new_image.raw_pixels), canvas.width(), canvas.height());

    // Place the new imagedata onto the canvas
    ctx.put_image_data(&newData.unwrap(), 0.0, 0.0);
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
pub fn new_rgb(imgdata: ImageData, r:u8, g:u8, b:u8) -> Rgb {
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
    let newData = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut raw_pixels), width, height).unwrap();

    return newData;
}

// draw image to canvas
//ctx.draw_image_with_html_image_element(&img, 0.0, 0.0);

fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function to get better error messages if we ever panic.
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

pub mod text;
pub mod helpers;
pub mod graphics;