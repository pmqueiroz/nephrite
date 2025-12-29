use std::path::PathBuf;

use bindings::platform;

#[derive(Clone)]
pub struct Config<'config> {
  pub source: Vec<String>,
  pub cwd: Option<PathBuf>,
  pub platforms: Vec<platform::Platform<'config>>,
}
