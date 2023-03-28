use core::fmt;
use std::marker::PhantomData;
use wasm_bindgen::prelude::*;

pub struct InvokeCommand<Args = (), Res = (), Err = ()> {
  name: &'static str,
  _args: PhantomData<Args>,
  _res: PhantomData<Res>,
  _err: PhantomData<Err>,
}

impl<Args, Res, Err> Clone for InvokeCommand<Args, Res, Err> {
  fn clone(&self) -> Self {
    Self {
      name: self.name,
      _args: self._args,
      _res: self._res,
      _err: self._err,
    }
  }
}

impl<Args, Res, Err> Copy for InvokeCommand<Args, Res, Err> {}

impl<Args, Res, Err> fmt::Debug for InvokeCommand<Args, Res, Err> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("InvokeCommand")
      .field("name", &self.name)
      .field("arguments_type", &core::any::type_name::<Args>())
      .field("response_type", &core::any::type_name::<Res>())
      .field("error_type", &core::any::type_name::<Err>())
      .finish()
  }
}

impl<Args, Res, Err> InvokeCommand<Args, Res, Err> {
  pub const fn new(name: &'static str) -> Self {
    Self {
      name,
      _args: PhantomData,
      _res: PhantomData,
      _err: PhantomData,
    }
  }
}

impl<Args, Res, Err> InvokeCommand<Args, Res, Err> {
  pub async fn invoke_with_serde(self, args: &Args) -> Result<Res, Err>
  where
    Args: serde::Serialize,
    Res: for<'de> serde::Deserialize<'de>,
    Err: for<'de> serde::Deserialize<'de>,
  {
    let args = serde_wasm_bindgen::to_value(args).unwrap();

    match invoke_js(self.name, args).await {
      Ok(res) => serde_wasm_bindgen::from_value(res).unwrap(),
      Err(err) => serde_wasm_bindgen::from_value(err).unwrap(),
    }
  }
}

impl<Args, Res> InvokeCommand<Args, Res, JsValue> {
  pub async fn invoke_with_serde_js_err(
    self,
    args: &Args,
  ) -> Result<Res, JsValue>
  where
    Args: serde::Serialize,
    Res: for<'de> serde::Deserialize<'de>,
  {
    let args = serde_wasm_bindgen::to_value(args).unwrap();

    invoke_js(self.name, args)
      .await
      .map(|res| serde_wasm_bindgen::from_value(res).unwrap())
  }
}

impl InvokeCommand<(), (), ()> {
  pub async fn invoke(self) {
    invoke_js(self.name, JsValue::UNDEFINED)
      .await
      .unwrap_or_else(|err| {
        panic!("`{}` invocation not to return `Err`: {err:#?}", self.name)
      });
  }

  pub fn invoke_sync(self) {
    wasm_bindgen_futures::spawn_local(async move {
      invoke_js(self.name, JsValue::UNDEFINED)
        .await
        .unwrap_or_else(|err| {
          panic!("`{}` invocation not to return `Err`: {err:#?}", self.name)
        });
    });
  }
}

impl InvokeCommand<JsValue, JsValue, JsValue> {
  pub async fn invoke_raw(self, args: JsValue) -> Result<JsValue, JsValue> {
    invoke_js(self.name, args).await
  }
}

#[wasm_bindgen]
extern "C" {
  /// Convert a device file path to an URL that can be loaded by the webview. Note that asset: and https://asset.localhost must be added to tauri.security.csp in tauri.conf.json. Example CSP value: "csp": "default-src 'self'; img-src 'self' asset: https://asset.localhost" to use the asset protocol on image sources.
  ///
  /// Additionally, asset must be added to tauri.allowlist.protocol in tauri.conf.json and its access scope must be defined on the assetScope array on the same protocol object.
  ///
  /// Returns the URL that can be used as source on the webview.
  #[wasm_bindgen(js_name = convertFileSrc, js_namespace = ["__TAURI__", "tauri"])]
  pub fn convert_ile_src(file_path: &str, protocol: Option<&str>) -> String;

  #[wasm_bindgen(js_name = invoke, js_namespace = ["__TAURI__", "tauri"], catch)]
  async fn invoke_js(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;

  /// Transforms a callback function to a string identifier that can be passed to the backend. The backend uses the identifier to eval() the callback.
  ///
  /// Returns a unique identifier associated with the callback function.
  pub fn transform_callback(
    callback: &js_sys::Function,
    once: Option<bool>,
  ) -> usize;

}
