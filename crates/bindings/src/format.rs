use crate::dictionary::Dictionary;
use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi(object)]
#[derive(Clone)]
pub struct FormatArguments {
  pub dictionary: Dictionary,
}

#[napi(object)]
#[derive(Clone)]
pub struct Format {
  pub name: String,
  pub format: Function<'static, FormatArguments, String>,
}

pub struct RegisteredFormat {
  pub name: String,
  pub format: FunctionRef<FormatArguments, String>,
}
