use crate::window::LogicalPosition;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
  /// A position represented in physical pixels.
  pub type PhysicalPosition;

  #[wasm_bindgen(constructor, js_namespace = ["__TAURI__", "window"])]
  pub fn new(x: u32, y: u32) -> PhysicalPosition;

  #[wasm_bindgen(method, getter)]
  pub fn x(this: &PhysicalPosition) -> u32;

  #[wasm_bindgen(method, getter)]
  pub fn y(this: &PhysicalPosition) -> u32;

  /// Converts the physical position to a logical one.
  #[wasm_bindgen(method, js_name = toLogical)]
  pub fn to_logical(
    this: &PhysicalPosition,
    scale_factor: f32,
  ) -> LogicalPosition;
}
