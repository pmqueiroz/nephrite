mod format_file;
mod resolve_transformers;
mod transform_tokens;
pub mod types;

pub use self::resolve_transformers::resolve_transformers;
use self::transform_tokens::transform_tokens;
use crate::{get_file_path, Config, TokensBucket};
use bindings::{Dictionary, Platform, RegisteredFormats};
use napi::bindgen_prelude::Env;

pub fn build<'build>(
  env: &Env,
  platform: Platform<'build>,
  collection: types::TransformersCollection<'build>,
  bucket: &TokensBucket,
  formatters: &RegisteredFormats,
  config: &Config<'build>,
) {
  let transformed_tokens = transform_tokens(collection, bucket, env);

  let dictionary = Dictionary {
    all_tokens: transformed_tokens,
  };

  for file in platform.files {
    let format = match formatters.get(&file.format) {
      Some(f) => f,
      None => {
        log::Logger::error(&format!(
          "Format '{}' not found for platform file '{}'",
          file.format, file.destination
        ));
        std::process::exit(1);
      }
    };

    let destination = get_file_path(&config.cwd, file.destination.clone());

    format_file::format_file(env, destination, format, &dictionary);
  }
}
