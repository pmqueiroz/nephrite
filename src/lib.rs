#![deny(clippy::all)]
use kernel::_plus_100;
use napi_derive::napi;

#[napi]
pub fn plus_100(input: u32) -> u32 {
  _plus_100(input)
}

#[napi(object)]
#[derive(Clone)]
pub struct Config {
  pub source: Vec<String>,
}

#[napi]
pub struct Nephrite {
  config: Config,
}

#[napi]
impl Nephrite {
  #[napi(constructor)]
  pub fn new(config: Config) -> Self {
    Self { config }
  }

  #[napi]
  pub fn get_config(&self) -> Config {
    self.config.clone()
  }
}
