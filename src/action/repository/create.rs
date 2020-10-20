use std::path::Path;
use std::process::exit;

use crate::action::Action;
use crate::cli::input;
use crate::config::{RepositoryOptions};
use crate::git;
use crate::out;
use crate::repository::custom_repository;
use crate::utils;

use clap::ArgMatches;

impl Action {
  pub fn repository_create(&self, args: &ArgMatches) {
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

    // validate name
    let repositories = self.config.get_repositories();
    if repositories.contains(&name) {
      out::error::repository_exists(&name);
      exit(1);
    }

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

    let mut options = RepositoryOptions {
      name: name.to_owned(),
      description: Some(description),
      git_options: git::Options::new(),
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

    options.git_options.url = Some(remote);

    // Create repository
    match custom_repository::create(&Path::new(&directory), &options) {
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
}
