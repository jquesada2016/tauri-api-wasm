macro_rules! declare_type {
  (
    $( #[extends = $extends:ty] )?
    $vis:vis struct $name:ident {
      $(
        $( @postfix($prop_postfix:ident) )?
        $prop_vis:vis $prop_name:ident : $prop_ty:ty
      ),* $(,)?
    }

    impl $_name:ident {
      $(
        $( @postfix($method_postfix:ident) )?
        $( #[$($wasm_method_opts:tt)*] )*
        $method_vis:vis fn $method:ident(
          $( &self $(,)? )?
          $($arg:ident : $arg_ty:ty),*
        ) $( -> $method_output:ty )?;
      )*

      $(
        @async

        $(
          $( @postfix($async_method_postfix:ident) )?
          $( #[$($async_wasm_method_opts:tt)*] )*
          $async_method_vis:vis async fn $async_method:ident(
            $( &self $(,)? )?
            $($async_arg:ident : $async_arg_ty:ty),*
          ) $( -> $async_method_output:ty )?;
        )*
      )?

      $(
        @async_catch

        $(
          $( @postfix($async_catch_method_postfix:ident) )?
          $( #[$($async_catch_wasm_method_opts:tt)*] )*
          $async_catch_method_vis:vis async fn $async_catch_method:ident(
            $( &self $(,)? )?
            $($async_catch_arg:ident : $async_catch_arg_ty:ty),*
          ) -> Result<$async_catch_method_output_ok:ty, $async_catch_method_output_err:ty>;
        )*
      )?
    }
  ) => {
    paste::paste! {
      impl $name {
        $(
          $(
            $async_method_vis async fn [<$async_method:snake $(_ $async_method_postfix)?>](
              &self,
              $( $async_arg: $async_arg_ty ),*
            ) $( -> $async_method_output )? {
              let _value = self
                .[<$async_method:snake __js>]($( $async_arg ),*)
                .await;

              $(
                let _ = stringify!($async_method_output);

                _value.unchecked_into()
              )?
            }
          )*
        )?

        $(
          $(
            $async_catch_method_vis async fn [<$async_catch_method:snake $(_ $async_catch_method_postfix)?>](
              &self,
              $( $async_catch_arg: $async_catch_arg_ty ),*
            ) -> Result<$async_catch_method_output_ok, $async_catch_method_output_err> {
              self
                .[<$async_catch_method:snake __js>]($( $async_catch_arg ),*)
                .await
                .map(|ok| ok.unchecked_into())
                .map_err(|err| err.unchecked_into())
            }
          )*
        )?
      }

      #[wasm_bindgen]
      extern "C" {
        #[wasm_bindgen($( extends = $extends )?)]
        $vis type $name;

        $(
          #[wasm_bindgen(js_name = $prop_name, method, getter)]
          $prop_vis fn [<$prop_name:snake $(_ $prop_postfix)?>](this: &$name) -> $prop_ty;
        )*

        $(
          #[wasm_bindgen(
            js_name = $method,
            method,
            $( $($wasm_method_opts)* ),*
          )]
          $method_vis fn [<$method:snake $(_ $method_postfix)?>](
            this: &$name,
            $( $arg: $arg_ty ),*
          ) $( -> $method_output )?;
        )*

        $(
          $(
            #[wasm_bindgen(
              js_name = $async_method,
              method,
              $( $($async_wasm_method_opts)* ),*
            )]
            $async_method_vis async fn [<$async_method:snake __js>](
              this: &$name,
              $( $async_arg: $async_arg_ty ),*
            ) -> JsValue;
          )*
        )?

        $(
          $(
            #[wasm_bindgen(
              js_name = $async_catch_method,
              method,
              catch,
              $( $($async_catch_wasm_method_opts)* ),*
            )]
            $async_catch_method_vis async fn [<$async_catch_method:snake __js>](
              this: &$name,
              $( $async_catch_arg: $async_catch_arg_ty ),*
            ) -> Result<JsValue, JsValue>;
          )*
        )?
      }
    }
  };
}

use wasm_bindgen::prelude::*;

pub trait FromJsValue {
  fn from_js_value(value: JsValue) -> Self;
}

impl<T> FromJsValue for T
where
  T: JsCast,
{
  fn from_js_value(value: JsValue) -> Self {
    value.unchecked_into()
  }
}
