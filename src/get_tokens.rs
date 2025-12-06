use kernel::read_file;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

pub fn get_tokens_path(cwd: &std::path::PathBuf, source: Vec<String>) -> Vec<String> {
  kernel::glob(cwd, source)
}

pub fn get_tokens(paths: Vec<String>) -> Vec<String> {
  paths
    .par_iter()
    .map(|path| read_file(path).unwrap_or_default())
    .collect()
}
