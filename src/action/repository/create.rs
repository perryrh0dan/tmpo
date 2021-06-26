use std::fs;
use std::path::Path;
use std::process::exit;

use crate::action::Action;
use crate::cli::input;
use crate::config::{RepositoryOptions};
use crate::context::Context;
use crate::git;
use crate::out;
use crate::repository::{remote_repository, external_repository};
use crate::utils;

use clap::ArgMatches;

impl Action {
  pub fn repository_create(&self, args: &ArgMatches) {
    let _ctx = Context::new(args);

    let kind = match input::select(
      "Type",
      &vec![String::from("remote"), String::from("external")],
    ) {
      Ok(value) => value,
      Err(error) => {
        log::error!("{}", error);
        eprintln!("{}", error);
        exit(1)
      }
    };

    if kind == "remote" {
      self.repository_create_remote(args);
    } else if kind == "external" {
      self.repository_create_external(args);
    }
  }

  pub fn repository_create_remote(&self, args: &ArgMatches) {
    let name = args.value_of("name");
    let description = args.value_of("description");
    let directory = args.value_of("directory");
    let remote = args.value_of("remote");

    // Get repository name from user input
    let name = if name.is_none() {
      match input::text("Enter the repository name", false) {
        Ok(value) => value,
        Err(error) => {
          log::error!("{}", error);
          eprintln!("{}", error);
          exit(1);
        }
      }
    } else {
      utils::lowercase(name.unwrap())
    };

    // Get repository name from user input
    let description = if description.is_none() {
      match input::text("Enter the repository description", false) {
        Ok(value) => value,
        Err(error) => {
          log::error!("{}", error);
          eprintln!("{}", error);
          exit(1);
        }
      }
    } else {
      utils::lowercase(description.unwrap())
    };

    // Get directory from user input
    let directory = if directory.is_none() {
      match input::text("Enter the target directory", false) {
        Ok(value) => value,
        Err(error) => {
          log::error!("{}", error);
          eprintln!("{}", error);
          exit(1);
        }
      }
    } else {
      directory.unwrap().to_string()
    };

    // Get remote from user input
    let remote = if remote.is_none() {
      match input::text("Enter the remote url", false) {
        Ok(value) => value,
        Err(error) => {
          log::error!("{}", error);
          eprintln!("{}", error);
          exit(1);
        }
      }
    } else {
      remote.unwrap().to_string()
    };

    let mut git_options = git::Options::new();
    git_options.url = Some(remote);

    let options = RepositoryOptions {
      name: name.to_owned(),
      kind: Some(String::from("remote")),
      directory: None,
      description: Some(description),
      git_options: Some(git_options),
    };

    // Create repository
    match remote_repository::create(&Path::new(&directory), &options) {
      Ok(()) => (),
      Err(error) => {
        log::error!("{}", error);
        eprintln!("{}", error);
        exit(1);
      }
    };

    out::success::remote_repository_created(
      &options.name,
      &self.config.repositories_dir.to_string_lossy(),
    );
  }

  pub fn repository_create_external(&self, args: &ArgMatches) {
    let name = args.value_of("name");
    let description = args.value_of("description");
    let directory = args.value_of("directory");

    // Get repository name from user input
    let name = if name.is_none() {
      match input::text("Enter the repository name", false) {
        Ok(value) => value,
        Err(error) => {
          log::error!("{}", error);
          eprintln!("{}", error);
          exit(1);
        }
      }
    } else {
      utils::lowercase(name.unwrap())
    };

    // Get repository name from user input
    let description = if description.is_none() {
      match input::text("Enter the repository description", false) {
        Ok(value) => value,
        Err(error) => {
          log::error!("{}", error);
          eprintln!("{}", error);
          exit(1);
        }
      }
    } else {
      utils::lowercase(description.unwrap())
    };

    let options = RepositoryOptions {
      name: name.to_owned(),
      kind: Some(String::from("external")),
      directory: None,
      description: Some(description),
      git_options: None,
    };

    // Get directory from user input
    let directory = if directory.is_none() {
      match input::text("Enter the target directory", false) {
        Ok(value) => value,
        Err(error) => {
          log::error!("{}", error);
          eprintln!("{}", error);
          exit(1);
        }
      }
    } else {
      directory.unwrap().to_string()
    };

    // Create repository
    match external_repository::create(&Path::new(&directory), &options) {
      Ok(()) => (),
      Err(error) => {
        log::error!("{}", error);
        eprintln!("{}", error);
        exit(1);
      }
    };

    let absolute_directory = match fs::canonicalize(&directory) {
      Ok(path) => path,
      Err(error) => {
        log::error!("{}", error);
        eprintln!("Cant canonicalize directory");
        exit(1);
      }
    };

    out::success::external_repository_created(
      &options.name,
      &absolute_directory.as_path().display().to_string(),
    );
  }
}
