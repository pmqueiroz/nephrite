mod interfaces;
mod name_kebab;

pub use interfaces::BuiltinTransformer;
pub use name_kebab::NAME_KEBAB_TRANSFORMER;

const TRANSFORMERS: &[&dyn BuiltinTransformer] = &[&NAME_KEBAB_TRANSFORMER];

pub fn get_transformers() -> &'static [&'static dyn BuiltinTransformer] {
  TRANSFORMERS
}
