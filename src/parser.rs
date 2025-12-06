use napi_derive::napi;

#[napi(object)]
pub struct Parser {
  pub name: String,
  pub pattern: String,
  pub transforms: Vec<String>,
}
