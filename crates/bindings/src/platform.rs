use napi_derive::napi;

#[napi(object)]
#[derive(Clone)]
pub struct Platform {
  pub name: String,
  pub transform_group: String,
  pub build_path: String,
}
