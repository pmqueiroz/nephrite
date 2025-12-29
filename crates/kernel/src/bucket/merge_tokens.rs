use serde_json::{Map, Value};

pub fn merge_tokens(values: Vec<Value>) -> Value {
  let mut merged = Map::new();

  for value in values {
    if let Value::Object(obj) = value {
      merged.extend(obj);
    }
  }

  Value::Object(merged)
}
