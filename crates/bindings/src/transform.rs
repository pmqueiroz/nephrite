use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi(object)]
#[derive(Clone)]
pub struct Transform {
  pub name: String,
  pub kind: TransformKind,
  #[napi(ts_type = "((token: ResolvedToken) => boolean)")]
  pub filter: Function<'static, serde_json::Value, bool>,
  #[napi(ts_type = "((token: ResolvedToken) => string)")]
  pub transform: Function<'static, serde_json::Value, String>,
}

pub struct RegisteredTransform {
  pub name: String,
  pub kind: TransformKind,
  pub filter: FunctionRef<serde_json::Value, bool>,
  pub transform: FunctionRef<serde_json::Value, String>,
}

#[napi(string_enum)]
#[derive(Clone)]
pub enum TransformKind {
  Attribute,
  Value,
}

#[napi(object)]
#[derive(Clone)]
pub struct TransformGroup {
  pub name: String,
  pub transforms: Vec<String>,
}
