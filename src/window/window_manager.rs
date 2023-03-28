use super::{
  web_view_window_handle::WebviewWindowHandle,
  PhysicalPosition,
  PhysicalSize,
};
use wasm_bindgen::prelude::*;

declare_type! {
  #[extends = WebviewWindowHandle]
  pub struct WindowManager {
    pub label: String,
  }

  impl WindowManager {
    @async

    pub async fn scaleFactor(&self) -> js_sys::Number;
    pub async fn innerPosition(&self) -> PhysicalPosition;
    pub async fn outerPosition(&self) -> PhysicalPosition;
    pub async fn innerSize(&self) -> PhysicalSize;
    pub async fn outerSize(&self) -> PhysicalSize;
    pub async fn isFullscreen(&self) -> js_sys::Boolean;
    pub async fn isMaximized(&self) -> js_sys::Boolean;
    pub async fn isDecorated(&self) -> js_sys::Boolean;
    pub async fn isResizable(&self) -> js_sys::Boolean;


    pub async fn isVisible(&self) -> js_sys::Boolean;
  }
}
