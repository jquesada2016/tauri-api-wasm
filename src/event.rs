use wasm_bindgen::prelude::*;

declare_type! {
  pub struct Event {
    pub event: String,
    pub windowLabel: String,
    pub id: js_sys::Number,
    pub payload: JsValue,
  }

  impl Event {}
}

pub struct ListenUnsubscriber<T: ?Sized>(
  pub(crate) Closure<T>,
  pub(crate) js_sys::Function,
);

impl<T: ?Sized> Drop for ListenUnsubscriber<T> {
  fn drop(&mut self) {
    let _ = self.1.call0(&JsValue::UNDEFINED);
  }
}

pub async fn listen(
  event: &str,
  handler: impl FnMut(Event) + 'static,
) -> ListenUnsubscriber<dyn FnMut(Event)> {
  #[wasm_bindgen]
  extern "C" {
    #[wasm_bindgen(js_name = listen, js_namespace = ["__TAURI__", "event"])]
    async fn listen_js(event: &str, handler: &js_sys::Function) -> JsValue;
  }

  let handler = Closure::new(handler);

  let unsubscribe = listen_js(event, handler.as_ref().unchecked_ref()).await;

  ListenUnsubscriber(handler, unsubscribe.unchecked_into())
}

pub async fn listen_serde<T: for<'de> serde::Deserialize<'de>>(
  event: &str,
  mut handler: impl FnMut(Result<T, serde_wasm_bindgen::Error>, Event) + 'static,
) -> ListenUnsubscriber<dyn FnMut(Event)> {
  listen(event, move |e| {
    let payload = serde_wasm_bindgen::from_value(e.payload());

    handler(payload, e)
  })
  .await
}

pub async fn once(event: &str, handler: impl FnOnce(Event) + 'static) {
  #[wasm_bindgen]
  extern "C" {
    #[wasm_bindgen(js_name = once, js_namespace = ["__TAURI__", "event"])]
    async fn once_js(event: &str, handler: &js_sys::Function) -> JsValue;
  }

  let handler = Closure::once(handler).into_js_value();

  once_js(event, handler.unchecked_ref()).await;
}

pub async fn once_serde<T: for<'de> serde::Deserialize<'de>>(
  event: &str,
  handler: impl FnOnce(Result<T, serde_wasm_bindgen::Error>, Event) + 'static,
) {
  once(event, |e| {
    let payload = serde_wasm_bindgen::from_value(e.payload());

    handler(payload, e);
  })
  .await;
}

#[wasm_bindgen]
extern "C" {

  /// Emits an event to the backend.
  #[wasm_bindgen(js_namespace = ["__TAURI__", "event"])]
  pub async fn emit(event: &str, payload: &JsValue);
}
