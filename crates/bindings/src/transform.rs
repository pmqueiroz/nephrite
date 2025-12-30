use napi::bindgen_prelude::*;
use napi_derive::napi;

use crate::token::TransformedToken;

#[napi(object)]
#[derive(Clone)]
pub struct Transform {
  pub name: String,
  pub kind: TransformKind,
  pub filter: Function<'static, TransformedToken, bool>,
  pub transform: Function<'static, TransformedToken, String>,
}

pub struct RegisteredTransform {
  pub name: String,
  pub kind: TransformKind,
  pub filter: FunctionRef<TransformedToken, bool>,
  pub transform: FunctionRef<TransformedToken, String>,
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
