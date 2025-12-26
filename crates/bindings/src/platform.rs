use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi(object)]
#[derive(Clone)]
pub struct Platform<'platform> {
  pub name: String,
  pub transform_group: String,
  pub build_path: String,
  pub files: Vec<PlatformFile<'platform>>,
}

#[napi(object)]
#[derive(Clone)]
pub struct PlatformFile<'platform> {
  pub destination: String,
  pub filter: Option<Function<'platform, serde_json::Value, bool>>,
  pub format: String,
}
