use chrono::Local;
use log::LevelFilter;
use log4rs::{
  append::{
    console::{ConsoleAppender, Target},
    file::FileAppender,
  },
  config::{Appender, Config, Root},
  encode::pattern::PatternEncoder,
  filter::threshold::ThresholdFilter,
};

use crate::config;

pub fn init() {
  // Build a stderr logger.
  let stderr = ConsoleAppender::builder()
    .encoder(Box::new(PatternEncoder::new(
      "{d(%Y-%m-%d %H:%M:%S)} {l} - {m}\n",
    )))
    .target(Target::Stderr)
    .build();

  // Logging to log file
  let logfile_path = config::directory().join(get_log_file_name());

  let logfile = FileAppender::builder()
    .encoder(Box::new(PatternEncoder::new(
      "{d(%Y-%m-%d %H:%M:%S)} {l} - {m}\n",
    )))
    .build(logfile_path)
    .unwrap();

  let config = Config::builder()
    .appender(Appender::builder().build("logfile", Box::new(logfile)))
    .appender(
      Appender::builder()
        .filter(Box::new(ThresholdFilter::new(log::LevelFilter::Off)))
        .build("stderr", Box::new(stderr)),
    )
    .build(
      Root::builder()
        .appender("logfile")
        .appender("stderr")
        .build(LevelFilter::Info),
    )
    .unwrap();

  log4rs::init_config(config).unwrap();
}

fn get_log_file_name() -> String {
  let local = Local::now();
  let timestamp = local.format("%Y-%m-%d").to_string();
  return String::from("log/") + &timestamp + ".log";
}
