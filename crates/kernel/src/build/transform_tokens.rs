use std::collections::HashMap;

use bindings::token::TransformedToken;
use log::Logger;
use napi::Env;

use crate::{build::types, TokensBucket};

pub fn transform_tokens<'transforms>(
  collection: types::TransformersCollection<'transforms>,
  bucket: &TokensBucket,
  env: &Env,
) -> Vec<TransformedToken> {
  let mut transformed_tokens: HashMap<String, TransformedToken> = HashMap::new();

  for transformer in collection {
    Logger::info(&format!("Applying transformer: {}", transformer.name));
    for token in bucket.iter() {
      if let Ok(filter_func) = transformer.filter.borrow_back(env) {
        let token_ref = match transformed_tokens.get(&token.path) {
          Some(t) => t.original.clone(),
          None => token.clone(),
        };

        let token_json = serde_json::to_value(token_ref).unwrap_or(serde_json::Value::Null);
        let bool_result = filter_func.call(token_json.clone());
        if let Ok(boolean) = bool_result {
          if boolean {
            Logger::debug(&format!(
              "Transformer '{}' matched token: {}",
              transformer.name, token.path
            ));
            if let Ok(transform_func) = transformer.transform.borrow_back(env) {
              let transformed_result = transform_func.call(token_json);
              match transformed_result {
                Ok(transformed_code) => {
                  transformed_tokens.insert(
                    token.path.clone(),
                    TransformedToken {
                      original: token.clone(),
                      value: transformed_code,
                    },
                  );
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

  Logger::debug(&format!(
    "Finished transforming. Transformed {} tokens.",
    transformed_tokens.len()
  ));

  transformed_tokens.values().cloned().collect()
}
