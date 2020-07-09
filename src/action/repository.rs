use std::io::ErrorKind;

use crate::cli::{input, select};
use crate::config::{Config, RepositoryOptions};
use crate::git;
use crate::renderer;
use crate::repository::Repository;

use clap::ArgMatches;

pub fn add(config: &mut Config, args: &ArgMatches) {
  let repository_name = args.value_of("repository");

  //// Get repository name from user input
  let repository_name = if repository_name.is_none() {
    match input("repository name", false) {
      Some(value) => value,
      None => return,
    }
  } else {
    String::from(repository_name.unwrap())
  };

  // validate name
  let repositories = Repository::get_repositories(config);
  if repositories.contains(&repository_name) {
    // TODO error
    return;
  }

  //// Get repository name from user input
  let repository_description = match input("repository description", false) {
    Some(value) => value,
    None => return,
  };

  config.templates_repositories.push(RepositoryOptions {
    name: repository_name,
    description: repository_description,
    git_options: git::GitOptions {
      enabled: false,
      url: None,
      auth: None,
      token: None,
      username: None,
      password: None,
    },
  });

  match config.save() {
    Ok(()) => (),
    Err(_error) => return,
  }
}

pub fn remove(config: &mut Config, args: &ArgMatches) {
  let repository_name = args.value_of("repository");

  //// Get repository name from user input
  let repository_name = if repository_name.is_none() {
    let repositories = Repository::get_repositories(config);
    match select("repository", &repositories) {
      Ok(value) => value,
      Err(error) => match error.kind() {
        ErrorKind::InvalidData => {
          renderer::errors::no_repositories();
          return;
        },
        _ => return,
      },
    }
  } else {
    String::from(repository_name.unwrap())
  };

  // remove template folder
  match Repository::delete_repository(config, &repository_name) {
    Ok(()) => (),
    Err(error) => match error.kind() {
      ErrorKind::NotFound => (),
      _ => return,
    },
  };

  // Update config
  let index = config
    .templates_repositories
    .iter()
    .position(|x| x.name == repository_name)
    .unwrap();
  config.templates_repositories.remove(index);

  match config.save() {
    Ok(()) => (),
    Err(_error) => return,
  }
}
