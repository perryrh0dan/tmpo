use std::fs;
use std::path::Path;

use crate::cli;
use crate::config::Config;
use crate::git;
use crate::renderer;
use crate::repository;

extern crate clap;
use clap::ArgMatches;

pub struct Options {
  pub name: String,
  pub template: String,
  pub directory: String,
  pub repository: Option<String>,
  pub replace: bool,
}

pub fn init(config: &Config, args: &ArgMatches, verbose: bool) {
  let mut opts = Options {
    name: String::from(""),
    template: String::from(""),
    directory: String::from(""),
    repository: None,
    replace: false,
  };

  let name = args.value_of("name");
  let template = args.value_of("template");
  let directory = args.value_of("directory");

  // Get name
  if name.is_none() {
    opts.name = match cli::get_value("project name", true, None) {
      Ok(name) => name.unwrap(),
      Err(_error) => return,
    };
  } else {
    opts.name = name.unwrap().to_string();
  }

  // Get template
  if template.is_none() {
    opts.template = match cli::get_value("template to use", true, None) {
      Ok(template) => template.unwrap(),
      Err(_error) => return,
    };
  } else {
    opts.template = template.unwrap().to_string();
  }

  // Get directory
  if directory.is_none() {
    opts.directory = match cli::get_value("target directory", true, None) {
      Ok(directory) => directory.unwrap(),
      Err(_error) => return,
    };
  } else {
    opts.directory = directory.unwrap().to_string();
  }

  // Get repository
  opts.repository = match cli::get_value("repository url", false, None) {
    Ok(repository) => repository,
    Err(_error) => return,
  };

  renderer::initiate_workspace(&opts.name);

  let repository = match repository::Repository::new(config, verbose) {
    Ok(repository) => repository,
    Err(_error) => return,
  };

  //Create the workspace directory
  let dir = opts.directory + "/" + &opts.name;
  match fs::create_dir(Path::new(&dir)) {
    Ok(()) => (),
    Err(error) => match error.kind() {
      std::io::ErrorKind::AlreadyExists => (),
      _ => {
        renderer::errors::create_directory(&dir);
        return;
      }
    },
  };

  let mut email: Option<String> = None;
  match git::get_email() {
    Ok(value) => email = Some(value),
    Err(_error) => (),
  };

  let mut username: Option<String> = None;
  match git::get_username() {
    Ok(value) => username = Some(value),
    Err(_error) => (),
  };

  let options = repository::template::Options {
    name: String::from(&opts.name),
    repository: opts.repository.clone(),
    username: username,
    email: email,
    replace: opts.replace,
  };

  // copy the template
  let template = match repository.get_template_by_name(&opts.template) {
    Ok(template) => template,
    Err(_error) => {
      renderer::errors::template_not_found(&opts.template);
      return;
    }
  };

  match template.copy(&repository, &dir, options) {
    Ok(()) => (),
    Err(_error) => {
      renderer::errors::copy_template();
      return;
    }
  };

  // Initialize git if repository is given
  if !opts.repository.is_none() {
    match git::init(&dir, &opts.repository.unwrap()) {
      Ok(()) => (),
      Err(_error) => {
        renderer::errors::init_repository();
        return;
      }
    }
  }

  renderer::success_create(&opts.name);
}
