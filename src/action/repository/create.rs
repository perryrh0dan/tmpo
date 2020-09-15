use std::fs;
use std::fs::File;
use std::io::{Error, Write};
use std::process::exit;

use crate::config::{Config, RepositoryOptions};
use crate::cli::input;
use crate::git;
use crate::meta;
use crate::out;
use crate::repository::{Repository};
use crate::utils;

use clap::ArgMatches;

pub fn create(config: &mut Config, args: &ArgMatches) {
  let name = args.value_of("repository");
  let description = args.value_of("description");
  let directory = args.value_of("directory");

  let repository_type = match input::select("Repository type", &vec!{String::from("remote"), String::from("local")}) {
    Ok(value) => value,
    Err(error) => {
      log::error!("{}", error);
      eprintln!("{}", error);
      exit(1);
    }
  };

  // Get repository name from user input
  let name = if name.is_none() {
    match input::text("repository name", false) {
      Ok(value) => value,
        Err(error) => {
          log::error!("{}", error);
          eprintln!("{}", error);
          exit(1);
        },
    }
  } else {
    utils::lowercase(name.unwrap())
  };

  // validate name
  let repositories = config.get_repositories();
  if repositories.contains(&name) {
    out::error::repository_exists(&name);
    exit(1);
  }

  // Get repository name from user input
  let description = if description.is_none() {
    match input::text("repository description", false) {
      Ok(value) => value,
        Err(error) => {
          log::error!("{}", error);
          eprintln!("{}", error);
          exit(1);
        },
    }
  } else {
    utils::lowercase(description.unwrap())
  };

  if repository_type == "remote" {
    create_remote(config, &name, &description, directory);
  } else {
    create_local(config, &name, &description);
  }

  match config.save() {
    Ok(()) => (),
    Err(error) => {
      log::error!("{}", error);
      eprintln!("{}", error);
      exit(1);
    }
  }
}

fn create_local(config: &mut Config, name: &str, description: &str) {
  config.template_repositories.push(RepositoryOptions {
    name: name.to_owned(),
    description: description.to_owned(),
    git_options: git::GitOptions::new(),
  });

  Repository::new(config, name);
}

fn create_remote(config: &mut Config, name: &str, description: &str, directory: Option<&str>) {
  // Get workspace directory from user input
  let dir = if directory.is_none() {
    match input::text("Please enter the target diectory", false) {
      Ok(value) => value,
      Err(error) => {
        log::error!("{}", error);
        eprintln!("{}", error);
        exit(1);
      },
    }
  } else {
    directory.unwrap().to_string()
  };

  // Create directory
  let repository_path = std::path::Path::new(&dir).join(name);
  std::fs::create_dir(&repository_path);

  // Create meta
  let meta_path = repository_path.join("meta.json");
  let mut meta_file = match std::fs::File::create(meta_path) {
    Ok(file) => file,
    Err(error) => {
      log::error!("{}", error);
      eprintln!("{}", error);
      exit(1);
    },
  };

  // Create meta data
  let mut meta = meta::Meta::new();
  meta.kind = String::from("repository");
  meta.name = name.to_owned();
  meta.version = Some(String::from("1.0.0"));

  let meta_data = serde_json::to_string_pretty(&meta).unwrap();
  match meta_file.write(meta_data.as_bytes()) {
    Ok(file) => file,
    Err(error) => {
      log::error!("{}", error);
      eprintln!("{}", error);
      exit(1);
    }
  };
}
