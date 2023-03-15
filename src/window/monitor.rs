use crate::window::{
  PhysicalPosition,
  PhysicalSize,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
  pub type Monitor;

  /// Human-readable name of the monitor
  #[wasm_bindgen(method, getter)]
  pub fn name(this: &Monitor) -> Option<String>;

  /// the Top-left corner position of the monitor relative to the larger full screen area.
  #[wasm_bindgen(method, getter)]
  pub fn position(this: &Monitor) -> PhysicalPosition;

  /// the Top-left corner position of the monitor relative to the larger full screen area.
  #[wasm_bindgen(method, getter, js_name = scaleFactor)]
  pub fn scale_factor(this: &Monitor) -> f32;

  /// The monitor's resolution.
  #[wasm_bindgen(method, getter)]
  pub fn size(this: &Monitor) -> PhysicalSize;
}
