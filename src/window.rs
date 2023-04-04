mod logical_position;
mod logical_size;
mod monitor;
mod physical_position;
mod physical_size;
mod web_view_window_handle;
mod webview_window;
mod window_manager;

pub use logical_position::*;
pub use logical_size::*;
pub use monitor::*;
pub use physical_position::*;
pub use physical_size::*;
use wasm_bindgen::prelude::*;
pub use webview_window::*;
pub use window_manager::*;

pub const WINDOW_RESIZED: &str = "tauri://resize";
pub const WINDOW_MOVED: &str = "tauri://move";
pub const WINDOW_CLOSE_REQUESTED: &str = "tauri://close-requested";
pub const WINDOW_CREATED: &str = "tauri://window-created";
pub const WINDOW_DESTROYED: &str = "tauri://destroyed";
pub const WINDOW_FOCUS: &str = "tauri://focus";
pub const WINDOW_BLUR: &str = "tauri://blur";
pub const WINDOW_SCALE_FACTOR_CHANGED: &str = "tauri://scale-change";
pub const WINDOW_THEME_CHANGED: &str = "tauri://theme-changed";
pub const WINDOW_FILE_DROP: &str = "tauri://file-drop";
pub const WINDOW_FILE_DROP_HOVER: &str = "tauri://file-drop-hover";
pub const WINDOW_FILE_DROP_CANCELLED: &str = "tauri://file-drop-cancelled";
pub const MENU: &str = "tauri://menu";
pub const CHECK_UPDATE: &str = "tauri://update";
pub const UPDATE_AVAILABLE: &str = "tauri://update-available";
pub const INSTALL_UPDATE: &str = "tauri://update-install";
pub const STATUS_UPDATE: &str = "tauri://update-status";
pub const DOWNLOAD_PROGRESS: &str = "tauri://update-download-progress";

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
  Dark = "dark",
}

/// The style of the macOS title bar.
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize)]
pub enum TitleBarStyle {
  Visible = "visible",
  Transparent = "transparent",
  Overlay = "overlay",
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CursorIcon {
  Default = "default",
  Crosshair = "crosshair",
  Hand = "hand",
  Arrow = "arrow",
  Move = "move",
  Text = "text",
  Wait = "wait",
  Help = "help",
  Progress = "progress",
  NotAllowed = "notAllowed",
  ContextMenu = "contextMenu",
  Cell = "cell",
  VerticalText = "verticalText",
  Alias = "alias",
  Copy = "copy",
  NoDrop = "noDrop",
  Grab = "grab",
  Grabbing = "grabbing",
  AllScroll = "allScroll",
  ZoomIn = "zoomIn",
  ZoomOut = "zoomOut",
  EResize = "eResize",
  NResize = "nResize",
  NEResize = "neResize",
  NWResize = "nwResize",
  SResize = "sResize",
  SEResize = "seResize",
  SWResize = "swResize",
  WResize = "wResize",
  EWResize = "ewResize",
  NSResize = "nsResize",
  NESWResize = "neswResize",
  NWSEResize = "nwseResize",
  ColResize = "colResize",
  RowResize = "rowResize",
}

/// Returns the list of all the monitors available on the system.
pub async fn available_monitors() -> Vec<Monitor> {
  available_monitors_js()
    .await
    .unchecked_into::<js_sys::Array>()
    .values()
    .into_iter()
    .map(|v| v.unwrap())
    .map(|monitor| monitor.unchecked_into())
    .collect()
}

/// Returns the monitor on which the window currently resides. Returns null if current monitor can't be detected.
pub async fn current_monitor() -> Option<Monitor> {
  let monitor = current_monitor_js().await;

  if monitor.is_null() {
    None
  } else {
    Some(monitor.unchecked_into::<Monitor>())
  }
}

/// Returns the primary monitor of the system. Returns null if it can't identify any monitor as a primary one.
pub async fn primary_monitor() -> Option<Monitor> {
  let monitor = primary_monitor_js().await;

  if monitor.is_null() {
    None
  } else {
    Some(monitor.unchecked_into::<Monitor>())
  }
}

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_name = appWindow, js_namespace = ["__TAURI__", "window"])]
  pub static APP_WINDOW: WebviewWindow;

  #[wasm_bindgen(js_name = availableMonitors, js_namespace = ["__TAURI__", "window"])]
  async fn available_monitors_js() -> JsValue;

  #[wasm_bindgen(js_name = currentMonitor, js_namespace = ["__TAURI__", "window"])]
  async fn current_monitor_js() -> JsValue;

  /// Gets a list of instances of WebviewWindow for all available webview windows.
  #[wasm_bindgen(js_name = getAll, js_namespace = ["__TAURI__", "window"])]
  pub fn get_all() -> Vec<WebviewWindow>;

  /// Get an instance of WebviewWindow for the current webview window.
  #[wasm_bindgen(js_name = getCurrent, js_namespace = ["__TAURI__", "window"])]
  pub fn get_current() -> WebviewWindow;

  #[wasm_bindgen(js_name = primaryMonitor, js_namespace = ["__TAURI__", "window"])]
  async fn primary_monitor_js() -> JsValue;
}
