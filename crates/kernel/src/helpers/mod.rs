pub fn get_file_path(cwd: &Option<std::path::PathBuf>, destination: String) -> String {
  if let Some(cwd_path) = cwd {
    let full_path = cwd_path.join(destination);
    full_path.to_string_lossy().to_string()
  } else {
    destination
  }
}
