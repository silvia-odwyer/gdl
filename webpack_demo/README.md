#### Webpack + GDL Demo

##### Instructions for Running

1. Clone this repo.
```sh 
git clone https://github.com/silvia-odwyer/gdl
```

2. Navigate to `crate` and run `wasm-pack build`. Ensure you have wasm-pack installed.

```sh
cd crate 
wasm-pack build
```

This will create an npm package, which can be found in `crate/pkg`. 

3. Then, navigate to this dir, and install dependencies. 

```sh
cd ../../react_app_demo 
npm install
```

4. Start a development server at http://localhost:8080

`npm run start` -- Serve the project locally for development at
  `http://localhost:8080`.


**Building**
* `npm run build` -- Bundle the project (in production mode).

##### Deep-Dive into The Code
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

    // Convert the ImageData to a PhotonImage (so that it can communicate with the core Rust library)
    let rust_image = module.open_image(imgData, canvas.width, canvas.height);

    // Draw text onto the image, the PhotonImage's raw pixels are modified
    module.draw_text(new_image, "This is text", 30, 40, "Roboto-Regular", 30.0);
    
    // Place the pixels back on the canvas
    ctx.putImageData(new_image, canvas, context);
  }
```

Not all functions available in the core Rust library are available in WebAssembly (currently investigating this). Only WASM-friendly functions have been annotated with #[wasm_bindgen]. All supported WASM functions are displayed in the starter demo.  -->
