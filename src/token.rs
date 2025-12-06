use napi_derive::napi;

#[napi(object)]
pub struct Token {
  #[napi(js_name = "type")]
  pub _type: String,
  pub value: String,
}
