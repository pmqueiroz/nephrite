extern crate bindings;
extern crate log;
extern crate napi;
extern crate rayon;
extern crate utils;
use napi::bindgen_prelude::Env;

use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use utils::read_file;

pub fn get_tokens_files_paths(cwd: &std::path::PathBuf, source: Vec<String>) -> Vec<String> {
  utils::glob(cwd, source)
}

pub fn get_tokens_files(paths: Vec<String>) -> Vec<bindings::parser::TokenFile> {
  paths
    .par_iter()
    .map(|path| {
      let content = read_file(path).unwrap_or_default();
      bindings::parser::TokenFile {
        path: path.clone(),
        content,
      }
    })
    .collect()
}

pub fn find_parser(
  file_path: String,
  parsers: &[bindings::parser::RegisteredParser],
) -> Option<&bindings::parser::RegisteredParser> {
  for parser in parsers {
    let pattern = &parser.pattern;
    if glob::Pattern::new(pattern).unwrap().matches(&file_path) {
      return Some(parser);
    }
  }
  None
}

pub fn parse_files(
  files: Vec<bindings::parser::TokenFile>,
  parsers: &[bindings::parser::RegisteredParser],
  env: &Env,
) -> Vec<serde_json::Value> {
  files
    .iter()
    .map(|file| {
      let parser_option = find_parser(file.path.clone(), parsers);
      if let Some(parser) = parser_option {
        if let Ok(parser_func) = parser.parser.borrow_back(env) {
          let json_result = parser_func.call(file.clone());
          if let Ok(json_string) = json_result {
            if let Ok(json_value) = serde_json::from_str(&json_string) {
              return json_value;
            }
          }
        }

        serde_json::Value::Null
      } else {
        serde_json::Value::Null
      }
    })
    .collect()
}

mod bucket;
mod build;

pub use bucket::TokensBucket;
pub use build::{build, resolve_transformers};
