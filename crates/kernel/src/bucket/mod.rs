use bindings::token::ResolvedToken;
use log::Logger;
use std::collections::HashMap;

mod token_ref;

pub struct TokensBucket {
  pub tokens: HashMap<String, ResolvedToken>,
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
    resolved_tokens: &mut HashMap<String, ResolvedToken>,
    tokens_with_references: &mut HashMap<String, ResolvedToken>,
  ) {
    while !tokens_with_references.is_empty() {
      let keys: Vec<String> = tokens_with_references.keys().cloned().collect();
      let initial_count = tokens_with_references.len();

      for key in keys {
        if let Some(token) = tokens_with_references.get(&key) {
          if let Some(resolved_value) = token_ref::resolve_value_ref(&token.value, &resolved_tokens)
          {
            let resolved_token = ResolvedToken {
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
    resolved_map: &mut HashMap<String, ResolvedToken>,
    references_map: &mut HashMap<String, ResolvedToken>,
  ) {
    for value in values {
      match value {
        serde_json::Value::Object(obj) => {
          if obj.contains_key("value") || obj.contains_key("$value") {
            let token_value = obj.get("value").or_else(|| obj.get("$value")).unwrap();

            let token = ResolvedToken {
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

  pub fn iter(&self) -> impl Iterator<Item = (&String, &ResolvedToken)> {
    self.tokens.iter()
  }

  pub fn keys(&self) -> impl Iterator<Item = &String> {
    self.tokens.keys()
  }

  pub fn values(&self) -> impl Iterator<Item = &ResolvedToken> {
    self.tokens.values()
  }

  pub fn iter_mut(&mut self) -> impl Iterator<Item = (&String, &mut ResolvedToken)> {
    self.tokens.iter_mut()
  }

  pub fn values_mut(&mut self) -> impl Iterator<Item = &mut ResolvedToken> {
    self.tokens.values_mut()
  }
}

impl IntoIterator for TokensBucket {
  type Item = (String, ResolvedToken);
  type IntoIter = std::collections::hash_map::IntoIter<String, ResolvedToken>;

  fn into_iter(self) -> Self::IntoIter {
    self.tokens.into_iter()
  }
}

impl<'a> IntoIterator for &'a TokensBucket {
  type Item = (&'a String, &'a ResolvedToken);
  type IntoIter = std::collections::hash_map::Iter<'a, String, ResolvedToken>;

  fn into_iter(self) -> Self::IntoIter {
    self.tokens.iter()
  }
}

impl<'a> IntoIterator for &'a mut TokensBucket {
  type Item = (&'a String, &'a mut ResolvedToken);
  type IntoIter = std::collections::hash_map::IterMut<'a, String, ResolvedToken>;

  fn into_iter(self) -> Self::IntoIter {
    self.tokens.iter_mut()
  }
}
