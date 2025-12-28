extern crate globwalk;
use std::path::Path;

pub fn glob(cwd: &Path, patterns: Vec<String>) -> Vec<String> {
  let mut paths: Vec<String> = Vec::new();

  let glob_paths: Vec<globwalk::DirEntry> =
    globwalk::GlobWalkerBuilder::from_patterns(cwd, &patterns)
      .build()
      .unwrap()
      .filter_map(Result::ok)
      .collect();

  for path in glob_paths {
    if let Some(pathname) = path.path().to_str() {
      paths.push(pathname.to_string());
    }
  }
  paths
}
