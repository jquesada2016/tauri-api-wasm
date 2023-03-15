use crate::window::LogicalSize;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
  /// A size represented in logical pixels.
  pub type PhysicalSize;

  #[wasm_bindgen(constructor, js_namespace = ["__TAURI__", "window"])]
  pub fn new(width: u32, height: u32) -> PhysicalSize;

  #[wasm_bindgen(method, getter)]
  pub fn width(this: &PhysicalSize) -> u32;

  #[wasm_bindgen(method, getter)]
  pub fn height(this: &PhysicalSize) -> u32;

  /// Converts the physical position to a logical one.
  #[wasm_bindgen(method, js_name = toLogical)]
  pub fn to_logical(this: &PhysicalSize, scale_factor: f32) -> LogicalSize;
}
