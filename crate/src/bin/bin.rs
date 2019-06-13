extern crate image;
use image::{GenericImageView, GenericImage, ImageBuffer};
extern crate metallic;
use metallic::{Rgb, PhotonImage, ColorScheme};
use std::io::Read;
use std::string::String;

fn main() {
    // Open the image (a PhotonImage is returned) and get dimensions
    let img = metallic::helpers::open_image("examples/input_images/daisies_fuji.jpg");
    let img2 = metallic::helpers::open_image("examples/input_images/fruit_med.jpg");
    let watermark = metallic::helpers::open_image("examples/input_images/watermark.jpg");

    // Apply a filter to the pixels
    let rgb1 = Rgb{r: 120, g: 130, b: 54};
    let rgb2 = Rgb{r: 0, g: 0, b: 0};
    let red = Rgb{r: 204, g: 0, b: 0};
    let black = Rgb { r: 0, g: 0, b:0};

    let mut photon_img = PhotonImage::new_with_background(800, 400, black);
    let rgb4 = Rgb{r: 120, g: 130, b: 54};
    let cs = ColorScheme::new(rgb4);

    photon_img.draw_text_with_border("Paris", 20, 10);
    photon_img.draw_text("Case Studies/Industry Buzz/", 20, 80, "Roboto-Bold", 20.0);

    // photon_img.draw_text_with_border("London", 20, 710);
    // photon_img.draw_text("Beijing", 20, 790);   
    let rgb3 = Rgb{r: 120, g: 130, b: 54};

    // metallic::text::draw_text(&new_img, "Welcome to Metallic", 10, 10);

    // Write the contents of this image in JPG format.
    metallic::helpers::save_image(photon_img, "background_img.png");

    drawCitiesGraphic();
}

fn drawCitiesGraphic() {
    // Open the image (a PhotonImage is returned) and get dimensions
    let img = metallic::helpers::open_image("examples/input_images/daisies_fuji.jpg");
    let img2 = metallic::helpers::open_image("examples/input_images/fruit_med.jpg");
    let watermark = metallic::helpers::open_image("examples/input_images/watermark.jpg");

    // Apply a filter to the pixels
    let rgb1 = Rgb{r: 120, g: 130, b: 54};
    let rgb2 = Rgb{r: 0, g: 0, b: 0};
    let new_img = metallic::graphics::create_image(500, 500, rgb1);
    let mut photon_img = PhotonImage::new_with_background(1000, 1000, rgb2);
    let rgb4 = Rgb{r: 120, g: 130, b: 54};
    let cs = ColorScheme::new(rgb4);

    photon_img.draw_text("Dublin", 20, 10, "Roboto-Black", 90.0);
    photon_img.draw_text("New York", 20, 80, "Roboto-Bold", 80.0);
    // photon_img.draw_text_with_border("Paris", 20, 150);
    photon_img.draw_text("Bucharest", 20, 150, "Roboto-Regular", 70.0);
    // photon_img.draw_text_with_border("Tokyo", 20, 290);
    // photon_img.draw_text("Cairo", 20, 360);
    // photon_img.draw_text_with_border("London", 20, 430);
    // photon_img.draw_text("Beijing", 20, 500);
    // photon_img.draw_text_with_border("Tokyo", 20, 570);
    photon_img.draw_text("Paris", 20, 220, "Roboto-Light", 60.0);

    // photon_img.draw_text_with_border("London", 20, 710);
    // photon_img.draw_text("Beijing", 20, 790);   
    let rgb3 = Rgb{r: 120, g: 130, b: 54};


    // metallic::text::draw_text(&new_img, "Welcome to Metallic", 10, 10);

    // Write the contents of this image in JPG format.
    metallic::helpers::save_image(photon_img, "background_img.png");
}

fn eatSleepCode() {
        // Open the image (a PhotonImage is returned) and get dimensions
    let img = metallic::helpers::open_image("examples/input_images/daisies_fuji.jpg");
    let img2 = metallic::helpers::open_image("examples/input_images/fruit_med.jpg");
    let watermark = metallic::helpers::open_image("examples/input_images/watermark.jpg");

    let rgb2 = Rgb{r: 0, g: 0, b: 0};
    let mut photon_img = PhotonImage::new_with_background(1000, 1000, rgb2);
    let rgb4 = Rgb{r: 120, g: 130, b: 54};
    let cs = ColorScheme::new(rgb4);

    photon_img.draw_text("EAT", 250, 10 + 100, "Roboto-Black", 90.0);
    photon_img.draw_text("SLEEP", 250, 80 + 100, "Roboto-Bold", 80.0);
    
    photon_img.draw_text("CODE ", 250, 250, "Roboto-Regular", 90.0);

    photon_img.draw_text("REPEAT", 250, 320, "Roboto-Light", 90.0);
    photon_img.draw_text("Then do it all over again", 450, 280, "Roboto-Regular", 10.0);
   
    let rgb3 = Rgb{r: 120, g: 130, b: 54};

    // Write the contents of this image in JPG format.
    metallic::helpers::save_image(photon_img, "background_img.png");
}