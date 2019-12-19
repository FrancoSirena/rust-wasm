use wasm_bindgen::prelude::*;
use web_sys::console;


// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();


    // Your code goes here!
    console::log_1(&JsValue::from_str("Hello world!"));

    Ok(())
}

#[wasm_bindgen]
extern "C" {
  type HTMLDocument;
  static document: HTMLDocument;
  #[wasm_bindgen(method)]
  fn createElement(this: &HTMLDocument, tagName: &str) -> Element;
  #[wasm_bindgen(method, getter)]
  fn body(this: &HTMLDocument) -> Element;

  type Element;
  #[wasm_bindgen(method, setter = innerHTML)]
  fn set_inner_html(this: &Element, html: &str);
  #[wasm_bindgen(method, js_name = appendChild)]
  fn append_child(this: &Element, other: Element);
  #[wasm_bindgen(js_namespace = console)]
  fn log(msg: &str);
}

macro_rules! log {
  ($($t:tt)*) => (log(&format!($($t)*)))
}

#[wasm_bindgen]
pub fn run() {
  let val = document.createElement("p");
  log!("franco {}", "ça");
  val.set_inner_html("Hwey aosd");
  document.body().append_child(val);
}

#[wasm_bindgen]
pub struct Color {
  red: u8,
  green: u8,
  blue: u8,
}

#[wasm_bindgen]
pub struct Image {
  pixels: Vec<Color>,
}

#[wasm_bindgen]
impl Image {
  pub fn new() -> Image {
    let color1 = Color {
      red: 255, green: 0, blue: 0
    };
    let color2 = Color {
      red: 200, green: 100, blue: 0
    };
    let pixels = vec![color1, color2];
    Image {
      pixels
    }
  }

  pub fn pixels_ptr(&self) -> *const Color {
    self.pixels.as_ptr()
  }
}