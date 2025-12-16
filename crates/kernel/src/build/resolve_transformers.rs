use super::types::TransformersCollection;
use bindings::{TransformGroup, Transforms};
use log::Logger;

pub fn resolve_transformers(
  transform_group: TransformGroup,
  transforms: Transforms,
) -> TransformersCollection {
  let mut collection = TransformersCollection::new();

  for group_transform in transform_group.transforms {
    if let Some(transform) = transforms.get(&group_transform) {
      collection.push(transform.clone());
    } else {
      Logger::error(&format!(
        "Transform '{}' not found in the registered transforms.",
        group_transform
      ));
      std::process::exit(1);
    }
  }

  collection
}
