# GDL

#### Graphic Design Library

GDL (Graphic Design Library) is a high-performance Rust 2D graphic design library, which compiles to WebAssembly, allowing developers to create 2D graphics at near-native speed in the browser, and natively on their machines. 

GDL allows you to create:
- Image collages
- Social media graphics
- Banners
- Adverts 

The following designs were created with GDL:

(All background/foreground images used in the designs are Public Domain, and available from Unsplash.)

<!-- 
##### Features:
- **Pure Rust** - Unlike other libraries, this library is built with 100% pure Rust, so security and safety is guaranteed. 
- **WebAssembly friendly** - For web-based graphic design, GDL is 4-10x faster than JS, leading to faster results, and less lag. 
- **Call WASM with JS** - This library's has exposed JS functions, allowing for zero-cost abstraction and faster development.
- **Over 90 functions** - GDL provides functions for every domain of image processing.  -->

### Live Demo
View the [official demo of WASM in action](https://silvia-odwyer.github.io/GDL).

### Documentation
Documentation can be found [here](https://silvia-odwyer.github.io/GDL/docs/GDL/index.html).

## Run The Examples

<!-- ## Cargo Status -->
<!-- `GDL` can be installed via Cargo by declaring the following dependency in your Cargo.toml file:
```toml
[dependencies]
GDL-rs = "*"
``` -->

Clone this repo:
```sh
git clone https://github.com/silvia-odwyer/GDL
```

Run the binary, which will create a sample graphic:
```sh
cd crate
cargo run --release 
```

### Why GDL?
A common issue among bloggers is resizing graphics for social media, as well as designing entire batches of graphics for large
numbers of blog posts. This does not attempt to be a full-fledged graphic design solution, but rather an aid to those who 
want to create graphics-on-the-fly quickly and in large quantities.

GDL can be used in-tandem with the Photon image processing library for combined effects and further image manipulation at a lower level. GDL intends to be a wrapper over Photon, perfect for developers who wish to incorporate high-level graphic design functionality into their web apps and native apps. 


#### Work-In-Progress
This is a work-in-progress, therefore the API will likely break in future versions until it reaches 1.0.0. 

Text-positioning, centering, scaling to fit the size of the image is heavily being worked on at the moment, 
so if you notice this to be an issue, it is being looked at.

<!-- 
### Native Use
GDL contains native-only functions for opening files from the filesystem. 

When an image is opened, it is converted to a `GDLImage` type, which can then be passed into any image processing function, and the `GDLImage` value is accordingly edited.

Getting started is relatively straightforward, this code snippet is all you need to get started:
```rust
extern crate GDL;
fn main() {

}
```

See the documentation for a full list of effects which you can apply. All functions take in a `GDLImage` similar to above.

### 🚴 Get Started With WebAssembly 

##### 🔋 Batteries Included

This repo comes pre-configured with a quick-start demo, which hooks into a Webpack build pipeline, and provides all WASM-friendly functions.

***WARNING***: Running WASM code in development mode is ***significantly*** slower than in production mode (often up to 10 times),
so don't be disheartened if the JS alternatives outperform WASM. For the blazing speeds promised, make sure to build the 
project under production mode with `npm run build` and visit `dist/index.html`. 

* `npm run start` -- Serve the project locally for development at
  `http://localhost:8080`.

* `npm run build` -- Bundle the project (in production mode).

##### WebAssembly Use
To allow for universal communication between the core Rust library and WebAssembly, the functions have been generalised to allow for both native and in-browser use. 

Due to this, image data from the browser must first be converted to a GDLImage before being passed to the image processing functions. 

The GDLImage can then be converted back to JS-compatible ImageData so that it can be displayed in-browser.

See the code snippet below:
```js
function filterImage(event) {
    var canvas = document.getElementById("canvas");
    var ctx = canvas.getContext("2d");
    
    ctx.drawImage(newimg, 0, 0);
    
    // Get the image data from the image
    let imgData = ctx.getImageData(0, 0, canvas.width, canvas.height);

    // Convert the ImageData to a GDLImage (so that it can communicate with the core Rust library)
    let rust_image = module.open_image(imgData, canvas.width, canvas.height);

    // Filter the image, the GDLImage's raw pixels are modified and 
    // the GDLImage is returned
    let new_image = module.filter(rust_image, filter_name);

    // Convert the GDLImage's raw pixels to JS-compatible ImageData
    let new_pixels = module.to_image_data(new_image);
    
    // Place the pixels back on the canvas
    ctx.putImageData(new_pixels, 0, 0);
  }
```

Not all functions available in the core Rust library are available in WebAssembly (currently investigating this). Only WASM-friendly functions have been annotated with #[wasm_bindgen]. All supported WASM functions are displayed in the starter demo.  -->

<!-- 
## Modules 
GDL contains a series of modules, which include:

- `text`: Add text, bordered text with TTF fonts.
- `collage`: Create collages, groups of images, image grids, etc.
- `elements`: Preset and customisable elements consisting of icons, shapes, gradients, etc. 
- `background`: Patterns and backgrounds.  
- `multiple`: A module for dealing with multiple images, such as watermarking images, etc.,
- `correction`: Hue rotation, adjusting saturation, lightening/darkening: all techniques available in multiple colour spaces, which lead to varying effects.

All effects and filters can be viewed below and on the official website.

## 📚 Documentation
View the official [documentation here](https://silvia-odwyer.github.io/GDL/docs/GDL/index.html). 


## Building For Production

#### Native
You can edit the binary file in the `bin` dir to create an executable or a command-line app, for example. 

Then, to build this executable for native use in release mode:

```sh
cd crate 
cargo build --release
```

#### WebAssembly
To build the example under production mode:

```sh
npm run build
```

Check the `dist` folder for the outputted static files, which can be deployed to a live server.


## Contributing

GDL is always ready for new filters and functions, so if you'd like to contribute, we're always ready to accept new Pull Requests or investigate new issues. 

## Authors

* **Silvia O'Dwyer** - [GitHub Profile](https://github.com/silvia-odwyer)
* **Future You(?)** - (See Contributing above ;) 

## License

This project is licensed under the Apache 2.0 License - see the [LICENSE.md](LICENSE.md) file for details. -->