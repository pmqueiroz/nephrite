use std::collections::HashMap;

use crate::{action, parser, platform, transform};

pub type Transforms = HashMap<String, transform::Transform>;
pub type TransformGroups = HashMap<String, transform::TransformGroup>;
pub type Parsers = Vec<parser::RegisteredParser>;
pub type Actions = HashMap<String, action::Action>;
pub type Platforms = HashMap<String, platform::Platform>;
