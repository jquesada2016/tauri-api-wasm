use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
  /// A position represented in logical pixels.
  pub type LogicalPosition;

  #[wasm_bindgen(constructor, js_namespace = ["__TAURI__", "window"])]
  pub fn new(x: f32, y: f32) -> LogicalPosition;

  #[wasm_bindgen(method, getter)]
  pub fn x(this: &LogicalPosition) -> f32;

  #[wasm_bindgen(method, getter)]
  pub fn y(this: &LogicalPosition) -> f32;
}
