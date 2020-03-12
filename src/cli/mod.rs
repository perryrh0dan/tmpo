mod input;
use crate::core;
use crate::config::Config; 

extern crate clap;
use clap::{ArgMatches};

pub fn init(config: &Config, args: &ArgMatches) {
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
    opts.name = match input::get_value("name", true, None) {
      Ok(name) => name.unwrap(),
      Err(_error) => return,
    };
  } else {
    opts.name = name.unwrap().to_string();
  }

  // Get template
  if template.is_none() {
    opts.template = match input::get_value("template", true, None) {
      Ok(template) => template.unwrap(),
      Err(_error) => return,
    };
  } else {
    opts.template = template.unwrap().to_string();
  }

  // Get dir
  if dir.is_none() {
    opts.dir = match input::get_value("dir", true, None) {
      Ok(dir) => dir.unwrap(),
      Err(_error) => return,
    };
  } else {
    opts.dir = dir.unwrap().to_string();
  }

  // Get repository
  opts.repository = match input::get_value("repository", false, None) {
    Ok(repository) => repository,
    Err(_error) => return,
  };

  match core::init(config, opts){
    Ok(()) => (),
    Err(_error) => return,
  };
}

pub fn list(config: &Config) {
  match core::list(config) {
    Ok(()) => (),
    Err(_error) => return,
  };
}