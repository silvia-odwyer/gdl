# GDL

#### Graphic Design Library

GDL (Graphic Design Library) is a high-performance Rust 2D graphic design library, allowing developers to create 2D graphics and resize them 
for various social media platforms. 

GDL allows you to create:
- Image collages
- Social media graphics
- Banners
- Adverts 

### View Example Graphics
[See examples of designs created with GDL here.](https://silvia-odwyer.github.io/gdl/)

### Documentation
Documentation can be found [here](https://silvia-odwyer.github.io/gdl/docs/gdl/index.html).

## Run The Examples

<!-- ## Cargo Status -->
<!-- `GDL` can be installed via Cargo by declaring the following dependency in your Cargo.toml file:
```toml
[dependencies]
GDL-rs = "*"
``` -->

Clone this repo:
```sh
git clone https://github.com/silvia-odwyer/gdl
```

Run the binary, which will create a sample graphic:
```sh
cd crate
cargo run --release 
```

See [/examples](https://github.com/silvia-odwyer/gdl/tree/master/crate/examples) for more examples.

## Use Custom Fonts [*.ttf supported only for now]
If you'd like to use your own custom fonts, which are TrueType fonts (those with a *.ttf file extension), follow these steps:

1. Navigate to the `fonts` directory, which can be found inside the `crates` dir. 

```bash
cd crate/fonts 
```

2. Insert the font's TTF file into this directory, ie: `crate/fonts`. 

3. When drawing text and you need to pass a font name into the function, use the name of the font found in the filename. 

For example, if the custom font is called Robotika.ttf then pass the name Robotika into the function. 


### Why GDL?
This does not attempt to be a full-fledged graphic design solution, but rather an aid to those who want to create graphics-on-the-fly quickly and in large quantities.
Designing batches of graphics for large numbers of blog posts can be quite cumbersome for bloggers and freelance writers. Hence why I started working on this library.

#### Work-In-Progress
This is a work-in-progress, therefore the API will likely break in future versions until it reaches 1.0.0. 

#### WebAssembly
Coming soon:tm:

The WebAssembly version of this library will make use of the browser's canvas, for graphic creation in the browser.

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