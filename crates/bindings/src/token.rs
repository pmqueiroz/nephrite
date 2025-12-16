use napi_derive::napi;

#[napi(object)]
#[derive(Clone)]
pub struct Token {
  #[napi(js_name = "type")]
  pub _type: String,
  pub value: String,
}
