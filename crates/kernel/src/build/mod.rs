use bindings::platform::Platform;
use log::Logger;

mod resolve_transformers;
pub mod types;

use crate::TokensBucket;

pub use self::resolve_transformers::resolve_transformers;

pub fn build(platform: Platform, collection: types::TransformersCollection, bucket: &TokensBucket) {
  Logger::info(&format!("Building platform: {}", platform.name));
  Logger::info(&format!("Using {} tokens", bucket.len()));

  for transformer in collection {
    Logger::info(&format!("Applying transformer: {}", transformer.name));
    // Here you would apply the transformer logic
  }
}
