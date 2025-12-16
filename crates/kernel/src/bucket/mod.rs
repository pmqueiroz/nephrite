use log::Logger;
use std::collections::HashMap;

mod token_ref;

#[derive(Debug)]
pub struct Token {
  pub path: String,
  pub original_value: serde_json::Value,
  pub value: serde_json::Value,
}

pub struct TokensBucket {
  pub tokens: HashMap<String, Token>,
}

impl TokensBucket {
  pub fn new(raw_tokens: Vec<serde_json::Value>) -> Self {
    let mut resolved_tokens = HashMap::new();
    let mut tokens_with_references = HashMap::new();

    Self::flatten(
      raw_tokens,
      "".into(),
      &mut resolved_tokens,
      &mut tokens_with_references,
    );

    Self::resolve_references(&mut resolved_tokens, &mut tokens_with_references);

    Self {
      tokens: resolved_tokens,
    }
  }

  pub fn print_tokens(&self) {
    println!("Tokens: {:#?}", &self.tokens);
  }

  fn resolve_references(
    resolved_tokens: &mut HashMap<String, Token>,
    tokens_with_references: &mut HashMap<String, Token>,
  ) {
    while !tokens_with_references.is_empty() {
      let keys: Vec<String> = tokens_with_references.keys().cloned().collect();
      let initial_count = tokens_with_references.len();

      for key in keys {
        if let Some(token) = tokens_with_references.get(&key) {
          if let Some(resolved_value) = token_ref::resolve_value_ref(&token.value, &resolved_tokens)
          {
            let resolved_token = Token {
              path: token.path.clone(),
              original_value: token.original_value.clone(),
              value: resolved_value,
            };
            resolved_tokens.insert(key.clone(), resolved_token);
            tokens_with_references.remove(&key);
          }
        }
      }

      if tokens_with_references.len() == initial_count {
        for (path, _) in tokens_with_references {
          Logger::error(&format!(
            "Referenced token does not exist for token at path '{}'",
            path
          ));
        }
        std::process::exit(1);
      }
    }
  }

  fn flatten(
    values: Vec<serde_json::Value>,
    prefix: String,
    resolved_map: &mut HashMap<String, Token>,
    references_map: &mut HashMap<String, Token>,
  ) {
    for value in values {
      match value {
        serde_json::Value::Object(obj) => {
          if obj.contains_key("value") || obj.contains_key("$value") {
            let token_value = obj.get("value").or_else(|| obj.get("$value")).unwrap();

            let token = Token {
              path: prefix.clone(),
              value: token_value.clone(),
              original_value: serde_json::Value::Object(obj.clone()),
            };

            if token_ref::is_value_ref(token_value) {
              references_map.insert(prefix.clone(), token);
            } else {
              resolved_map.insert(prefix.clone(), token);
            }
          } else {
            for (key, val) in obj {
              let new_prefix = if prefix.is_empty() {
                key.clone()
              } else {
                format!("{}.{}", prefix, key)
              };
              Self::flatten(vec![val], new_prefix, resolved_map, references_map);
            }
          }
        }
        _ => {
          Logger::warn(&format!(
            "Ignored unterminated token at path '{}': {:?}",
            prefix, value
          ));
        }
      }
    }
  }

  pub fn len(&self) -> usize {
    self.tokens.len()
  }
}
