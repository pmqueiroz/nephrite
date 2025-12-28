use bindings::{Dictionary, FormatArguments, RegisteredFormat};
use log::Logger;
use napi::Env;
use std::fs;
use std::path::Path;

pub fn format_file(
  env: &Env,
  destination: String,
  format: &RegisteredFormat,
  dictionary: &Dictionary,
) {
  Logger::debug(&format!(
    "Processing platform file: destination='{}', format='{}', using dictionary with {} entries",
    destination,
    format.name,
    dictionary.all_tokens.len()
  ));

  if let Ok(format_func) = format.format.borrow_back(env) {
    let file_result = format_func.call(FormatArguments {
      dictionary: dictionary.clone(),
    });
    if let Ok(file_content) = file_result {
      match write_file_to_destination(&destination, &file_content) {
        Ok(_) => {
          Logger::info(&format!(
            "Successfully wrote formatted content to '{}'",
            destination
          ));
        }
        Err(e) => {
          Logger::error(&format!("Failed to write file '{}': {}", destination, e));
          std::process::exit(1);
        }
      }
    } else {
      Logger::error(&format!(
        "Format function '{}' failed to generate content for '{}'",
        format.name, destination
      ));
      std::process::exit(1);
    }
  } else {
    Logger::error(&format!(
      "Failed to borrow format function '{}' for '{}'",
      format.name, destination
    ));
    std::process::exit(1);
  }
}

fn write_file_to_destination(destination: &str, content: &str) -> Result<(), std::io::Error> {
  if let Some(parent) = Path::new(destination).parent() {
    fs::create_dir_all(parent)?;
  }

  fs::write(destination, content)?;

  Ok(())
}
