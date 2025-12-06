mod glob;
mod read_file;

pub fn read_file(path: &str) -> std::io::Result<String> {
  read_file::read_file(path)
}

pub fn glob(cwd: &std::path::PathBuf, patterns: Vec<String>) -> Vec<String> {
  glob::glob(cwd, patterns)
}
