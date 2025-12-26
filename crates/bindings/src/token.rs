use napi_derive::napi;

#[napi(object)]
#[derive(Clone)]
pub struct Token {
  #[napi(js_name = "type")]
  pub _type: String,
  pub value: String,
}
#[napi(object)]
#[derive(Debug, Clone, serde::Serialize)]
pub struct ResolvedToken {
  pub path: String,
  #[napi(ts_type = "Token")]
  pub original_value: serde_json::Value,
  pub value: serde_json::Value,
}

#[derive(Debug, Clone)]
pub struct TransformedToken {
  pub original: ResolvedToken,
  pub value: String,
}
