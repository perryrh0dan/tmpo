use std::fs::{OpenOptions};
use std::io::prelude::*;

use crate::config;

pub fn migrate() {
  configuration();
}

fn configuration() {
  let mut file = OpenOptions::new()
    .append(true)
    .open(config::config_location())
    .unwrap();

  let config = config::get_default_config();

  if let Err(e) = writeln!(file, "") {
    eprintln!("Couldn't write to file: {}", e);
  }

  if let Err(e) = writeln!(file, "version: 2.0.0") {
    eprintln!("Couldn't write to file: {}", e);
  }

  if let Err(e) = writeln!(file, "repositories_dir: {}", config.repositories_dir.to_str().unwrap()) {
    eprintln!("Couldn't write to file: {}", e);
  }

  if let Err(e) = writeln!(file, "templates_dir: {}", config.templates_dir.to_str().unwrap()) {
    eprintln!("Couldn't write to file: {}", e);
  }
}
