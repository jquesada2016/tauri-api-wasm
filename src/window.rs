mod logical_position;
mod logical_size;
mod physical_position;
mod physical_size;
mod webview_window;
mod window_manager;

pub use logical_position::*;
pub use logical_size::*;
pub use physical_position::*;
pub use physical_size::*;
use wasm_bindgen::prelude::*;
pub use webview_window::*;
pub use window_manager::*;

/// Attention type to request on a window.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[wasm_bindgen]
pub enum UserAttentionType {
  /// #### Platform-specific
  /// - **macOS**: Bounces the dock icon until the application is in focus.
  /// - **Windows**: Flashes both the window and the taskbar button until the application is in focus.
  Critical = 1,
  /// #### Platform-specific
  /// - **macOS**: Bounces the dock icon once.
  /// - **Windows**: Flashes the taskbar button until the application is in focus.
  Informational = 2,
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize)]
pub enum Theme {
  Light = "light",
  Dark = "Dark",
}

/// The style of the macOS title bar.
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize)]
pub enum TitleBarStyle {
  Visible = "visible",
  Transparent = "transparent",
  Overlay = "overlay",
}
