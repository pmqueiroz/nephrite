#![deny(clippy::all)]
use bindings::{action, parser, platform, transform};
use kernel::{get_tokens_files, get_tokens_files_paths};
use napi::bindgen_prelude::Env;
use napi_derive::napi;
use std::collections::HashMap;

#[napi]
pub struct Nephrite {
  config: Config,
  transforms: HashMap<String, transform::Transform>,
  transform_groups: HashMap<String, transform::TransformGroup>,
  parsers: Vec<parser::RegisteredParser>,
  actions: HashMap<String, action::Action>,
  platforms: HashMap<String, platform::Platform>,
}

#[napi(object)]
#[derive(Clone)]
pub struct Config {
  pub source: Vec<String>,
  pub cwd: Option<String>,
  pub platforms: Vec<platform::Platform>,
}

#[napi]
impl Nephrite {
  #[napi(constructor)]
  pub fn new(config: Config) -> Self {
    let mut platforms = HashMap::new();

    for platform in &config.platforms {
      platforms.insert(platform.name.clone(), platform.clone());
    }

    Self {
      config,
      transforms: HashMap::new(),
      transform_groups: HashMap::new(),
      parsers: Vec::new(),
      actions: HashMap::new(),
      platforms,
    }
  }

  #[napi]
  pub fn get_config(&self) -> Config {
    self.config.clone()
  }

  #[napi]
  pub fn build(&self, platform_name: String, env: &Env) {
    println!("Building for platform: {}", platform_name);
    let tokens_files = self.fetch_tokens_files();
    let parsed_files = kernel::parse_files(tokens_files, &self.parsers, env);
    for parsed_file in &parsed_files {
      println!("Token file path: {:#?}", parsed_file);
    }

    let platform = self.platforms.get(&platform_name);

    match platform {
      Some(p) => println!("Using transform_group: {}", p.transform_group),
      None => println!("Platform '{}' not found", platform_name),
    }
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
  pub fn register_parser<'parser>(&mut self, parser: parser::Parser) {
    let name = parser.name.clone();
    let registered_parser = parser::RegisteredParser {
      name: parser.name,
      pattern: parser.pattern,
      parser: parser.parser.create_ref().unwrap(),
    };
    self.parsers.push(registered_parser);

    println!("Registered parser: {}", name)
  }

  #[napi]
  pub fn register_action(&mut self, action: action::Action) {
    let name = action.name.clone();
    self.actions.insert(action.name.clone(), action);
    println!("Registered action: {}", name)
  }

  fn fetch_tokens_files(&self) -> Vec<bindings::parser::TokenFile> {
    let cwd = match &self.config.cwd {
      Some(path) => std::path::PathBuf::from(path),
      None => std::env::current_dir().unwrap(),
    };

    let path = get_tokens_files_paths(&cwd, self.config.source.clone());
    get_tokens_files(path)
  }
}
