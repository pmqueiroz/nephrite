use super::types::TransformersCollection;
use bindings::{transform::RegisteredTransform, TransformGroup, Transforms};
use log::Logger;

pub fn resolve_transformers(
  transform_group: TransformGroup,
  transforms: Transforms,
) -> TransformersCollection {
  let mut collection = TransformersCollection::new();

  for group_transform in transform_group.transforms {
    if let Some(transform) = transforms.get(&group_transform) {
      collection.push(RegisteredTransform {
        name: transform.name.clone(),
        kind: transform.kind.clone(),
        filter: match transform.filter.create_ref() {
          Ok(filter) => filter,
          Err(e) => {
            Logger::error(&format!(
              "Failed to create filter reference for transform '{}': {}",
              group_transform, e
            ));
            std::process::exit(1);
          }
        },
        transform: match transform.transform.create_ref() {
          Ok(transform) => transform,
          Err(e) => {
            Logger::error(&format!(
              "Failed to create transform reference for transform '{}': {}",
              group_transform, e
            ));
            std::process::exit(1);
          }
        },
      });
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
