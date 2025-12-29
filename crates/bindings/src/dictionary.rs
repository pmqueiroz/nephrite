use napi_derive::napi;

use crate::token::TransformedToken;

#[napi(object)]
#[derive(Clone)]
pub struct Dictionary {
  pub tokens: serde_json::Value,
  // pub unfiltered_tokens: serde_json::Value,
  pub all_tokens: Vec<TransformedToken>,
}
