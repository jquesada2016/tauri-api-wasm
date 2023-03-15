use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
  /// A size represented in logical pixels.
  pub type LogicalSize;

  #[wasm_bindgen(constructor, js_namespace = ["__TAURI__", "window"])]
  pub fn new(width: f32, height: f32) -> LogicalSize;

  #[wasm_bindgen(method, getter)]
  pub fn width(this: &LogicalSize) -> f32;

  #[wasm_bindgen(method, getter)]
  pub fn height(this: &LogicalSize) -> f32;
}
