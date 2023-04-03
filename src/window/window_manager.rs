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
    pub async fn center(&self);
    pub async fn setResizable(&self, resizable: bool);
    pub async fn setTitle(&self, title: &str);
    pub async fn maximize(&self);
    pub async fn unmaximize(&self);
    pub async fn toggleMaximize(&self);
    pub async fn minimize(&self);
    pub async fn unminimize(&self);
    pub async fn show(&self);
    pub async fn hide(&self);
    pub async fn close(&self);
    pub async fn setDecorations(&self);
    pub async fn setAlwaysOnTop(&self);
    pub async fn setFullscreen(&self, fullscreen: bool);
    pub async fn setFocus(&self);
    pub async fn setSkipTaskbar(&self, skip: bool);
    pub async fn setIgnoreCursorEvents(&self, ignore: bool);
  }
}
