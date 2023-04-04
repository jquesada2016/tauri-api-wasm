use crate::event::{
  Event,
  ListenUnsubscriber,
};
use wasm_bindgen::prelude::*;

declare_type! {
  pub struct WebviewWindowHandle {
    pub label: String,
  }

  impl WebviewWindowHandle {
  }
}

impl WebviewWindowHandle {
  pub async fn listen(
    &self,
    event: &str,
    handler: impl FnMut(Event) + 'static,
  ) -> ListenUnsubscriber<dyn FnMut(Event)> {
    #[wasm_bindgen]
    extern "C" {
      #[wasm_bindgen(js_name = listen, method)]
      async fn listen_js(
        this: &WebviewWindowHandle,
        event: &str,
        handler: &JsValue,
      ) -> JsValue;
    }

    let closure = Closure::new(handler);

    let unsub = self
      .listen_js(event, closure.as_ref())
      .await
      .unchecked_into();

    ListenUnsubscriber(closure, unsub)
  }

  pub async fn listen_serde<T: for<'de> serde::Deserialize<'de>>(
    &self,
    event: &str,
    mut handler: impl FnMut(Result<T, serde_wasm_bindgen::Error>, Event) + 'static,
  ) -> ListenUnsubscriber<dyn FnMut(Event)> {
    self
      .listen(event, move |e| {
        let payload = e.payload();

        let payload = serde_wasm_bindgen::from_value(payload);

        handler(payload, e);
      })
      .await
  }

  pub async fn once(
    &self,
    event: &str,
    handler: impl FnOnce(Event) + 'static,
  ) -> ListenUnsubscriber<dyn FnMut(Event)> {
    #[wasm_bindgen]
    extern "C" {
      #[wasm_bindgen(js_name = once, method)]
      async fn once_js(
        this: &WebviewWindowHandle,
        event: &str,
        handler: &JsValue,
      ) -> JsValue;
    }

    let closure = Closure::once(handler);

    let unsub = self.once_js(event, closure.as_ref()).await.unchecked_into();

    ListenUnsubscriber(closure, unsub)
  }

  pub async fn once_serde<T: for<'de> serde::Deserialize<'de>>(
    &self,
    event: &str,
    handler: impl FnOnce(Result<T, serde_wasm_bindgen::Error>, Event) + 'static,
  ) -> ListenUnsubscriber<dyn FnMut(Event)> {
    self
      .once(event, move |e| {
        let payload = serde_wasm_bindgen::from_value(e.payload());

        handler(payload, e);
      })
      .await
  }

  pub async fn emit(&self, event: &str, payload: JsValue) {
    #[wasm_bindgen]
    extern "C" {
      #[wasm_bindgen(js_name = emit, method)]
      async fn emit_js(
        this: &WebviewWindowHandle,
        event: &str,
        payload: JsValue,
      );
    }

    self.emit_js(event, payload).await;
  }

  pub async fn emit_serde<T: serde::Serialize>(
    &self,
    event: &str,
    payload: &T,
  ) {
    let payload = serde_wasm_bindgen::to_value(payload).unwrap();

    self.emit(event, payload).await;
  }
}

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(constructor, js_namespace = ["__TAURI__", "window"])]
  pub fn new(label: &str) -> WebviewWindowHandle;
}
