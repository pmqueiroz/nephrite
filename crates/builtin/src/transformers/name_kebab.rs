use bindings::token::TransformedToken;

pub struct NameKebab;

impl super::interfaces::BuiltinTransformer for NameKebab {
  fn transform(&self, token: &TransformedToken) -> TransformedToken {
    let mut transformed = token.value.clone();
    transformed = transformed
      .chars()
      .map(|c| {
        if c.is_uppercase() {
          format!("-{}", c.to_lowercase())
        } else {
          c.to_string()
        }
      })
      .collect::<String>();
    let transformed = if transformed.starts_with('-') {
      transformed.trim_start_matches('-').to_string()
    } else {
      transformed
    };

    TransformedToken {
      value: transformed,
      ..token.clone()
    }
  }

  fn name(&self) -> String {
    "name/kebab".to_string()
  }
}

pub const NAME_KEBAB_TRANSFORMER: NameKebab = NameKebab {};
