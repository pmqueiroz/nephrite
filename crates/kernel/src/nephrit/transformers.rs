use bindings::token::TransformedToken;

pub enum Kind {
  Value,
  Name,
}

pub trait InternalTransformerCb {
  fn call(&self, token: TransformedToken) -> TransformedToken;
}

pub struct InternalTransformer {
  pub name: String,
  pub kind: Kind,
  pub get_transformer: fn() -> Box<dyn InternalTransformerCb>,
}
