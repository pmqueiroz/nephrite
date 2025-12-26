#![deny(clippy::all)]
use bindings::{
  action, parser, platform, transform, Actions, Parsers, Platforms, TransformGroups, Transforms,
};
use kernel::{get_tokens_files, get_tokens_files_paths, TokensBucket};
use log::Logger;
use napi::bindgen_prelude::Env;
use napi_derive::napi;
use std::collections::HashMap;

#[napi]
pub struct Nephrite<'env> {
  config: Config<'env>,
  transforms: Transforms,
  transform_groups: TransformGroups,
  parsers: Parsers,
  actions: Actions,
  platforms: Platforms<'env>,
}

#[napi(object)]
#[derive(Clone)]
pub struct Config<'platform> {
  pub source: Vec<String>,
  pub cwd: Option<String>,
  pub platforms: Vec<platform::Platform<'platform>>,
}

#[napi]
impl<'env> Nephrite<'env> {
  #[napi(constructor)]
  pub fn new(config: Config<'env>) -> Self {
    Logger::init();

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
  pub fn get_config(&self) -> Config<'_> {
    self.config.clone()
  }

  fn generate_bucket(&self, env: &Env) -> TokensBucket {
    let tokens_files = self.fetch_tokens_files();
    let parsed_files = kernel::parse_files(tokens_files, &self.parsers, env);
    TokensBucket::new(parsed_files)
  }

  #[napi]
  pub fn build_platform(&self, platform_name: String, env: &Env) {
    let tokens_bucket = self.generate_bucket(env);

    self.build_single_platform(&platform_name, &tokens_bucket, env);
  }

  #[napi]
  pub fn build_all(&self, env: &Env) {
    Logger::info("Building all platforms in parallel");

    let tokens_bucket = self.generate_bucket(env);

    let platform_names: Vec<String> = self.platforms.keys().cloned().collect();

    platform_names.iter().for_each(|platform_name| {
      self.build_single_platform(&platform_name, &tokens_bucket, env);
    });

    Logger::info("All platforms built successfully");
  }

  fn build_single_platform(&self, platform_name: &str, tokens_bucket: &TokensBucket, env: &Env) {
    let platform = self.platforms.get(platform_name);

    let transform_group = match platform {
      Some(p) => self.transform_groups.get(&p.transform_group),
      None => {
        Logger::error(&format!("Platform '{}' not found", platform_name));
        return;
      }
    };

    match transform_group {
      Some(t) => {
        let collection = kernel::resolve_transformers(t.clone(), self.transforms.clone());
        kernel::build(platform.unwrap().clone(), collection, tokens_bucket, env);
      }
      None => {
        Logger::error(&format!(
          "Transform group for platform '{}' not found",
          platform_name
        ));
      }
    }
  }

  #[napi]
  pub fn register_transform(&mut self, transform: transform::Transform) {
    let name = transform.name.clone();
    self.transforms.insert(transform.name.clone(), transform);

    Logger::info(&format!("Registered transform: {}", name));
  }

  #[napi]
  pub fn register_transform_group(&mut self, transform_group: transform::TransformGroup) {
    let name = transform_group.name.clone();
    self
      .transform_groups
      .insert(transform_group.name.clone(), transform_group);

    Logger::info(&format!("Registered transform group: {}", name));
  }

  #[napi]
  pub fn register_parser<'parser>(&mut self, parser: parser::Parser) {
    let name = parser.name.clone();
    let registered_parser = parser::RegisteredParser {
      name: parser.name,
      pattern: parser.pattern,
      parser: match parser.parser.create_ref() {
        Ok(parser) => parser,
        Err(e) => {
          Logger::error(&format!(
            "Failed to create parser reference for parser '{}': {}",
            name, e
          ));
          std::process::exit(1);
        }
      },
    };
    self.parsers.push(registered_parser);

    Logger::info(&format!("Registered parser: {}", name));
  }

  #[napi]
  pub fn register_action(&mut self, action: action::Action) {
    let name = action.name.clone();
    self.actions.insert(action.name.clone(), action);
    Logger::info(&format!("Registered action: {}", name));
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
