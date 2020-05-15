mod input;
use crate::config::Config;
use crate::core;

extern crate clap;
use clap::ArgMatches;

pub fn init(config: &Config, args: &ArgMatches, verbose: bool) {
  let mut opts = core::InitOpts {
    name: String::from(""),
    template: String::from(""),
    directory: String::from(""),
    repository: None,
    replace: false,
  };

  let name = args.value_of("name");
  let template = args.value_of("template");
  let directory = args.value_of("directory");
  let replace = args.value_of("replace");

  // Get name
  if name.is_none() {
    opts.name = match input::get_value("project name", true, None) {
      Ok(name) => name.unwrap(),
      Err(_error) => return,
    };
  } else {
    opts.name = name.unwrap().to_string();
  }

  // Get template
  if template.is_none() {
    opts.template = match input::get_value("template to use", true, None) {
      Ok(template) => template.unwrap(),
      Err(_error) => return,
    };
  } else {
    opts.template = template.unwrap().to_string();
  }

  // Get directory
  if directory.is_none() {
    opts.directory = match input::get_value("target directory", true, None) {
      Ok(directory) => directory.unwrap(),
      Err(_error) => return,
    };
  } else {
    opts.directory = directory.unwrap().to_string();
  }

  // Get repository
  opts.repository = match input::get_value("repository url", false, None) {
    Ok(repository) => repository,
    Err(_error) => return,
  };

  match core::init(config, verbose, opts) {
    Ok(()) => (),
    Err(_error) => return,
  };
}

pub fn list(config: &Config, verbose: bool) {
  match core::list(config, verbose) {
    Ok(()) => (),
    Err(_error) => return,
  };
}

pub fn update(config: &Config, verbose: bool) {
  match core::update(config, verbose) {
    Ok(()) => (),
    Err(_error) => println!("{}", _error),
  };
}

pub fn view(config: &Config, args: &ArgMatches, verbose: bool) {
  let template_opt = args.value_of("template");

  // Get template
  let template: String;
  if template_opt.is_none() {
    template = match input::get_value("template to show", true, None) {
      Ok(template) => template.unwrap(),
      Err(_error) => return,
    };
  } else {
    template = template_opt.unwrap().to_string();
  }

  match core::view(config, verbose, &template) {
    Ok(()) => (),
    Err(_error) => return,
  }
}
