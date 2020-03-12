use std::io::{Error};

mod input;
use crate::core;
use crate::config::Config; 

extern crate clap;
use clap::{ArgMatches};

pub fn init(config: &Config, args: &ArgMatches) -> std::result::Result<(), Error> {
  let mut opts = core::InitOpts {
    name: String::from(""),
    template: String::from(""),
    dir: String::from(""),
    repository: None,
  };
  
  let name = args.value_of("name");
  let template = args.value_of("template");
  let dir = args.value_of("dir");
  
  // Get name
  if name.is_none() {
    opts.name = input::get_value("name", true, None)?.unwrap();
  } else {
    opts.name = name.unwrap().to_string();
  }

  // Get template
  if template.is_none() {
    opts.template = input::get_value("template", true, None)?.unwrap();
  } else {
    opts.template = template.unwrap().to_string();
  }

  // Get dir
  if dir.is_none() {
    opts.dir = input::get_value("dir", true, None)?.unwrap();
  } else {
    opts.dir = dir.unwrap().to_string();
  }

  // Get repository
  opts.repository = input::get_value("repository", false, None)?;

  core::init(config, opts)?;

  Ok(())
}

pub fn list(config: &Config) -> std::result::Result<(), Error> {
  core::list(config)?;

  Ok(())
}