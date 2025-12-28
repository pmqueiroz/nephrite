use std::collections::HashMap;

use crate::{action, format, parser, platform, transform};

pub type Transforms = HashMap<String, transform::Transform>;
pub type RegisteredTransforms = HashMap<String, transform::RegisteredTransform>;
pub type TransformGroups = HashMap<String, transform::TransformGroup>;
pub type Parsers = Vec<parser::RegisteredParser>;
pub type Actions = HashMap<String, action::Action>;
pub type Platforms<'platform> = HashMap<String, platform::Platform<'platform>>;
pub type Formats = HashMap<String, format::Format>;
pub type RegisteredFormats = HashMap<String, format::RegisteredFormat>;
