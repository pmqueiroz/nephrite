use std::fs;

pub fn read_file(path: &str) -> std::io::Result<String> {
  fs::read_to_string(path)
}
