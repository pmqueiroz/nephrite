use napi_derive::napi;

#[napi(object)]
#[derive(Clone)]
pub struct OriginalToken {
  #[napi(js_name = "type")]
  pub _type: String,
  pub value: String,
}

#[derive(Debug, Clone)]
pub struct ResolvedToken {
  pub key: String,
  pub original_value: serde_json::Value,
  pub name: String,
  pub value: serde_json::Value,
  pub path: Vec<String>,
  pub file_path: String,
}

impl ResolvedToken {
  fn to_transformed_token(&self) -> TransformedToken {
    let formatted_value = match &self.value {
      serde_json::Value::String(s) => s.clone(),
      _ => self.value.to_string(),
    };

    TransformedToken {
      key: self.key.clone(),
      value: formatted_value,
      file_path: self.file_path.clone(),
      is_source: true,
      original: self.original_value.clone(),
      name: self.name.clone(),
      attributes: TokenAttrs::from_path(&self.path),
      path: self.path.clone(),
    }
  }
}

impl Into<TransformedToken> for ResolvedToken {
  fn into(self) -> TransformedToken {
    self.to_transformed_token()
  }
}

impl From<&ResolvedToken> for TransformedToken {
  fn from(resolved_token: &ResolvedToken) -> Self {
    resolved_token.to_transformed_token()
  }
}

#[napi(object)]
#[derive(Debug, Clone, serde::Serialize)]
pub struct TokenAttrs {
  pub category: Option<String>,
  #[napi(js_name = "type")]
  pub _type: Option<String>,
  pub item: Option<String>,
  pub subitem: Option<String>,
  pub state: Option<String>,
}

impl TokenAttrs {
  pub fn from_path(path: &Vec<String>) -> Self {
    Self {
      category: path.get(0).cloned(),
      _type: path.get(1).cloned(),
      item: path.get(2).cloned(),
      subitem: path.get(3).cloned(),
      state: path.get(4).cloned(),
    }
  }
}

#[napi(object)]
#[derive(Debug, Clone, serde::Serialize)]
pub struct TransformedToken {
  pub key: String,
  pub value: String,
  pub file_path: String,
  pub is_source: bool,
  #[napi(ts_type = "OriginalToken & { [k: string]: any }")]
  pub original: serde_json::Value,
  pub name: String,
  pub attributes: TokenAttrs,
  pub path: Vec<String>,
}
