use wasm_bindgen::prelude::*;

#[must_use = "dropping this type will unsubscribe the shortcut"]
pub struct GlobalShortcutHandler(String, Closure<dyn FnMut(String)>);

impl Drop for GlobalShortcutHandler {
  fn drop(&mut self) {
    let shortcut = self.0.clone();

    wasm_bindgen_futures::spawn_local(async move {
      unregister(&shortcut).await;
    });
  }
}

impl GlobalShortcutHandler {
  pub fn shortcut(&self) -> &str {
    &self.0
  }
}

/// Determines whether the given shortcut is registered by this application or not.
pub async fn is_registered(shortcut: &str) -> bool {
  #[wasm_bindgen]
  extern "C" {
    #[wasm_bindgen(js_name = is_registered, js_namespace = ["__TAURI__", "globalShortcut"])]
    async fn is_registered(shortcut: &str) -> JsValue;
  }

  is_registered(shortcut).await.as_bool().unwrap()
}

/// Register a global shortcut.
pub async fn register(
  shortcut: &str,
  handler: impl FnMut(String) + 'static,
) -> GlobalShortcutHandler {
  #[wasm_bindgen]
  extern "C" {
    #[wasm_bindgen(js_namespace = ["__TAURI__", "globalShortcut"])]
    async fn register(shortcut: &str, handler: &JsValue);
  }

  let handler = Closure::new(handler);

  register(shortcut, handler.as_ref().unchecked_ref()).await;

  GlobalShortcutHandler(shortcut.into(), handler)
}

#[wasm_bindgen]
extern "C" {
  /// Unregister a global shortcut.
  #[wasm_bindgen(js_namespace = ["__TAURI__", "globalShortcut"])]
  pub async fn unregister(shortcut: &str);

  /// Unregisters all shortcuts registered by the application.
  #[wasm_bindgen(js_name = unregisterAll, js_namespace = ["__TAURI__", "globalShortcut"])]
  pub async fn unregister_all();
}
