extern crate chromalog;

use chromalog::{
  debug, error, info, trace, warn, ArgColor, ChromaLog, Color, ColorConfig, LevelFilter,
};

pub struct Logger;

impl Logger {
  pub fn init() {
    let custom_colors = ColorConfig {
      error_color: Color::Red,
      warn_color: Color::BrightYellow,
      info_color: Color::BrightGreen,
      debug_color: Color::BrightBlue,
      trace_color: Color::BrightMagenta,
      arg_color: ArgColor::None,
      target_color: Color::Cyan,
      datetime_color: None,
    };

    let result = ChromaLog::init(LevelFilter::Trace, custom_colors, None);

    match result {
      Ok(_) => (),
      Err(e) => eprintln!("Failed to initialize logger: {}", e),
    }
  }

  pub fn error(message: &str) {
    error!("{}", message);
  }

  pub fn warn(message: &str) {
    warn!("{}", message);
  }

  pub fn info(message: &str) {
    info!("{}", message);
  }

  pub fn debug(message: &str) {
    debug!("{}", message);
  }

  pub fn trace(message: &str) {
    trace!("{}", message);
  }
}
