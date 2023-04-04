use wasm_bindgen::prelude::*;

/// Gets the clipboard content as plain text.
pub async fn read_text() -> String {
  #[wasm_bindgen]
  extern "C" {
    #[wasm_bindgen(js_name = readText, js_namespace = ["__TAURI__", "clipboard"])]
    async fn read_text() -> JsValue;
  }

  read_text()
    .await
    .unchecked_into::<js_sys::JsString>()
    .into()
}

#[wasm_bindgen]
extern "C" {
  /// Writes plain text to the clipboard.
  #[wasm_bindgen(js_name = writeText, js_namespace = ["__TAURI__", "clipboard"])]
  pub async fn write_text(text: &str);
}
