extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/utils.js")]
extern {
  fn appendStr(s: &str);
}

#[wasm_bindgen]
pub fn run_str() {
  appendStr("blaaaa")
}