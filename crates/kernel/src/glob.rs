extern crate globwalk;
use std::path::PathBuf;

pub fn glob(cwd: &PathBuf, patterns: Vec<String>) -> Vec<String> {
  let mut paths: Vec<String> = Vec::new();

  let glob_paths: Vec<globwalk::DirEntry> =
    globwalk::GlobWalkerBuilder::from_patterns(cwd.clone(), &patterns)
      .build()
      .unwrap()
      .into_iter()
      .filter_map(Result::ok)
      .collect();

  for path in glob_paths {
    if let Some(pathname) = path.path().to_str() {
      paths.push(pathname.to_string());
    }
  }
  paths
}
