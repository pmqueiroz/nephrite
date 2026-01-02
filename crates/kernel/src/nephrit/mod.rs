mod transformers;

use bindings::token::{TokenAttrs, TransformedToken};
pub use transformers::*;

use crate::Config;
use std::collections::HashMap;

pub struct Nephrit<'env> {
  config: Config<'env>,
  transforms: HashMap<String, transformers::InternalTransformer>,
}

impl<'env> Nephrit<'env> {
  pub fn new(config: Config<'env>) -> Self {
    Self {
      config,
      transforms: HashMap::new(),
    }
  }

  pub fn get_config(&self) -> &Config<'env> {
    &self.config
  }

  pub fn register_transform(&mut self, name: String, transform: transformers::InternalTransformer) {
    self.transforms.insert(name, transform);
  }

  pub fn build(&self, transform: String) -> TransformedToken {
    if let Some(transformer) = self.transforms.get(&transform) {
      let transformer_func = (transformer.get_transformer)();

      return transformer_func.call(TransformedToken {
        name: "example_token".to_string(),
        attributes: TokenAttrs {
          _type: None,
          category: None,
          item: None,
          subitem: None,
          state: None,
        },
        file_path: "path/to/file".to_string(),
        is_source: true,
        key: "example_key".to_string(),
        original: serde_json::Value::Null,
        path: vec![],
        value: "example_value".to_string(),
      });
    }
    TransformedToken {
      name: "".to_string(),
      attributes: TokenAttrs {
        _type: None,
        category: None,
        item: None,
        subitem: None,
        state: None,
      },
      file_path: "".to_string(),
      is_source: false,
      key: "".to_string(),
      original: serde_json::Value::Null,
      path: vec![],
      value: "".to_string(),
    }
  }
}
