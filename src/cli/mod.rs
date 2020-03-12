use std::io;
use std::io::{Error};
use std::io::*;

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
    opts.name = get_value("name", true, None)?.unwrap();
  } else {
    opts.name = name.unwrap().to_string();
  }

  // Get template
  if template.is_none() {
    opts.template = get_value("template", true, None)?.unwrap();
  } else {
    opts.template = template.unwrap().to_string();
  }

  // Get dir
  if dir.is_none() {
    opts.dir = get_value("dir", true, None)?.unwrap();
  } else {
    opts.dir = dir.unwrap().to_string();
  }

  // Get repository
  opts.repository = get_value("repository", false, None)?;

  core::init(config, opts)?;

  Ok(())
}

fn get_value(name: &str, required: bool, default: Option<&str>) -> std::result::Result<Option<String>, Error> {
  if required {
    print!("Enter {}: ", name);
  } else {
    print!{"Enter {}?: ", name};
  }
  // directly print message
  io::stdout().flush()?;

  let mut value = String::new();
  while(true) {
    io::stdin().read_line(&mut value).expect("error: unable to read user input");
    if value == "" && !required {
      return Ok(None);
    } else if value != "" {
      break;
    }
  }

  Ok(Some(value))
}