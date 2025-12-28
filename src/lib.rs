#![deny(clippy::all)]
use bindings::{
  action, parser, platform, transform, Actions, Format, Parsers, Platforms, RegisteredFormat,
  RegisteredFormats, RegisteredTransforms, TransformGroups,
};
use kernel::{get_tokens_files, get_tokens_files_paths, TokensBucket};
use log::Logger;
use napi::bindgen_prelude::Env;
use napi_derive::napi;
use std::collections::HashMap;

#[napi]
pub struct Nephrite<'env> {
  config: NephriteConfig<'env>,
  transforms: RegisteredTransforms,
  transform_groups: TransformGroups,
  parsers: Parsers,
  actions: Actions,
  formats: RegisteredFormats,
  platforms: Platforms<'env>,
}

#[napi(object)]
#[derive(Clone)]
pub struct NephriteConfig<'platform> {
  pub source: Vec<String>,
  pub cwd: Option<String>,
  pub platforms: Vec<platform::Platform<'platform>>,
}

#[napi]
impl<'env> Nephrite<'env> {
  #[napi(constructor)]
  pub fn new(config: NephriteConfig<'env>) -> Self {
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
      formats: HashMap::new(),
      platforms,
    }
  }

  #[napi]
  pub fn get_config(&self) -> NephriteConfig<'_> {
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
    let platform = match self.platforms.get(platform_name) {
      Some(p) => p,
      None => {
        Logger::error(&format!("Platform '{}' not found", platform_name));
        return;
      }
    };

    let transform_group = match self.transform_groups.get(&platform.transform_group) {
      Some(t) => t,
      None => {
        Logger::error(&format!(
          "Transform group for platform '{}' not found",
          platform_name
        ));
        return;
      }
    };

    let collection = kernel::resolve_transformers(transform_group.clone(), &self.transforms);
    Logger::debug(&format!("Building platform: {}", platform.name.clone()));
    kernel::build(
      env,
      platform.clone(),
      collection,
      tokens_bucket,
      &self.formats,
    );
  }

  #[napi]
  pub fn register_transform(&mut self, transform: transform::Transform) {
    let name = transform.name.clone();
    self.transforms.insert(
      transform.name.clone(),
      transform::RegisteredTransform {
        name: transform.name.clone(),
        kind: transform.kind.clone(),
        transform: match transform.transform.create_ref() {
          Ok(transform) => transform,
          Err(e) => {
            Logger::error(&format!(
              "Failed to create transform reference for transform '{}': {:#?}",
              name, e
            ));
            std::process::exit(1);
          }
        },
        filter: match transform.filter.create_ref() {
          Ok(filter) => filter,
          Err(e) => {
            Logger::error(&format!(
              "Failed to create filter reference for transform '{}': {}",
              name, e
            ));
            std::process::exit(1);
          }
        },
      },
    );

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

  #[napi]
  pub fn register_format(&mut self, format: Format) {
    let name = format.name.clone();
    self.formats.insert(
      format.name.clone(),
      RegisteredFormat {
        name: format.name.clone(),
        format: match format.format.create_ref() {
          Ok(format_func) => format_func,
          Err(e) => {
            Logger::error(&format!(
              "Failed to create format reference for format '{}': {}",
              name, e
            ));
            std::process::exit(1);
          }
        },
      },
    );

    Logger::info(&format!("Registered format: {}", name));
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
