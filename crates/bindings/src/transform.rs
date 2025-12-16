use napi::bindgen_prelude::*;
use napi_derive::napi;

use crate::token::Token;

#[napi(object)]
#[derive(Clone)]
pub struct Transform {
  pub name: String,
  pub kind: TransformKind,
  pub filter: Function<'static, Token, bool>,
  pub transform: Function<'static, Token, String>,
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
