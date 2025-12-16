use bindings::platform::Platform;
use log::Logger;

mod resolve_transformers;
pub mod types;

pub use self::resolve_transformers::resolve_transformers;

pub fn build(platform: Platform, collection: types::TransformersCollection) {
  Logger::info(&format!("Building platform: {}", platform.name));

  for transformer in collection {
    Logger::info(&format!("Applying transformer: {}", transformer.name));
    // Here you would apply the transformer logic
  }
}
