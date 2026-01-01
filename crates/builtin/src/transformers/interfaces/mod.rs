use bindings::token::TransformedToken;

pub trait BuiltinTransformer {
  fn transform(&self, token: &TransformedToken) -> TransformedToken;

  fn name(&self) -> String;
}
