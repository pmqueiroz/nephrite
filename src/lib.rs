#![deny(clippy::all)]
use napi_derive::napi;
use std::collections::HashMap;

mod action;
mod get_tokens;
mod parser;
mod token;
mod transform;

#[napi]
pub struct Nephrite {
  config: Config,
  transforms: HashMap<String, transform::Transform>,
  transform_groups: HashMap<String, transform::TransformGroup>,
  parsers: HashMap<String, parser::Parser>,
  actions: HashMap<String, action::Action>,
}

#[napi(object)]
#[derive(Clone)]
pub struct Config {
  pub source: Vec<String>,
  pub cwd: Option<String>,
}

#[napi]
impl Nephrite {
  #[napi(constructor)]
  pub fn new(config: Config) -> Self {
    Self {
      config,
      transforms: HashMap::new(),
      transform_groups: HashMap::new(),
      parsers: HashMap::new(),
      actions: HashMap::new(),
    }
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

  #[napi]
  pub fn register_transform(&mut self, transform: transform::Transform) {
    let name = transform.name.clone();
    self.transforms.insert(transform.name.clone(), transform);

    println!("Registered transform: {}", name)
  }

  #[napi]
  pub fn register_transform_group(&mut self, transform_group: transform::TransformGroup) {
    let name = transform_group.name.clone();
    self
      .transform_groups
      .insert(transform_group.name.clone(), transform_group);

    println!("Registered transform group: {}", name)
  }

  #[napi]
  pub fn register_parser(&mut self, parser: parser::Parser) {
    let name = parser.name.clone();
    self.parsers.insert(parser.name.clone(), parser);
    println!("Registered parser: {}", name)
  }

  #[napi]
  pub fn register_action(&mut self, action: action::Action) {
    let name = action.name.clone();
    self.actions.insert(action.name.clone(), action);
    println!("Registered action: {}", name)
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
