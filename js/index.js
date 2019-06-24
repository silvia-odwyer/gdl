import Fruit from "./accelerate.jpg";
import Daisies from "./scintillate.jpg";
import Lemons from "./night-sky.jpg";
import Underground from "./underground.jpg";
import NineYards from "./nine_yards.jpg";
import BlueMetro from "./finance.jpg";
import Watermark from "./wasm_logo.png"
import LargeFruit from "./fruit.jpg";

// Setup global variables
var canvas, canvas2, watermark_canvas;
var ctx, ctx2, watermark_ctx;

import("../crate/pkg").then(module => {


  var startTime;
  var endTime;
  module.run();
 
  // Setup images
  const newimg = new Image();
  newimg.src = Fruit;
  newimg.style.display = "none";
  newimg.onload = () => {
    setUpCanvas();
  }

  const img2 = new Image();
  img2.src = Daisies;
  img2.style.display = "none";
  img2.onload=() => {
    setUpCanvas2();
  }

  const watermark_img = new Image();
  watermark_img.src = Watermark;
  watermark_img.style.display = "none";
  watermark_img.onload = () => {
    setUpWatermark();
  }

  let effect_buttons = document.getElementsByClassName("effect");
  for (let i = 0; i < effect_buttons.length; i++) {
    let button = effect_buttons[i];
    button.addEventListener("click", function(){applyEffect(event)}, false);
  }


  function applyEffect(event) {
    console.time("wasm_time"); 
    console.log(canvas2.height);
    console.log(canvas2.width);

    startTime = performance.now();

    // Get the name of the effect the user wishes to apply to the image
    // This is the id of the element they clicked on
    let filter_name = event.originalTarget.id;
    
    // Convert the ImageData to a PhotonImage (so that it can communicate with the core Rust library)
    let rust_image = module.open_image(canvas, ctx);
    let rust_image2 = module.open_image(canvas2, ctx2);

    let rgb = module.new_rgb(150, 100, 200);
    let filter_dict = {"gradient_rect" : function() { return rust_image.draw_gradient_rect(200, 200, 10, 20)},
    "solid_rect" : function() { return rust_image.draw_solid_rect(rgb, 200, 200, 100, 20)},
    "draw_text" : function() { return rust_image.draw_text("Hello and welcome :)", 20, 30, "Roboto-Regular", 90)},
    "draw_text_border" : function() { return rust_image.draw_text_with_border("Hello and Welcome :)", 150, 100)},
    };
    filter_dict[filter_name]();
    let new_img = module.collage_test(rust_image2, canvas.width, canvas.height);

    // Update the canvas with the new imagedata
    module.putImageData(canvas, ctx, new_img);
    console.timeEnd("wasm_time");
    endTime = performance.now()
    updateBenchmarks()
  }

  function blendImages(event) {
    console.time("wasm_blend_time"); 

    ctx.drawImage(newimg, 0, 0);
    startTime = performance.now();

    // Get the name of the effect the user wishes to apply to the image
    // This is the id of the element they clicked on
    let filter_name = event.originalTarget.id;
    
    // Get the image data from the image
    let imgData = module.getImageData(canvas, ctx);

    // Convert the ImageData to a PhotonImage (so that it can communicate with the core Rust library)
    let rust_image = module.open_image(imgData, canvas.width, canvas.height);

    let imgData2 = ctx2.getImageData(0, 0, canvas2.width, canvas2.height);

    let rust_image2 = module.open_image(imgData2, canvas2.width, canvas2.height);


    // Maps the name of an effect to its relevant function in the Rust library
    let filter_dict = {
                      "blend": function() {return module.blend(rust_image, rust_image2, "over")},
                      "overlay": function() {return module.blend(rust_image, rust_image2, "overlay")},
                      "atop": function() {return module.blend(rust_image, rust_image2, "atop")},
                      "xor": function() {return module.blend(rust_image, rust_image2, "xor")},
                      "plus": function() {return module.blend(rust_image, rust_image2, "plus")},
                      "multiply": function() {return module.blend(rust_image, rust_image2, "multiply")},
                      "burn": function() {return module.blend(rust_image, rust_image2, "burn")},
                      "difference": function() {return module.blend(rust_image, rust_image2, "difference")},
                      "soft_light": function() {return module.blend(rust_image, rust_image2, "soft_light")},
                      "hard_light": function() {return module.blend(rust_image, rust_image2, "hard_light")},
                      "dodge": function() {return module.blend(rust_image, rust_image2, "dodge")},
                      "exclusion": function() {return module.blend(rust_image, rust_image2, "exclusion")},
                      "lighten": function() {return module.blend(rust_image, rust_image2, "lighten")},
                      "darken": function() {return module.blend(rust_image, rust_image2, "darken")},
                      "watermark": function() {return module.watermark(rust_image, watermark_img, 10, 30)},
                      "text": function() {return module.draw_text(rust_image, "welcome to WebAssembly", 10, 20)},
                      "text_border": function() {return module.draw_text_with_border(rust_image, "welcome to the edge", 10, 20)},
                    };

    // Filter the image, the PhotonImage's raw pixels are modified and 
    // the PhotonImage is returned
    let new_image = filter_dict[filter_name]();

    // Update the canvas with the new imagedata
    module.putImageData(canvas, ctx, new_image);
    console.timeEnd("wasm_blend_time");
    endTime = performance.now()
    updateBenchmarks()
  }

  function colour_space(rust_image, colour_space, effect) {
    let new_image;
    if (colour_space == "hsl") {
      new_image = module.lch(rust_image, effect, 0.3);
    }
    else if (colour_space == "hsv") {
      new_image = module.hsv(rust_image, effect, 0.3)
    }
    else {
      new_image = module.lch(rust_image, effect, 0.3);
    }
    updateCanvas(new_image);
  }
  
  
  function single_channel_grayscale(rust_image, channel) {
    let new_image = module.single_channel_grayscale(rust_image, channel);
    updateCanvas(new_image);
  }

  function updateCanvas(new_image) {
    let new_pixels = module.to_image_data(new_image);
    
    // Place the pixels back on the canvas
    ctx.putImageData(new_pixels, 0, 0);
  }


  function filterImage(event) {
    startTime = performance.now();
    ctx.drawImage(newimg, 0, 0);
    let filter_name = event.originalTarget.id;
  
    console.time("wasm_time"); 
    
    // Get the image data from the image
    let imgData = ctx.getImageData(0, 0, canvas.width, canvas.height);

    // Convert the ImageData to a PhotonImage (so that it can communicate with the core Rust library)
    let rust_image = module.open_image(imgData, canvas.width, canvas.height);

    // Filter the image, the PhotonImage's raw pixels are modified and 
    // the PhotonImage is returned
    let new_image = module.filter(rust_image, filter_name);

    // Convert the PhotonImage's raw pixels to JS-compatible ImageData
    let new_pixels = module.to_image_data(new_image);
    
    endTime = performance.now();
    updateBenchmarks();
    // Place the pixels back on the canvas
    ctx.putImageData(new_pixels, 0, 0);
    console.timeEnd("wasm_time");
  }

  function setUpCanvas() {
    let element = document.getElementById("image_container");
    element.appendChild(newimg);

    canvas = document.getElementById("canvas");
    canvas.width = newimg.width;
    canvas.height = newimg.height;

    ctx = canvas.getContext("2d");
    ctx.drawImage(newimg, 0, 0);
  }

  function setUpCanvas2() {
    let element = document.getElementById("image_container");
    element.appendChild(img2);
    canvas2 = document.createElement("canvas");
    canvas2.width = img2.width;
    canvas2.height = img2.width;

    ctx2 = canvas2.getContext("2d");
    ctx2.drawImage(img2, 0, 0);

  }

  function setUpWatermark() {
    let element = document.getElementById("image_container");
    element.appendChild(watermark_img);
    watermark_canvas = document.createElement("canvas");
    watermark_canvas.width = watermark_img.width;
    watermark_canvas.height = watermark_img.height;

    watermark_ctx = watermark_canvas.getContext("2d");
    watermark_ctx.drawImage(watermark_img, 0, 0);

  }

  function updateBenchmarks() {
    console.log("update benchmarks");
    let time_taken = endTime - startTime;
    let time_elem = document.getElementById("time");
    time_elem.innerHTML = `Time: ${time_taken}ms`;
  }

  // Change the image currently being edited.
  let change_image_elems = document.getElementsByClassName("change_image");

  for (let i = 0; i < change_image_elems.length; i++) {
    let change_image_elem = change_image_elems[i];

    change_image_elem.addEventListener("click", function(event) {
      console.log("image changed")
      let img_name = event.originalTarget.id;
      let imgNamesToImages = {"largefruit": LargeFruit, "lemons": Lemons, "underground": Underground, "blue_metro": BlueMetro, "nine_yards": NineYards, "daisies": Daisies, "fruit": Fruit};
      newimg.src = imgNamesToImages[img_name];
      newimg.onload = () => {
        canvas.width = newimg.width;
        canvas.height = newimg.height;
        ctx.drawImage(newimg, 0, 0);
      }
    }, false);
  }

});

function editImage(canvas, ctx) {
  let imgData = ctx.getImageData(0, 0, canvas.width, canvas.height);
  for (let i = 0; i < imgData.data.length; i += 4) {
    imgData[i] += 30;
  }
  ctx.putImageData(imgData, 0, 0);
}