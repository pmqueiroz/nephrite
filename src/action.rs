use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi(object)]
pub struct Action {
  pub name: String,
  #[napi(js_name = "do")]
  pub _do: Function<'static, (), ()>,
  pub undo: Function<'static, (), ()>,
}
