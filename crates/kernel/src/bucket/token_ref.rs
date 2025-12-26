use bindings::token::ResolvedToken;

pub fn is_value_ref(value: &serde_json::Value) -> bool {
  match value {
    serde_json::Value::String(s) => s.starts_with('{') && s.ends_with('}'),
    _ => false,
  }
}

pub fn resolve_value_ref(
  value: &serde_json::Value,
  resolved_tokens: &std::collections::HashMap<String, ResolvedToken>,
) -> Option<serde_json::Value> {
  if let serde_json::Value::String(s) = value {
    if is_value_ref(value) {
      let key = s.trim_start_matches('{').trim_end_matches('}').to_string();
      if let Some(token) = resolved_tokens.get(&key) {
        return Some(token.value.clone());
      }
    }
  }
  None
}
