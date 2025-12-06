#![deny(clippy::all)]
use napi_derive::napi;

mod get_tokens;

#[napi(object)]
#[derive(Clone)]
pub struct Config {
  pub source: Vec<String>,
  pub cwd: Option<String>,
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

  #[napi]
  pub fn build(&self) {
    let tokens = self.fetch_tokens();

    println!("{:#?}", tokens);
  }

  fn fetch_tokens(&self) -> Vec<String> {
    let cwd = match &self.config.cwd {
      Some(path) => std::path::PathBuf::from(path),
      None => std::env::current_dir().unwrap(),
    };

    let path = get_tokens::get_tokens_path(&cwd, self.config.source.clone());
    get_tokens::get_tokens(path)
  }
}
