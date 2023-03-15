use super::{
  Theme,
  TitleBarStyle,
};
use wasm_bindgen::prelude::*;

/// Configuration for the window to create.
#[derive(Clone, Debug, TypedBuilder, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowOptions {
  /// Whether clicking an inactive window also clicks through to the webview on macOS.
  #[builder(default, setter(strip_option))]
  pub accept_first_mouse: Option<bool>,
  /// Whether the window should always be on top of other windows or not.
  #[builder(default, setter(strip_option))]
  pub always_on_top: Option<bool>,
  /// Show window in the center of the screen..
  #[builder(default, setter(strip_option))]
  pub center: Option<bool>,
  /// Whether the window should have borders and bars or not.
  #[builder(default, setter(strip_option))]
  pub decorations: Option<bool>,
  /// Whether the file drop is enabled or not on the webview. By default it is enabled.
  ///
  /// Disabling it is required to use drag and drop on the frontend on Windows.
  #[builder(default, setter(strip_option))]
  pub file_drop_enabled: Option<bool>,
  /// Whether the window will be initially focused or not.
  #[builder(default, setter(strip_option))]
  pub focus: Option<bool>,
  /// Whether the window is in fullscreen mode or not.
  #[builder(default, setter(strip_option))]
  pub fullscreen: Option<bool>,
  /// The initial height.
  #[builder(default, setter(strip_option))]
  pub height: Option<u32>,
  /// If true, sets the window title to be hidden on macOS.
  #[builder(default, setter(strip_option))]
  pub hidden_title: Option<bool>,
  /// The maximum height. Only applies if maxWidth is also set.
  #[builder(default, setter(strip_option))]
  pub max_height: Option<u32>,
  /// The maximum width. Only applies if maxHeight is also set.
  #[builder(default, setter(strip_option))]
  pub max_width: Option<u32>,
  /// Whether the window should be maximized upon creation or not.
  #[builder(default, setter(strip_option))]
  pub maximized: Option<bool>,
  /// The minimum height. Only applies if minWidth is also set.
  #[builder(default, setter(strip_option))]
  pub min_height: Option<u32>,
  /// The minimum width. Only applies if minHeight is also set.
  #[builder(default, setter(strip_option))]
  pub min_width: Option<u32>,
  /// Whether the window is resizable or not.
  #[builder(default, setter(strip_option))]
  pub resizable: Option<bool>,
  /// Whether or not the window icon should be added to the taskbar.
  #[builder(default, setter(strip_option))]
  pub skip_task_bar: Option<bool>,
  /// Defines the window tabbing identifier on macOS.
  ///
  /// Windows with the same tabbing identifier will be grouped together. If the tabbing identifier is not set, automatic tabbing will be disabled.
  #[builder(default, setter(strip_option, into))]
  pub tabbing_identifier: Option<String>,
  /// The initial window theme. Defaults to the system theme.
  ///
  /// Only implemented on Windows and macOS 10.14+.
  #[builder(default, setter(strip_option))]
  pub theme: Option<Theme>,
  /// Window title.
  #[builder(default, setter(strip_option, into))]
  pub title: Option<String>,
  /// The style of the macOS title bar.
  #[builder(default, setter(strip_option))]
  pub title_bar_style: Option<TitleBarStyle>,
  /// Whether the window is transparent or not. Note that on macOS this requires the macos-private-api feature flag, enabled under tauri.conf.json > tauri > macOSPrivateApi. WARNING: Using private APIs on macOS prevents your application from being accepted to the App Store.
  #[builder(default, setter(strip_option))]
  pub transparent: Option<bool>,
  /// Remote URL or local file path to open.
  ///
  /// URL such as https://github.com/tauri-apps is opened directly on a Tauri window.
  /// - data: URL such as data:text/html,<html>... is only supported with the window-data-url Cargo feature for the tauri dependency.
  /// - local file path or route such as /path/to/page.html or /users is appended to the application URL (the devServer URL on development, or tauri://localhost/ and https://tauri.localhost/ on production).
  #[builder(default, setter(strip_option, into))]
  pub url: Option<String>,
  /// The user agent for the webview.
  #[builder(default, setter(strip_option, into))]
  pub user_agent: Option<String>,
  /// Whether the window should be immediately visible upon creation or not.
  #[builder(default, setter(strip_option))]
  pub visible: Option<bool>,
  /// The initial width.
  #[builder(default, setter(strip_option))]
  pub width: Option<u32>,
  /// The initial horizontal position. Only applies if x is also set.
  #[builder(default, setter(strip_option))]
  pub y: Option<u32>,
}

impl WebviewWindow {
  pub fn new(label: &str, options: Option<WindowOptions>) -> Self {
    let options = options
      .map(|options| serde_wasm_bindgen::to_value(options).unwrap())
      .unwrap_or_default();

    Self::new_js(label, options)
  }
}

#[wasm_bindgen]
extern "C" {

  /// Create new webview windows and get a handle to existing ones.
  ///
  /// Windows are identified by a label a unique identifier that can be used to reference it later. It may only contain alphanumeric characters a-zA-Z plus the following special characters -, /, : and _.
  pub type WebviewWindow;

  #[wasm_bindgen(constructor)]
  fn new_js(label: &str, options: JsValue) -> WebviewWindow;
}
