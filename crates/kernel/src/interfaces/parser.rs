#[derive(Clone, Debug)]
pub struct ParsedFile {
  pub path: String,
  pub content: serde_json::Value,
}
