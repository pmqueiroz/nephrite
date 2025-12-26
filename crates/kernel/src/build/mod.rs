use bindings::platform::Platform;
use log::Logger;
use napi::bindgen_prelude::Env;

mod resolve_transformers;
pub mod types;

use crate::TokensBucket;

pub use self::resolve_transformers::resolve_transformers;

pub fn build(
  platform: Platform,
  collection: types::TransformersCollection,
  bucket: &TokensBucket,
  env: &Env,
) {
  Logger::info(&format!("Building platform: {}", platform.name));
  Logger::info(&format!("Using {} tokens", bucket.len()));
  // let mut transformed_tokens = Vec::new();

  for transformer in collection {
    Logger::info(&format!("Applying transformer: {}", transformer.name));
    for token in bucket.iter() {
      Logger::debug(&format!("Processing token: {:?}", token));

      if let Ok(filter_func) = transformer.filter.borrow_back(env) {
        let token_json = serde_json::to_value(token).unwrap_or(serde_json::Value::Null);
        let bool_result = filter_func.call(token_json.clone());
        if let Ok(boolean) = bool_result {
          if boolean {
            Logger::info(&format!(
              "Transformer '{}' matched token: {:?}",
              transformer.name, token
            ));
            if let Ok(transform_func) = transformer.transform.borrow_back(env) {
              let transformed_result = transform_func.call(token_json);
              match transformed_result {
                Ok(transformed_code) => {
                  Logger::info(&format!(
                    "Transformed token: {:?} to code: {}",
                    token, transformed_code
                  ));
                  // Here you can store or use the transformed code as needed
                }
                Err(e) => {
                  Logger::error(&format!(
                    "Error transforming token {:?} with transformer '{}': {:?}",
                    token, transformer.name, e
                  ));
                }
              }
            } else {
              Logger::error(&format!(
                "Failed to borrow transform function for transformer '{}'",
                transformer.name
              ));
            }
          }
        }
      }
    }
  }
}
