use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi(object)]
#[derive(Clone)]
pub struct TokenFile {
  pub path: String,
  pub content: String,
}

#[napi(object)]
#[derive(Clone)]
pub struct Parser<'parser> {
  pub name: String,
  pub pattern: String,
  pub parser: Function<'parser, TokenFile, String>,
}

pub struct RegisteredParser {
  pub name: String,
  pub pattern: String,
  pub parser: FunctionRef<TokenFile, String>,
}
